//! DWARF-based reflection.
//!
//! Use the [`Reflect`] trait to debug or recursively destructure any value.
//!
//!

use addr2line::{gimli, object};
use gimli::{AttributeValue, EndianReader, RunTimeEndian, UnitOffset};
use once_cell::sync::Lazy;
use std::{
    borrow::Cow,
    ffi::c_void,
    fmt,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr::slice_from_raw_parts,
    rc::Rc,
    sync::Arc,
};

mod debug;
pub mod error;

pub mod schema;
pub mod value;

type Byte = MaybeUninit<u8>;
type Bytes<'value> = &'value [Byte];

type Addr2LineReader = EndianReader<RunTimeEndian, Rc<[u8]>>;
type Context = addr2line::Context<Addr2LineReader>;

use std::backtrace::Backtrace as Stacktrace;

/// An error with a backtrace.
#[derive(thiserror::Error, Debug)]
#[error("{}\n{}", self.kind, self.stacktrace)]
pub struct Error {
    pub kind: error::Kind,
    pub stacktrace: Stacktrace,
}

impl<E> From<E> for Error
where
    error::Kind: From<E>,
{
    fn from(error: E) -> Self {
        Self {
            kind: error.into(),
            stacktrace: std::backtrace::Backtrace::capture(),
        }
    }
}

/// A source of debug info that can be trusted to correspond to the current executable.
pub unsafe trait DebugInfo {
    type Reader: gimli::Reader<Offset = usize>;

    fn context(&self) -> &addr2line::Context<Self::Reader>;
}

/// Asserts that the given context can be trusted to correspond to the current executable.
pub struct AssertTrusted<C, R>
where
    C: AsRef<addr2line::Context<R>>,
    R: gimli::Reader<Offset = usize>,
{
    context: C,
    reader: PhantomData<R>,
}

impl<C, R> AssertTrusted<C, R>
where
    C: AsRef<addr2line::Context<R>>,
    R: gimli::Reader<Offset = usize>,
{
    /// Assert that the given context can be trusted to correspond to the current executable.
    pub unsafe fn new(context: C) -> Self {
        Self {
            context,
            reader: PhantomData,
        }
    }
}

unsafe impl<C, R> DebugInfo for AssertTrusted<C, R>
where
    C: AsRef<addr2line::Context<R>>,
    R: gimli::Reader<Offset = usize>,
{
    type Reader = R;

    fn context(&self) -> &addr2line::Context<Self::Reader> {
        self.context.as_ref()
    }
}

/// The default provider of DWARF debug info.
///
/// On Linux, this is simply the current executable.
pub fn default_provider() -> Result<impl DebugInfo, impl std::error::Error> {
    struct DefaultProvider {
        context: Rc<addr2line::Context<Addr2LineReader>>,
    }

    unsafe impl DebugInfo for DefaultProvider {
        type Reader = Addr2LineReader;

        fn context(&self) -> &addr2line::Context<Self::Reader> {
            &self.context
        }
    }

    thread_local! {
        pub static CONTEXT: Result<Rc<Context>, Arc<crate::Error>> = {
            // mmap this process's executable
            static CURRENT_EXE: Lazy<Result<memmap2::Mmap,  Arc<crate::Error>>> = Lazy::new(|| {
                let path = std::env::current_exe().map_err(Error::from)?;
                let file =  std::fs::File::open(path).map_err(Error::from)?;
                Ok(unsafe { memmap2::Mmap::map(&file).map_err(Error::from)? })
            });

            // parse it as an object
            static OBJECT: Lazy<Result<object::File<'static, &[u8]>, Arc<crate::Error>>> = Lazy::new(|| {
                let data = CURRENT_EXE.as_ref().map_err(Arc::clone)?.as_ref();
                Ok(object::File::parse(data).map_err(Error::from)?)
            });

            let object = OBJECT.as_ref().map_err(Arc::clone)?;
            let context = addr2line::Context::new(object).map_err(Error::from)?;

            Ok(Rc::new(context))
        };
    }

    CONTEXT.with(|context| match context {
        Ok(context) => Ok(unsafe { AssertTrusted::new(context.clone()) }),
        Err(err) => Err(err.clone()),
    })
}

/// A reflectable type.
pub trait Reflect {
    /// Produces the symbol address of itself.
    #[inline(never)]
    fn symbol_addr(&self) -> Option<*mut c_void> {
        let ip = (<Self as Reflect>::symbol_addr as usize + 1) as *mut c_void;
        let mut symbol_addr = None;
        backtrace::resolve(ip, |symbol| {
            symbol_addr = symbol.addr();
        });
        symbol_addr
    }
}

impl<T: ?Sized> Reflect for T {}

