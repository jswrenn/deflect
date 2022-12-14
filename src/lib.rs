//! Deflect brings reflection to Rust using [DWARF] debug info.
//!
//! Deflect can be used to recover the concrete types of trait objects, inspect
//! the internal state of `async` generators, and pretty-print arbitrary data.
//!
//! [DWARF]: https://en.wikipedia.org/wiki/DWARF
//!
//! ## Example
//! Use the [`Reflect`] trait to debug or recursively destructure any value.
//!
//! ```
//! # use std::any::Any;
//! pub struct Foo {
//!     a: u8
//! }
//!
//! // initialize the debuginfo provider
//! let context = deflect::default_provider()?;
//!
//! // create some type-erased data
//! let data: Box<dyn Any> = Box::new(Foo { a: 42 });
//!
//! // cast it to `&dyn Reflect`
//! let erased: &dyn deflect::Reflect = &data;
//!
//! // reflect it!
//! let value: deflect::Value = erased.reflect(&context)?;
//!
//! // pretty-print the reflected value
//! assert_eq!(value.to_string(), "box Foo { a: 42 }");
//!
//! // downcast into a `BoxedDyn` value
//! let value: deflect::value::BoxedDyn = value.try_into()?;
//!
//! // dereference the boxed value
//! let value: deflect::Value = value.deref()?;
//! // downcast into a `Struct` value
//! let value: deflect::value::Struct = value.try_into()?;
//!
//! // pretty-print the reflected value
//! assert_eq!(value.to_string(), "Foo");
//! # Ok::<_, Box<dyn std::error::Error>>(())
//! ```

#![allow(clippy::len_without_is_empty, clippy::needless_lifetimes)]
#![deny(missing_docs)]

#[macro_use]
pub extern crate anyhow;
use anyhow::Error;

pub use addr2line::{self, gimli, object};

use dashmap::DashMap;
use gimli::{AttributeValue, EndianReader, RunTimeEndian, UnitOffset};
use once_cell::sync::Lazy;
use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
    collections::HashMap,
    fmt,
    mem::{self, MaybeUninit},
    path::Path,
    ptr::slice_from_raw_parts,
    rc::Rc,
};

mod debug;
mod error;
pub use error::DowncastErr;

pub mod schema;
pub mod value;

type Byte = MaybeUninit<u8>;
type Bytes<'value> = &'value [Byte];

type Addr2LineReader = EndianReader<RunTimeEndian, Rc<[u8]>>;
type Context = addr2line::Context<Addr2LineReader>;

/// Raw debug info for a function.
pub struct DebugInfo<'d, R>
where
    R: gimli::Reader<Offset = usize>,
{
    context: &'d addr2line::Context<R>,
    unit: &'d gimli::Unit<R>,
    entry: gimli::UnitOffset,
}

impl<'d, R> DebugInfo<'d, R>
where
    R: gimli::Reader<Offset = usize>,
{
    /// Constructs a new `DebugInfo`.
    pub fn new(
        context: &'d addr2line::Context<R>,
        unit: &'d gimli::Unit<R>,
        entry: gimli::UnitOffset,
    ) -> Self {
        Self {
            context,
            unit,
            entry,
        }
    }
}

/// A source of debug info that can be trusted to correspond to the current
/// executable.
///
/// ## Safety
/// Implementers of this trait must provide accurate debug info for this
/// program.
pub unsafe trait DebugInfoProvider: Clone {
    /// The type of the DWARF reader.
    type Reader: gimli::Reader<Offset = usize>;

    /// Produces debug info for a given function.
    fn info_for(&self, fn_addr: u64) -> Result<DebugInfo<'_, Self::Reader>, crate::Error>;
}

mod dbginfo_provider {
    use super::*;

    struct Map {
        path: std::path::PathBuf,
        static_addr: usize,
    }

    fn map_of(dynamic_addr: usize) -> Result<Map, crate::Error> {
        let pid = std::process::id();
        let mappings = procmaps::Mappings::from_pid(pid as _)?;

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
        bail!("Could not map the dynamic address 0x{dynamic_addr:x} to a static address in the binary.");
    }

