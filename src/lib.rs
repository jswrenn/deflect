//! DWARF-based reflection.
//!
//! Use the [`Reflect`] trait to debug or recursively destructure any value.
//!
//!

use addr2line::{gimli, object};
use dashmap::DashMap;
use gimli::{AttributeValue, EndianReader, RunTimeEndian, UnitOffset};
use once_cell::sync::Lazy;
use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
    collections::HashMap,
    ffi::c_void,
    fmt,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    path::Path,
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

pub struct DebugInfo<'d, R>
where
    R: gimli::Reader<Offset = usize>,
{
    context: &'d addr2line::Context<R>,
    unit: &'d gimli::Unit<R>,
    entry: gimli::UnitOffset,
}

/// A source of debug info that can be trusted to correspond to the current executable.
pub unsafe trait DebugInfoProvider: Clone {
    type Reader: gimli::Reader<Offset = usize>;

    fn info_for(&self, fn_addr: u64) -> Result<DebugInfo<'_, Self::Reader>, crate::Error>;
}

mod dbginfo_provider {
    use super::*;

    struct Map {
        path: std::path::PathBuf,
        static_addr: usize,
    }

    fn map_of(dynamic_addr: usize) -> Result<Map, ()> {
        let pid = std::process::id();
        let mappings = procmaps::Mappings::from_pid(pid as _).unwrap();

        for map in mappings.iter() {
            if (map.base..=map.ceiling).contains(&dynamic_addr) {
                if let procmaps::Path::MappedFile(file) = &map.pathname {
                    return Ok(Map {
                        static_addr: (dynamic_addr - map.base) + map.offset,
                        path: file.into(),
                    });
                }
            }
        }
        Err(())
    }

    pub fn context_of(dynamic_addr: usize) -> Result<(&'static Context, usize), crate::Error> {
        let Map { path, static_addr } =
            map_of(dynamic_addr).map_err(|_| crate::error::Kind::Other)?;
        let context = read_context(path)?;
        Ok((context, static_addr))
    }

    pub fn read_context<P>(path: P) -> Result<&'static Context, crate::Error>
    where
        P: Borrow<Path>,
    {
        static OBJECT_CACHE: Lazy<
            DashMap<std::path::PathBuf, &'static object::File<'static, &[u8]>>,
        > = Lazy::new(|| DashMap::new());

        let path = path.borrow().to_owned();

        let object = OBJECT_CACHE.entry(path.clone()).or_try_insert_with(|| {
            let file = std::fs::File::open(&path)?;
            let mmap = Box::new(unsafe { memmap2::Mmap::map(&file)? });
            let mmap: &'static memmap2::Mmap = Box::leak::<'static>(mmap);
            let mmap: &'static [u8] = mmap;
            let object = object::File::parse(mmap)?;
            let object = Box::leak(Box::new(object));
            Ok::<_, crate::Error>(object)
        })?;

        thread_local! {
            pub static CONTEXT_CACHE: RefCell<HashMap<std::path::PathBuf, &'static Context>> =
                RefCell::new(HashMap::new());
        }

        let context = CONTEXT_CACHE.with(move |context_cache| {
            let mut context_cache = context_cache.borrow_mut();
            if let Some(context) = context_cache.get(&path) {
                Ok(*context)
            } else {
                let context = addr2line::Context::new(*object).unwrap();
                let context: &'static _ = Box::leak(Box::new(context));
                context_cache.insert(path, context);
                Ok(context)
            }
        });
        context
    }
}

/// The default provider of DWARF debug info.
///
/// On Linux, this is simply the current executable.
pub fn default_provider() -> Result<impl DebugInfoProvider, crate::Error> {
    #[derive(Copy, Clone)]
    pub struct DefaultProvider {}

    unsafe impl DebugInfoProvider for DefaultProvider {
        type Reader = Addr2LineReader;

        fn info_for(&self, fn_addr: u64) -> Result<DebugInfo<'static, Self::Reader>, crate::Error> {
            let (context, static_addr) = crate::dbginfo_provider::context_of(fn_addr as _)?;
            let (unit, entry) = crate::dw_unit_and_die_of_addr(context, static_addr)?;
            Ok(DebugInfo {
                context,
                unit,
                entry,
            })
        }
    }

    Ok(DefaultProvider {})
}

/// A reflectable type.
pub trait Reflect {
    fn type_id(&self) -> usize;
}

impl<T: ?Sized> Reflect for T {
    fn type_id(&self) -> usize {
        <Self as Reflect>::type_id as usize + 1
    }
}

