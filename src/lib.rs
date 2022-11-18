#![feature(once_cell)]
#![feature(
    provide_any,
    error_generic_member_access,
    result_flattening,
    pointer_byte_offsets
)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]

use addr2line::gimli;

use gimli::{AttributeValue, EndianReader, RunTimeEndian, UnitOffset};

use anyhow::anyhow;
use std::{
    backtrace::Backtrace,
    borrow::Cow,
    ffi::c_void,
    fmt,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ops,
    ptr::slice_from_raw_parts,
    rc::Rc,
    sync::LazyLock,
};

mod debug;
mod schema;
mod r#value;

pub use r#schema::Type;
pub use r#value::Value;

type Byte = MaybeUninit<u8>;
type Bytes<'value> = &'value [Byte];

type Addr2LineReader = EndianReader<RunTimeEndian, Rc<[u8]>>;
pub type Context = addr2line::Context<Addr2LineReader>;

/// A source of debug info that can be trusted to correspond to the current executable.
pub unsafe trait DebugInfo {
    type Reader: gimli::Reader<Offset = usize>;

    fn context(&self) -> &addr2line::Context<Self::Reader>;
}

pub struct CurrentExeContext {
    context: Rc<addr2line::Context<Addr2LineReader>>,
}

pub fn current_exe_debuginfo() -> CurrentExeContext {
    impl ops::Deref for CurrentExeContext {
        type Target = addr2line::Context<Addr2LineReader>;

        fn deref(&self) -> &Self::Target {
            self.context.as_ref()
        }
    }

    unsafe impl DebugInfo for CurrentExeContext {
        type Reader = Addr2LineReader;

        fn context(&self) -> &addr2line::Context<Self::Reader> {
            &self.context
        }
    }

    thread_local! {
        pub static CONTEXT: Rc<Context> = {
            static MMAP: LazyLock<memmap2::Mmap> = LazyLock::new(|| {
                let file = current_binary().unwrap();

                unsafe { memmap2::Mmap::map(&file).unwrap() }
            });

            static OBJECT: LazyLock<object::File<'static, &'static [u8]>> = LazyLock::new(|| {
                object::File::parse(MMAP.as_ref()).unwrap()
            });
            Rc::new(addr2line::Context::new(&*OBJECT).unwrap())
        };
    }

    CurrentExeContext {
        context: CONTEXT.with(|ctx| ctx.clone()),
    }
}

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

impl<T> Reflect for T {}