impl dyn Reflect + '_ {
    /// Produces a reflected `Value` of `&self`.
    pub fn reflect<'value, 'dwarf, D: DebugInfo>(
        &'value self,
        debug_info: &'dwarf D,
    ) -> Result<Value<'value, 'dwarf, D::Reader>, crate::Error> {
        let context = debug_info.context();
        let (unit, offset) = self.dw_unit_and_die_of(context)?;
        let entry = unit.entry(offset)?;
        let r#type = schema::Type::from_die(context.dwarf(), unit, entry)?;
        let value =
            slice_from_raw_parts(self as *const Self as *const Byte, mem::size_of_val(self));
        unsafe { value::Value::with_type(r#type, &*value) }
    }

    /// Produces the DWARF unit and entry offset for the DIE of `T`.
    fn dw_unit_and_die_of<'ctx, R>(
        &self,
        ctx: &'ctx addr2line::Context<R>,
    ) -> Result<(&'ctx crate::gimli::Unit<R>, crate::gimli::UnitOffset), crate::Error>
    where
        R: crate::gimli::Reader<Offset = usize>,
    {
        let Some(symbol_addr) = self.symbol_addr() else {
            return Err(error::Kind::missing_symbol_address().into())
        };

        let Some(dw_die_offset) = ctx
            .find_frames(symbol_addr as u64)?
            .next()?
            .and_then(|f| f.dw_die_offset) else {
                return Err(error::Kind::missing_debug_info().into())
            };

        let Some(unit) = ctx.find_dwarf_unit(symbol_addr as u64) else {
            return Err(error::Kind::missing_debug_info().into())
        };

        let mut ty = None;
        let mut tree = unit.entries_tree(Some(dw_die_offset))?;
        let mut children = tree.root()?.children();

        while let Some(child) = children.next()? {
            if ty.is_none() && child.entry().tag() == crate::gimli::DW_TAG_template_type_parameter {
                ty = Some(get_type(child.entry())?);
                break;
            }
        }

        let Some(ty) = ty else {
            return Err(error::Kind::Other.into())
        };

        Ok((unit, ty))
    }
}

impl fmt::Debug for dyn Reflect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let context = default_provider().map_err(crate::fmt_err)?;
        let value = self.reflect(&context).map_err(crate::fmt_err)?;
        fmt::Display::fmt(&value, f)
    }
}

/// A reflected value.
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    /// A reflected [`prim@bool`] value.
    bool(value::bool<'value, 'dwarf, R>),

    /// A reflected [`prim@char`] value.
    char(value::char<'value, 'dwarf, R>),

    /// A reflected [`prim@f32`] value.
    f32(value::f32<'value, 'dwarf, R>),

    /// A reflected [`prim@f64`] value.
    f64(value::f64<'value, 'dwarf, R>),

    /// A reflected [`prim@i8`] value.
    i8(value::i8<'value, 'dwarf, R>),

    /// A reflected [`prim@i16`] value.
    i16(value::i16<'value, 'dwarf, R>),

    /// A reflected [`prim@i32`] value.
    i32(value::i32<'value, 'dwarf, R>),

    /// A reflected [`prim@i64`] value.
    i64(value::i64<'value, 'dwarf, R>),

    /// A reflected [`prim@i128`] value.
    i128(value::i128<'value, 'dwarf, R>),

    /// A reflected [`prim@isize`] value.
    isize(value::isize<'value, 'dwarf, R>),

    /// A reflected [`prim@u8`] value.
    u8(value::u8<'value, 'dwarf, R>),

    /// A reflected [`prim@u16`] value.
    u16(value::u16<'value, 'dwarf, R>),

    /// A reflected [`prim@u32`] value.
    u32(value::u32<'value, 'dwarf, R>),

    /// A reflected [`prim@u64`] value.
    u64(value::u64<'value, 'dwarf, R>),

    /// A reflected [`prim@u128`] value.
    u128(value::u128<'value, 'dwarf, R>),

    /// A reflected [`prim@usize`] value.
    usize(value::usize<'value, 'dwarf, R>),

    /// A reflected [`()`][prim@unit] value.
    unit(value::unit<'value, 'dwarf, R>),

    /// A reflected [`str`][prim@str] value.
    str(value::str<'value, 'dwarf, R>),

    Array(value::Array<'value, 'dwarf, R>),
    Slice(value::Slice<'value, 'dwarf, R>),
    Struct(value::Struct<'value, 'dwarf, R>),
    Enum(value::Enum<'value, 'dwarf, R>),
    Function(value::Function<'value, 'dwarf, R>),
    SharedRef(value::Pointer<'value, 'dwarf, crate::schema::Shared, R>),
    UniqueRef(value::Pointer<'value, 'dwarf, crate::schema::Unique, R>),
    ConstPtr(value::Pointer<'value, 'dwarf, crate::schema::Const, R>),
    MutPtr(value::Pointer<'value, 'dwarf, crate::schema::Mut, R>),
}

fn check_tag<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
    expected: crate::gimli::DwTag,
) -> Result<(), crate::error::Kind> {
    let actual = entry.tag();
    if actual != expected {
        Err(crate::error::Kind::tag_mismatch(expected, actual))
    } else {
        Ok(())
    }
}

fn get<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
    attr: crate::gimli::DwAt,
) -> Result<AttributeValue<R>, crate::error::Kind> {
    entry
        .attr_value(attr)?
        .ok_or(crate::error::Kind::missing_attr(attr))
}