impl dyn Reflect + '_ {
    /// Produces a reflected `Value` of `&self`.
    pub fn reflect<'value, 'dwarf, P: DebugInfoProvider>(
        &'value self,
        provider: &'dwarf P,
    ) -> Result<Value<'value, 'dwarf, P>, crate::Error> {
        let DebugInfo {
            context,
            unit,
            entry,
        } = provider.info_for(self.type_id() as _)?;
        let entry = unit.entry(entry)?;
        let r#type = schema::Type::from_die(context.dwarf(), unit, entry)?;
        let value =
            slice_from_raw_parts(self as *const Self as *const Byte, mem::size_of_val(self));
        unsafe { value::Value::with_type(r#type, &*value, provider) }
    }
}

/// Produces the DWARF unit and entry offset for the DIE of `T`.
fn dw_unit_and_die_of_addr<'ctx, R>(
    ctx: &'ctx addr2line::Context<R>,
    static_addr: usize,
) -> Result<(&'ctx crate::gimli::Unit<R>, crate::gimli::UnitOffset), crate::Error>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    let Some(dw_die_offset) = ctx
        .find_frames(static_addr as u64)?
        .next()?
        .and_then(|f| f.dw_die_offset) else {
            return Err(error::Kind::missing_debug_info().into())
        };

    let Some(unit) = ctx.find_dwarf_unit(static_addr as u64) else {
        return Err(error::Kind::missing_debug_info().into())
    };

    let e = unit.entry(dw_die_offset)?;

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
pub enum Value<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// A reflected [`prim@bool`] value.
    bool(value::bool<'value, 'dwarf, P>),

    /// A reflected [`prim@char`] value.
    char(value::char<'value, 'dwarf, P>),

    /// A reflected [`prim@f32`] value.
    f32(value::f32<'value, 'dwarf, P>),

    /// A reflected [`prim@f64`] value.
    f64(value::f64<'value, 'dwarf, P>),

    /// A reflected [`prim@i8`] value.
    i8(value::i8<'value, 'dwarf, P>),

    /// A reflected [`prim@i16`] value.
    i16(value::i16<'value, 'dwarf, P>),

    /// A reflected [`prim@i32`] value.
    i32(value::i32<'value, 'dwarf, P>),

    /// A reflected [`prim@i64`] value.
    i64(value::i64<'value, 'dwarf, P>),

    /// A reflected [`prim@i128`] value.
    i128(value::i128<'value, 'dwarf, P>),

    /// A reflected [`prim@isize`] value.
    isize(value::isize<'value, 'dwarf, P>),

    /// A reflected [`prim@u8`] value.
    u8(value::u8<'value, 'dwarf, P>),

    /// A reflected [`prim@u16`] value.
    u16(value::u16<'value, 'dwarf, P>),

    /// A reflected [`prim@u32`] value.
    u32(value::u32<'value, 'dwarf, P>),

    /// A reflected [`prim@u64`] value.
    u64(value::u64<'value, 'dwarf, P>),

    /// A reflected [`prim@u128`] value.
    u128(value::u128<'value, 'dwarf, P>),

    /// A reflected [`prim@usize`] value.
    usize(value::usize<'value, 'dwarf, P>),

    /// A reflected [`()`][prim@unit] value.
    unit(value::unit<'value, 'dwarf, P>),

    /// A reflected [`str`][prim@str] value.
    str(value::str<'value, 'dwarf, P>),

    Array(value::Array<'value, 'dwarf, P>),

    /// A reflected [`Box`] value.
    Box(value::Box<'value, 'dwarf, P>),

    /// A reflected [`Box`]'d slice value.
    BoxedSlice(value::BoxedSlice<'value, 'dwarf, P>),

    /// A reflected [`Box`]'d dyn value.
    BoxedDyn(value::BoxedDyn<'value, 'dwarf, P>),

    Slice(value::Slice<'value, 'dwarf, P>),
    Struct(value::Struct<'value, 'dwarf, P>),
    Enum(value::Enum<'value, 'dwarf, P>),
    Function(value::Function<'value, 'dwarf, P>),
    SharedRef(value::Pointer<'value, 'dwarf, crate::schema::Shared, P>),
    UniqueRef(value::Pointer<'value, 'dwarf, crate::schema::Unique, P>),
    ConstPtr(value::Pointer<'value, 'dwarf, crate::schema::Const, P>),
    MutPtr(value::Pointer<'value, 'dwarf, crate::schema::Mut, P>),
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
            return Ok(format!("{dirname}/{filename}"));
        }
    }
    Ok(filename.into_owned())
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

pub fn to_static_addr(dynamic_addr: usize) -> Result<usize, crate::Error> {
    use std::{
        fs,
        io::{BufRead, BufReader},
        process,
    };
    let pid = process::id();
    let proc_maps_filename = format!("/proc/{}/maps", pid);

    let file = fs::File::open(proc_maps_filename).unwrap();
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut tokens = line.split(' ');

        let range = tokens.next().unwrap();
        let (section_start_addr, section_end_addr) = {
            let mut tokens = range.split(|c| c == '-' || c == ' ');
            let section_start_addr = usize::from_str_radix(tokens.next().unwrap(), 16).unwrap();
            let section_end_addr = usize::from_str_radix(tokens.next().unwrap(), 16).unwrap();
            (section_start_addr, section_end_addr)
        };
        let _permissions = tokens.next().unwrap();
        let offset_addr = usize::from_str_radix(tokens.next().unwrap(), 16).unwrap();

        if dynamic_addr >= section_start_addr && dynamic_addr < section_end_addr {
            return Ok((dynamic_addr - section_start_addr) + offset_addr);
        }
    }

    Err(error::Kind::Other.into())
}