    pub fn context_of(dynamic_addr: usize) -> Result<(&'static Context, usize), crate::Error> {
        let Map { path, static_addr } = map_of(dynamic_addr)?;
        let context = read_context(path)?;
        Ok((context, static_addr))
    }

    pub fn read_context<P>(path: P) -> Result<&'static Context, crate::Error>
    where
        P: Borrow<Path>,
    {
        static OBJECT_CACHE: Lazy<
            DashMap<std::path::PathBuf, &'static object::File<'static, &[u8]>>,
        > = Lazy::new(DashMap::new);

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

        CONTEXT_CACHE.with(move |context_cache| {
            let mut context_cache = context_cache.borrow_mut();
            if let Some(context) = context_cache.get(&path) {
                Ok(*context)
            } else {
                let context = addr2line::Context::new(*object)?;
                let context: &'static _ = Box::leak(Box::new(context));
                context_cache.insert(path, context);
                Ok(context)
            }
        })
    }
}

pub(crate) mod private {
    #[derive(Copy, Clone)]
    pub struct DefaultProvider {}
}

pub(crate) use private::DefaultProvider;

/// The default provider of DWARF debug info.
pub fn default_provider() -> Result<DefaultProvider, crate::Error> {
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
    /// Produces an ID that uniquely identifies the type within its compilation
    /// unit.
    fn local_type_id(&self) -> usize;
}

impl<T: ?Sized> Reflect for T {
    #[inline(never)]
    fn local_type_id(&self) -> usize {
        <Self as Reflect>::local_type_id as usize
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
        } = provider.info_for(self.local_type_id() as _)?;
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
    let Some(frame) = ctx
        .find_frames(static_addr as u64)?
        .next()? else {
            return Err(error::missing_debug_info())
        };

    let Some(dw_die_offset) = frame.dw_die_offset else {
            return Err(error::missing_debug_info())
        };

