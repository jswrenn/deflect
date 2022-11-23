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
mod err;

pub mod schema;
pub mod r#value;

pub use r#schema::Type;
pub use r#value::Value;

type Byte = MaybeUninit<u8>;
type Bytes<'value> = &'value [Byte];

type Addr2LineReader = EndianReader<RunTimeEndian, Rc<[u8]>>;
type Context = addr2line::Context<Addr2LineReader>;

/// A source of debug info that can be trusted to correspond to the current executable.
pub unsafe trait DebugInfo {
    type Reader: gimli::Reader<Offset = usize>;

    fn context(&self) -> &addr2line::Context<Self::Reader>;
}

pub struct CurrentExeContext {
    context: Rc<addr2line::Context<Addr2LineReader>>,
}

pub fn current_exe_debuginfo() -> CurrentExeContext {
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

impl<T> Reflect for T {}

impl dyn Reflect {
    /// Produces a reflected `Value` of `&self`.
    pub fn reflect<'value, 'dwarf, D: DebugInfo>(
        &'value self,
        debug_info: &'dwarf D,
    ) -> Result<Value<'value, 'dwarf, D::Reader>, crate::err::Error> {
        let context = debug_info.context();
        let (unit, offset) = self.dw_unit_and_die_of(context)?;
        let entry = unit.entry(offset)?;
        let r#type = Type::from_die(context.dwarf(), unit, entry)?;
        let value =
            slice_from_raw_parts(self as *const Self as *const Byte, mem::size_of_val(self));
        unsafe { value::Value::with_type(r#type, &*value) }
    }

    /// Produces the DWARF unit and entry offset for the DIE of `T`.
    fn dw_unit_and_die_of<'ctx, R>(
        &self,
        ctx: &'ctx addr2line::Context<R>,
    ) -> Result<(&'ctx crate::gimli::Unit<R>, crate::gimli::UnitOffset), crate::err::Error>
    where
        R: crate::gimli::Reader<Offset = usize>,
    {
        let Some(symbol_addr) = self.symbol_addr() else {
            panic!("Could not find symbol address for `symbol_addr::<T>`.")
        };

        let Some(dw_die_offset) = ctx
            .find_frames(symbol_addr as u64)?
            .next()?
            .and_then(|f| f.dw_die_offset) else {
                panic!("Could not find debug info for `symbol_addr::<T>`.");
            };

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

        let Some(ty) = ty else {
            panic!( "Could not find parameter type entry");
        };

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

fn current_binary() -> Option<std::fs::File> {
    let file = std::fs::File::open(std::env::current_exe().unwrap()).ok()?;
    Some(file)
}

fn check_tag<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
    expected: crate::gimli::DwTag,
) -> Result<(), crate::err::Kind> {
    let actual = entry.tag();
    if actual != expected {
        Err(crate::err::Kind::tag_mismatch(expected, actual))
    } else {
        Ok(())
    }
}

fn get_name<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
    dwarf: &crate::gimli::Dwarf<R>,
    unit: &crate::gimli::Unit<R, usize>,
) -> Result<R, crate::err::Kind> {
    let name = get(entry, crate::gimli::DW_AT_name)?;
    let name = dwarf.attr_string(unit, name)?;
    Ok(name)
}

fn get<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
    attr: crate::gimli::DwAt,
) -> Result<AttributeValue<R>, crate::err::Kind> {
    entry
        .attr_value(attr)?
        .ok_or(crate::err::Kind::missing_attr(attr))
}

pub(crate) fn get_size<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<u64, crate::err::Kind> {
    let size = get(entry, crate::gimli::DW_AT_byte_size)?;
    size.udata_value().ok_or(crate::err::Kind::invalid_attr(
        crate::gimli::DW_AT_byte_size,
    ))
}

pub(crate) fn get_size_opt<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<Option<u64>, crate::err::Kind> {
    let maybe_size = entry.attr_value(crate::gimli::DW_AT_byte_size)?;
    if let Some(size) = maybe_size {
        Ok(Some(size.udata_value().ok_or(
            crate::err::Kind::invalid_attr(crate::gimli::DW_AT_byte_size),
        )?))
    } else {
        Ok(None)
    }
}

pub(crate) fn get_align<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<Option<u64>, crate::err::Kind> {
    let maybe_size = entry.attr_value(crate::gimli::DW_AT_alignment)?;
    if let Some(size) = maybe_size {
        size.udata_value()
            .ok_or(crate::err::Kind::invalid_attr(
                crate::gimli::DW_AT_alignment,
            ))
            .map(Some)
    } else {
        Ok(None)
    }
}

pub(crate) fn get_type_ref<'entry, 'dwarf, R: crate::gimli::Reader<Offset = usize>>(
    entry: &'entry crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
) -> Result<UnitOffset, crate::err::Kind> {
    if let AttributeValue::UnitRef(offset) = get(entry, crate::gimli::DW_AT_type)? {
        Ok(offset)
    } else {
        Err(crate::err::Kind::invalid_attr(crate::gimli::DW_AT_type))
    }
}

pub(crate) fn get_type_res<'entry, 'dwarf, R: crate::gimli::Reader<Offset = usize>>(
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: &'entry crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
) -> Result<crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>, crate::err::Kind> {
    if let AttributeValue::UnitRef(offset) = get(entry, crate::gimli::DW_AT_type)? {
        Ok(unit.entry(offset)?)
    } else {
        Err(crate::err::Kind::invalid_attr(crate::gimli::DW_AT_type))
    }
}

pub(crate) fn get_type_opt<'entry, 'dwarf, R: crate::gimli::Reader<Offset = usize>>(
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: &'entry crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
) -> Result<Option<crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>>, crate::err::Kind> {
    let maybe_type = entry.attr_value(crate::gimli::DW_AT_type)?;
    Ok(if let Some(_type) = maybe_type {
        if let AttributeValue::UnitRef(offset) = get(entry, crate::gimli::DW_AT_type)? {
            Some(unit.entry(offset)?)
        } else {
            return Err(crate::err::Kind::invalid_attr(crate::gimli::DW_AT_type));
        }
    } else {
        None
    })
}

pub(crate) fn get_type<'dwarf, R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
) -> Result<UnitOffset, crate::err::Kind> {
    let attr = crate::gimli::DW_AT_type;
    let value = get(entry, attr)?;
    let x = entry.attr(attr)?.unwrap();

    if let AttributeValue::UnitRef(offset) = value {
        Ok(offset)
    } else {
        Err(err::Kind::invalid_attr(attr))
    }
}

fn get_file<'a, R: crate::gimli::Reader<Offset = usize> + 'a>(
    dwarf: &'a crate::gimli::Dwarf<R>,
    unit: &'a crate::gimli::Unit<R, usize>,
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<Option<Cow<'a, str>>, crate::err::Error> {
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
) -> Result<Option<UnitOffset>, crate::err::Kind> {
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

fn fmt_err<E: fmt::Display>(err: E) -> fmt::Error {
    eprintln!("ERROR: {}", err);
    fmt::Error
}