impl dyn Reflect {
    /// Produces a reflected `Value` of `&self`.
    pub fn reflect<'value, 'dbginfo, D: DebugInfo>(
        &'value self,
        debug_info: &'dbginfo D,
    ) -> Result<Value<'value, 'dbginfo, D::Reader>, Error> {
        let context = debug_info.context();
        let (unit, offset) = self.dw_unit_and_die_of(context)?;
        let entry = unit.entry(offset)?;
        let r#type = Type::from_die(context.dwarf(), unit, entry)?;
        let value =
            slice_from_raw_parts(self as *const Self as *const Byte, mem::size_of_val(self));
        Ok(unsafe { value::Value::with_type(r#type, &*value) })
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
            return Err(ErrorKind::MakeAMoreSpecificVariant("Could not find symbol address for `symbol_addr::<T>`.").into())
        };

        let dw_die_offset = ctx
            .find_frames(symbol_addr as u64)?
            .next()?
            .and_then(|f| f.dw_die_offset)
            .ok_or(ErrorKind::MakeAMoreSpecificVariant(
                "Could not find debug info for `symbol_addr::<T>`.",
            ))?;

        let unit = ctx.find_dwarf_unit(symbol_addr as u64).unwrap();

        let mut ty = None;
        let mut tree = unit.entries_tree(Some(dw_die_offset))?;
        let mut children = tree.root()?.children();

        while let Some(child) = children.next()? {
            if ty.is_none() && child.entry().tag() == crate::gimli::DW_TAG_template_type_parameter {
                ty = Some(get_type(child.entry())?);
                break;
            }
        }

        let ty = ty.ok_or(ErrorKind::MakeAMoreSpecificVariant(
            "Could not find parameter type entry",
        ))?;

        Ok((unit, ty))
    }
}

impl fmt::Debug for dyn Reflect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let context = current_exe_debuginfo();
        let value = self.reflect(&context).unwrap();
        fmt::Display::fmt(&value, f)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("{}\n{}", self.error, self.backtrace)]
pub struct Error {
    error: ErrorKind,
    backtrace: Backtrace,
}

impl From<ErrorKind> for Error {
    fn from(error: ErrorKind) -> Self {
        Self {
            error,
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<crate::gimli::Error> for Error {
    fn from(error: crate::gimli::Error) -> Self {
        Self {
            error: ErrorKind::Gimli(error),
            backtrace: Backtrace::capture(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Could not downcast {:?}, received {:?}", .value, std::any::type_name::<T>())]
pub struct DowncastErr<V, T>
where
    V: fmt::Debug,
{
    value: V,
    r#type: PhantomData<T>,
}

impl<V, T> DowncastErr<V, T>
where
    V: fmt::Debug,
{
    pub fn new(value: V) -> Self {
        Self {
            value,
            r#type: PhantomData,
        }
    }

    pub fn into<V2, T2>(self) -> DowncastErr<V2, T2>
    where
        V2: fmt::Debug + From<V>,
    {
        DowncastErr::new(self.value.into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("tag mismatch; expected {:?}, received {:?}", .expected.static_string(), .actual.static_string())]
    TagMismatch {
        expected: crate::gimli::DwTag,
        actual: crate::gimli::DwTag,
    },
    #[error("tag had value of unexpected type")]
    ValueMismatch,
    #[error("die did not have the tag {:?}", .attr.static_string())]
    MissingAttr { attr: crate::gimli::DwAt },
    #[error("die did not have the child {tag}")]
    MissingChild { tag: crate::gimli::DwTag },
    #[error("{0}")]
    Gimli(crate::gimli::Error),
    #[error("{0}")]
    MakeAMoreSpecificVariant(&'static str),
}

impl From<crate::gimli::Error> for ErrorKind {
    fn from(value: crate::gimli::Error) -> Self {
        Self::Gimli(value)
    }
}

/// Produces the DWARF unit and entry offset for the DIE of `T`.
fn dw_unit_and_die_of<'ctx, T: ?Sized, R>(
    ctx: &'ctx addr2line::Context<R>,
) -> Result<(&'ctx crate::gimli::Unit<R>, crate::gimli::UnitOffset), crate::Error>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Produces the symbol address of itself.
    #[inline(never)]
    fn symbol_addr<T: ?Sized>() -> Option<*mut c_void> {
        let ip = (symbol_addr::<T> as usize + 1) as *mut c_void;
        let mut symbol_addr = None;
        backtrace::resolve(ip, |symbol| {
            symbol_addr = symbol.addr();
        });
        symbol_addr
    }

    let Some(symbol_addr) = symbol_addr::<T>() else {
        return Err(ErrorKind::MakeAMoreSpecificVariant("Could not find symbol address for `symbol_addr::<T>`.").into())
    };

    let dw_die_offset = ctx
        .find_frames(symbol_addr as u64)?
        .next()?
        .and_then(|f| f.dw_die_offset)
        .ok_or(ErrorKind::MakeAMoreSpecificVariant(
            "Could not find debug info for `symbol_addr::<T>`.",
        ))?;

    let unit = ctx.find_dwarf_unit(symbol_addr as u64).unwrap();

    let mut ty = None;
    let mut tree = unit.entries_tree(Some(dw_die_offset))?;
    let mut children = tree.root()?.children();

    while let Some(child) = children.next()? {
        if ty.is_none() && child.entry().tag() == crate::gimli::DW_TAG_template_type_parameter {
            ty = Some(get_type(child.entry())?);
            break;
        }
    }

    let ty = ty.ok_or(ErrorKind::MakeAMoreSpecificVariant(
        "Could not find parameter type entry",
    ))?;

    Ok((unit, ty))
}

pub fn reflect<'ctx, 'value, T: ?Sized, R>(
    ctx: &'ctx addr2line::Context<R>,
    value: &'value T,
) -> Result<value::Value<'value, 'ctx, R>, Error>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    let r#type = reflect_type::<T, _>(ctx)?;
    let value = slice_from_raw_parts(value as *const T as *const Byte, mem::size_of_val(value));
    let value = unsafe { &*value };
    Ok(unsafe { value::Value::with_type(r#type, value) })
}

pub fn reflect_type<'ctx, T: ?Sized, R>(
    ctx: &'ctx addr2line::Context<R>,
) -> Result<Type<'ctx, R>, Error>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    let (unit, offset) = dw_unit_and_die_of::<T, _>(ctx)?;

    let _tree = unit.entries_tree(Some(offset))?;
    //debug::inspect_tree(&mut tree, ctx.dwarf(), unit);

    let die = unit.entry(offset).unwrap();
    Type::from_die(ctx.dwarf(), unit, die)
}

fn current_binary() -> Option<std::fs::File> {
    let file = std::fs::File::open(std::env::current_exe().unwrap()).ok()?;
    Some(file)
}

fn check_tag<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
    expected: crate::gimli::DwTag,
) -> Result<(), ErrorKind> {
    let actual = entry.tag();
    if actual != expected {
        Err(ErrorKind::TagMismatch { expected, actual })
    } else {
        Ok(())
    }
}

fn get_name<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
    dwarf: &crate::gimli::Dwarf<R>,
    unit: &crate::gimli::Unit<R, usize>,
) -> Result<R, ErrorKind> {
    let name = get(entry, crate::gimli::DW_AT_name)?;
    let name = dwarf.attr_string(unit, name)?;
    Ok(name)
}

fn get<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
    attr: crate::gimli::DwAt,
) -> Result<AttributeValue<R>, ErrorKind> {
    entry
        .attr_value(attr)?
        .ok_or(ErrorKind::MissingAttr { attr })
}

fn get_data_member_location<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<AttributeValue<R>, ErrorKind> {
    get(entry, crate::gimli::DW_AT_data_member_location)
}

pub(crate) fn get_size<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<Option<u64>, ErrorKind> {
    let maybe_size = entry.attr_value(crate::gimli::DW_AT_byte_size)?;
    if let Some(size) = maybe_size {
        size.udata_value().ok_or(ErrorKind::ValueMismatch).map(Some)
    } else {
        Ok(None)
    }
}

pub(crate) fn get_align<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<Option<u64>, ErrorKind> {
    let maybe_size = entry.attr_value(crate::gimli::DW_AT_byte_size)?;
    if let Some(size) = maybe_size {
        size.udata_value().ok_or(ErrorKind::ValueMismatch).map(Some)
    } else {
        Ok(None)
    }
}

pub(crate) fn get_type_opt<'dwarf, R: crate::gimli::Reader<Offset = usize>>(
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: &'dwarf crate::gimli::DebuggingInformationEntry<R>,
) -> Result<Option<crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>>, ErrorKind> {
    let maybe_type = entry.attr_value(crate::gimli::DW_AT_type)?;
    Ok(if let Some(_type) = maybe_type {
        if let AttributeValue::UnitRef(offset) = get(entry, crate::gimli::DW_AT_type)? {
            Some(unit.entry(offset)?)
        } else {
            return Err(ErrorKind::ValueMismatch);
        }
    } else {
        None
    })
}

pub(crate) fn get_type<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<UnitOffset, ErrorKind> {
    if let AttributeValue::UnitRef(offset) = get(entry, crate::gimli::DW_AT_type)? {
        Ok(offset)
    } else {
        Err(ErrorKind::ValueMismatch)
    }
}

fn get_file<'a, R: crate::gimli::Reader<Offset = usize> + 'a>(
    dwarf: &'a crate::gimli::Dwarf<R>,
    unit: &'a crate::gimli::Unit<R, usize>,
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<Option<Cow<'a, str>>, Error> {
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
) -> Result<Option<UnitOffset>, anyhow::Error> {
    if let Some(attr) = entry.attr(name)? {
        if let AttributeValue::UnitRef(offset) = attr.value() {
            return Ok(Some(offset));
        }
    }
    Ok(None)
}

fn fi_to_string<'a, R: crate::gimli::Reader<Offset = usize> + 'a>(
    file_index: u64,
    unit: &'a crate::gimli::Unit<R>,
) -> Result<String, anyhow::Error> {
    let line_program = unit
        .line_program
        .as_ref()
        .ok_or(anyhow!("no lineprogram"))?;
    let file = line_program
        .header()
        .file(file_index)
        .ok_or(anyhow!("no such file"))?;
    let AttributeValue::String(ref bytes) = file.path_name() else {
        return Err(anyhow!("path name was not a string"));
    };
    let path = bytes.to_string_lossy().unwrap().into_owned();
    Ok(path)
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