    let Some(unit) = ctx.find_dwarf_unit(static_addr as u64) else {
        return Err(error::missing_debug_info())
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
        return Err(error::missing_child(crate::gimli::DW_TAG_template_type_parameter))
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

macro_rules! generate_type_and_value {
    ($($(#[$attr:meta])* $t:ident,)*) => {
        /// A reflected type.
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        #[non_exhaustive]
        pub enum Type<'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = usize>,
        {
            $(
                $(#[$attr])*
                $t(schema::$t::<'dwarf, R>),
            )*
        }

        impl<'dwarf, R> fmt::Display for Type<'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = usize>,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$t(v) => v.fmt(f),)*
                }
            }
        }

        $(
            #[doc = concat!(
                "Upcast a [`",
                stringify!($t),
                "<'dwarf, R>`][crate::schema::",
                stringify!($t),
                "] into a [`Type<'dwarf, R>`][Type].",
            )]
            impl<'dwarf, R> From<crate::schema::$t<'dwarf, R>>
            for Type<'dwarf, R>
            where
                R: crate::gimli::Reader<Offset = std::primitive::usize>,
            {
                fn from(atom: crate::schema::$t<'dwarf, R>) -> Self {
                    Type::$t(atom)
                }
            }

            #[doc = concat!(
                "Attempt to downcast a [`Type<'dwarf, R>`][Type] into a [`",
                stringify!($t),
                "<'dwarf, R>`][crate::schema::",
                stringify!($t),
                "].",
            )]
            impl<'value, 'dwarf, R> TryFrom<Type<'dwarf, R>> for crate::schema::$t<'dwarf, R>
            where
                R: crate::gimli::Reader<Offset = std::primitive::usize>,
            {
                type Error = crate::DowncastErr;

                fn try_from(value: Type<'dwarf, R>) -> Result<Self, Self::Error> {
                    if let Type::$t(value) = value {
                        Ok(value)
                    } else {
                        Err(crate::DowncastErr::new::<Type<'dwarf, R>, Self>())
                    }
                }
            }

            #[doc = concat!(
                "Attempt to downcast a [`&Type<'dwarf, R>`][Type] into a [`&",
                stringify!($t),
                "<'dwarf, R>`][crate::schema::",
                stringify!($t),
                "].",
            )]
            impl<'a, 'value, 'dwarf, R> TryFrom<&'a Type<'dwarf, R>>
                for &'a crate::schema::$t<'dwarf, R>
            where
                R: crate::gimli::Reader<Offset = std::primitive::usize>,
            {
                type Error = crate::DowncastErr;

                fn try_from(value: &'a Type<'dwarf, R>) -> Result<Self, Self::Error> {
                    if let Type::$t(value) = value {
                        Ok(value)
                    } else {
                        Err(crate::DowncastErr::new::<
                            &'a Type<'dwarf, R>,
                            Self,
                        >())
                    }
                }
            }
        )*

        /// A reflected value.
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        #[non_exhaustive]
        pub enum Value<'value, 'dwarf, P = crate::DefaultProvider>
        where
            P: crate::DebugInfoProvider,
        {
            $(
                $(#[$attr])*
                $t(value::$t::<'value, 'dwarf, P>),
            )*
        }

        impl<'value, 'dwarf, P> Value<'value, 'dwarf, P>
        where
            P: crate::DebugInfoProvider,
        {
            /// Safety: `value` absolutely must have the correct `type`.
            pub(crate) unsafe fn with_type(
                r#type: crate::schema::Type<'dwarf, P::Reader>,
                value: crate::Bytes<'value>,
                provider: &'dwarf P,
            ) -> Result<Self, crate::Error> {
                match r#type {
                    $(crate::schema::Type::$t(schema) => schema.with_bytes(provider, value).map(Self::$t),)*
                }
            }
        }

        impl<'value, 'dwarf, P> fmt::Display for Value<'value, 'dwarf, P>
        where
            P: crate::DebugInfoProvider,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$t(v) => v.fmt(f),)*
                }
            }
        }

        $(
            #[doc = concat!(
                "Upcast a [`",
                stringify!($t),
                "<'value, 'dwarf, P>`][value::",
                stringify!($t),
                "] into a [`Value<'value, 'dwarf, P>`][Value].",
            )]
            impl<'value, 'dwarf, P> From<value::$t<'value, 'dwarf, P>>
            for Value<'value, 'dwarf, P>
            where
                P: crate::DebugInfoProvider,
            {
                fn from(atom: value::$t<'value, 'dwarf, P>) -> Self {
                    Value::$t(atom)
                }
            }

            #[doc = concat!(
                "Attempt to downcast a [`Value<'value, 'dwarf, P>`][Value] into a [`",
                stringify!($t),
                "<'value, 'dwarf, P>`][value::",
                stringify!($t),
                "].",
            )]
            impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>> for value::$t<'value, 'dwarf, P>
            where
                P: crate::DebugInfoProvider,
            {
                type Error = crate::DowncastErr;

                fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
                    if let Value::$t(value) = value {
                        Ok(value)
                    } else {
                        Err(crate::DowncastErr::new::<Value<'value, 'dwarf, P>, Self>())
                    }
                }
            }

            #[doc = concat!(
                "Attempt to downcast a [`&Value<'value, 'dwarf, P>`][Value] into a [`&",
                stringify!($t),
                "<'value, 'dwarf, P>`][value::",
                stringify!($t),
                "].",
            )]
            impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>>
                for &'a value::$t<'value, 'dwarf, P>
            where
                P: crate::DebugInfoProvider,
            {
                type Error = crate::DowncastErr;

                fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
                    if let Value::$t(value) = value {
                        Ok(value)
                    } else {
                        Err(crate::DowncastErr::new::<
                            &'a Value<'value, 'dwarf, P>,
                            Self,
                        >())
                    }
                }
            }
        )*
    }
}