fn get_size<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<u64, crate::error::Kind> {
    let size = get(entry, crate::gimli::DW_AT_byte_size)?;
    size.udata_value().ok_or(crate::error::Kind::invalid_attr(
        crate::gimli::DW_AT_byte_size,
    ))
}

fn get_size_opt<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<Option<u64>, crate::error::Kind> {
    let maybe_size = entry.attr_value(crate::gimli::DW_AT_byte_size)?;
    if let Some(size) = maybe_size {
        Ok(Some(size.udata_value().ok_or(
            crate::error::Kind::invalid_attr(crate::gimli::DW_AT_byte_size),
        )?))
    } else {
        Ok(None)
    }
}

fn get_align<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<Option<u64>, crate::error::Kind> {
    let maybe_size = entry.attr_value(crate::gimli::DW_AT_alignment)?;
    if let Some(size) = maybe_size {
        size.udata_value()
            .ok_or(crate::error::Kind::invalid_attr(
                crate::gimli::DW_AT_alignment,
            ))
            .map(Some)
    } else {
        Ok(None)
    }
}

fn get_type_ref<'entry, 'dwarf, R: crate::gimli::Reader<Offset = usize>>(
    entry: &'entry crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
) -> Result<UnitOffset, crate::error::Kind> {
    if let AttributeValue::UnitRef(offset) = get(entry, crate::gimli::DW_AT_type)? {
        Ok(offset)
    } else {
        Err(crate::error::Kind::invalid_attr(crate::gimli::DW_AT_type))
    }
}

fn get_type_res<'entry, 'dwarf, R: crate::gimli::Reader<Offset = usize>>(
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: &'entry crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
) -> Result<crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>, crate::error::Kind> {
    if let AttributeValue::UnitRef(offset) = get(entry, crate::gimli::DW_AT_type)? {
        Ok(unit.entry(offset)?)
    } else {
        Err(crate::error::Kind::invalid_attr(crate::gimli::DW_AT_type))
    }
}

fn get_type<'dwarf, R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
) -> Result<UnitOffset, crate::error::Kind> {
    let attr = crate::gimli::DW_AT_type;
    let value = get(entry, attr)?;
    if let AttributeValue::UnitRef(offset) = value {
        Ok(offset)
    } else {
        Err(error::Kind::invalid_attr(attr))
    }
}

fn get_file<'a, R: crate::gimli::Reader<Offset = usize> + 'a>(
    dwarf: &'a crate::gimli::Dwarf<R>,
    unit: &'a crate::gimli::Unit<R, usize>,
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<Option<Cow<'a, str>>, crate::Error> {
    let file = get(entry, crate::gimli::DW_AT_decl_file)?;
    let AttributeValue::FileIndex(index) = file else {
        return Ok(None); // error?
    };
    let Some(prog) = &unit.line_program else { return Ok(None) };
    let Some(file) = prog.header().file(index) else { return Ok(None) };
    let filename = dwarf.attr_string(unit, file.path_name())?;
    let filename = filename.to_string_lossy()?;
    if let Some(dir) = file.directory(prog.header()) {
        let dirname = dwarf.attr_string(unit, dir)?;
        let dirname = dirname.to_string_lossy()?;
        if !dirname.is_empty() {
            return Ok(Some(format!("{dirname}/{filename}").into()));
        }
    }
    // TODO: Any other way around this lifetime error?
    Ok(Some(filename.into_owned().into()))
}

fn get_attr_ref<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
    name: crate::gimli::DwAt,
) -> Result<Option<UnitOffset>, crate::error::Kind> {
    if let Some(attr) = entry.attr(name)? {
        if let AttributeValue::UnitRef(offset) = attr.value() {
            return Ok(Some(offset));
        }
    }
    Ok(None)
}

fn fi_to_string<'a, R: crate::gimli::Reader<Offset = usize> + 'a>(
    dwarf: &'a crate::gimli::Dwarf<R>,
    unit: &'a crate::gimli::Unit<R>,
    file_index: u64,
) -> Result<String, crate::Error> {
    let line_program = unit
        .line_program
        .as_ref()
        .ok_or(error::Kind::file_indexing())?;

    let file = line_program
        .header()
        .file(file_index)
        .ok_or(error::Kind::file_indexing())?;

    let filename = dwarf.attr_string(unit, file.path_name())?;
    let filename = filename.to_string_lossy()?;
    if let Some(dir) = file.directory(line_program.header()) {
        let dirname = dwarf.attr_string(unit, dir)?;
        let dirname = dirname.to_string_lossy()?;
        if !dirname.is_empty() {
            return Ok(format!("{dirname}/{filename}").into());
        }
    }
    Ok(filename.into_owned().into())
}

struct DebugDisplay<T>(T);

impl<T> fmt::Debug for DebugDisplay<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

fn fmt_err<E: fmt::Display>(err: E) -> fmt::Error {
    eprintln!("ERROR: {}", err);
    fmt::Error
}