generate_type_and_value! {
    /// A reflected [`prim@bool`].
    bool,

    /// A reflected [`prim@char`].
    char,

    /// A reflected [`prim@f32`].
    f32,

    /// A reflected [`prim@f64`].
    f64,

    /// A reflected [`prim@i8`].
    i8,

    /// A reflected [`prim@i16`].
    i16,

    /// A reflected [`prim@i32`].
    i32,

    /// A reflected [`prim@i64`].
    i64,

    /// A reflected [`prim@i128`].
    i128,

    /// A reflected [`prim@isize`].
    isize,

    /// A reflected [`prim@u8`].
    u8,

    /// A reflected [`prim@u16`].
    u16,

    /// A reflected [`prim@u32`].
    u32,

    /// A reflected [`prim@u64`].
    u64,

    /// A reflected [`prim@u128`].
    u128,

    /// A reflected [`prim@usize`].
    usize,

    /// A reflected [`()`][prim@unit].
    unit,

    /// A reflected [`str`][prim@str].
    str,

    /// A reflected [`array`][prim@array].
    Array,

    /// A reflected [`Box`].
    Box,

    /// A reflected [`Box`]'d slice.
    BoxedSlice,

    /// A reflected [`Box`]'d dyn.
    BoxedDyn,

    /// A reflected slice.
    Slice,

    /// A reflected struct.
    Struct,

    /// A reflected enum.
    Enum,

    /// A reflected function.
    Function,

    /// A reflected shared reference.
    SharedRef,

    /// A reflected unique reference.
    UniqueRef,

    /// A reflected `const` pointer.
    ConstPtr,

    /// A reflected `mut` pointer.
    MutPtr,
}

#[track_caller]
fn check_tag<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
    expected: crate::gimli::DwTag,
) -> Result<(), crate::Error> {
    let actual = entry.tag();
    if actual != expected {
        Err(crate::error::tag_mismatch(expected, actual))
    } else {
        Ok(())
    }
}

fn get<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
    attr: crate::gimli::DwAt,
) -> Result<AttributeValue<R>, crate::Error> {
    entry
        .attr_value(attr)?
        .ok_or_else(|| crate::error::missing_attr(attr))
}

fn get_opt<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
    attr: crate::gimli::DwAt,
) -> Result<Option<AttributeValue<R>>, crate::Error> {
    Ok(entry.attr_value(attr)?)
}

fn get_size<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<u64, crate::Error> {
    let size = get(entry, crate::gimli::DW_AT_byte_size)?;
    size.udata_value()
        .ok_or_else(|| crate::error::invalid_attr(crate::gimli::DW_AT_byte_size))
}

fn get_size_opt<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<Option<u64>, crate::Error> {
    let maybe_size = entry.attr_value(crate::gimli::DW_AT_byte_size)?;
    if let Some(size) = maybe_size {
        Ok(Some(size.udata_value().ok_or_else(|| {
            crate::error::invalid_attr(crate::gimli::DW_AT_byte_size)
        })?))
    } else {
        Ok(None)
    }
}

fn get_align<R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<R>,
) -> Result<Option<u64>, crate::Error> {
    let maybe_size = entry.attr_value(crate::gimli::DW_AT_alignment)?;
    if let Some(size) = maybe_size {
        size.udata_value()
            .ok_or_else(|| crate::error::invalid_attr(crate::gimli::DW_AT_alignment))
            .map(Some)
    } else {
        Ok(None)
    }
}

fn get_type_ref<'entry, 'dwarf, R: crate::gimli::Reader<Offset = usize>>(
    entry: &'entry crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
) -> Result<UnitOffset, crate::Error> {
    if let AttributeValue::UnitRef(offset) = get(entry, crate::gimli::DW_AT_type)? {
        Ok(offset)
    } else {
        Err(crate::error::invalid_attr(crate::gimli::DW_AT_type))
    }
}

fn get_type_res<'entry, 'dwarf, R: crate::gimli::Reader<Offset = usize>>(
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: &'entry crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
) -> Result<crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>, crate::Error> {
    if let AttributeValue::UnitRef(offset) = get(entry, crate::gimli::DW_AT_type)? {
        Ok(unit.entry(offset)?)
    } else {
        Err(crate::error::invalid_attr(crate::gimli::DW_AT_type))
    }
}

fn get_type<'dwarf, R: crate::gimli::Reader<Offset = usize>>(
    entry: &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
) -> Result<UnitOffset, crate::Error> {
    let attr = crate::gimli::DW_AT_type;
    let value = get(entry, attr)?;
    if let AttributeValue::UnitRef(offset) = value {
        Ok(offset)
    } else {
        Err(error::invalid_attr(attr))
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
) -> Result<Option<UnitOffset>, crate::Error> {
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
        .ok_or_else(error::file_indexing)?;

    let file = line_program
        .header()
        .file(file_index)
        .ok_or_else(error::file_indexing)?;

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
    eprintln!("ERROR: {err}");
    fmt::Error
}
