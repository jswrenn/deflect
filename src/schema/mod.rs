//! Reflections of Rust types.

use std::{fmt, primitive};

mod array_impl;
mod data;
mod enum_impl;
mod field;
mod fields;
mod function;
mod name;
mod offset;
mod pointer;
mod slice_impl;
mod struct_impl;
mod variant;
mod variants;

pub use array_impl::array;
pub use data::Data;
pub use fields::{Fields, FieldsIter};
pub use function::Function;
pub use name::Name;
pub use offset::Offset;
pub use pointer::{Const, Mut, Pointer, Reference, Shared, Unique};
pub use enum_impl::Enum;
pub use r#field::Field;
pub use struct_impl::Struct;
pub use r#variant::Variant;
pub use slice_impl::slice;
pub use variants::{Variants, VariantsIter};

/// A reflected language type.
#[allow(non_camel_case_types)]
#[derive(Clone)]
#[non_exhaustive]
pub enum Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = primitive::usize>,
{
    /// A reflected [`prim@bool`].
    bool(bool<'dwarf, R>),
    /// A reflected [`prim@char`].
    char(char<'dwarf, R>),
    /// A reflected [`prim@f32`].
    f32(f32<'dwarf, R>),
    /// A reflected [`prim@f64`].
    f64(f64<'dwarf, R>),
    /// A reflected [`prim@i8`].
    i8(i8<'dwarf, R>),
    /// A reflected [`prim@i16`].
    i16(i16<'dwarf, R>),
    /// A reflected [`prim@i32`].
    i32(i32<'dwarf, R>),
    /// A reflected [`prim@i64`].
    i64(i64<'dwarf, R>),
    /// A reflected [`prim@i128`].
    i128(i128<'dwarf, R>),
    /// A reflected [`prim@isize`].
    isize(isize<'dwarf, R>),
    /// A reflected [`prim@u8`].
    u8(u8<'dwarf, R>),
    /// A reflected [`prim@u16`].
    u16(u16<'dwarf, R>),
    /// A reflected [`prim@u32`].
    u32(u32<'dwarf, R>),
    /// A reflected [`prim@u64`].
    u64(u64<'dwarf, R>),
    /// A reflected [`prim@u128`].
    u128(u128<'dwarf, R>),
    /// A reflected [`prim@usize`].
    usize(usize<'dwarf, R>),
    /// A reflected [`()`][prim@unit].
    unit(unit<'dwarf, R>),
    /// A reflected [`[T; N]`][prim@array].
    array(array<'dwarf, R>),
    /// A reflected [`&[T]`][prim@slice].
    slice(slice<'dwarf, R>),
    /// A reflected [`struct`](https://doc.rust-lang.org/std/keyword.struct.html).
    Struct(Struct<'dwarf, R>),
    /// A reflected [`struct`](https://doc.rust-lang.org/std/keyword.enum.html).
    Enum(Enum<'dwarf, R>),
    /// A shared reference type.
    SharedRef(Pointer<'dwarf, pointer::Shared, R>),
    /// A unique reference type.
    UniqueRef(Pointer<'dwarf, pointer::Unique, R>),
    /// A `const` pointer type.
    ConstPtr(Pointer<'dwarf, pointer::Const, R>),
    /// A `mut` pointer type.
    MutPtr(Pointer<'dwarf, pointer::Mut, R>),
    /// A reflected [`fn`][prim@fn].
    Function(function::Function<'dwarf, R>),
}

impl<'dwarf, R> Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) fn from_die(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::error::Error> {
        Ok(match entry.tag() {
            crate::gimli::DW_TAG_base_type => {
                let name = Name::from_die(dwarf, unit, &entry)?;
                let name = name.to_slice()?;
                match name.as_ref() {
                    b"bool" => Self::bool(bool::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"char" => Self::char(char::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"f32" => Self::f32(f32::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"f64" => Self::f64(f64::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"i8" => Self::i8(i8::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"i16" => Self::i16(i16::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"i32" => Self::i32(i32::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"i64" => Self::i64(i64::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"i128" => Self::i128(i128::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"isize" => Self::isize(isize::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"u8" => Self::u8(u8::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"u16" => Self::u16(u16::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"u32" => Self::u32(u32::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"u64" => Self::u64(u64::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"u128" => Self::u128(u128::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"usize" => Self::usize(usize::from_dw_tag_base_type(dwarf, unit, entry)?),
                    b"()" => Self::unit(unit::from_dw_tag_base_type(dwarf, unit, entry)?),
                    _ => unimplemented!(
                        "unhandled primitive: {:#?}",
                        crate::debug::DebugEntry::new(dwarf, unit, &entry)
                    ),
                }
            }
            crate::gimli::DW_TAG_structure_type => {
                let name = Name::from_die(dwarf, unit, &entry)?;
                if name.to_slice()?.starts_with(b"&[") {
                    return Ok(Self::slice(slice::from_dw_tag_structure_type(
                        dwarf, unit, entry,
                    )?));
                }

                let mut tree = unit.entries_tree(Some(entry.offset())).unwrap();
                let root = tree.root().unwrap();
                let mut children = root.children();
                let mut variants = None;

                while let Some(child) = children.next().unwrap() {
                    if child.entry().tag() == crate::gimli::DW_TAG_variant_part {
                        variants = Some(child.entry().clone());
                        break;
                    }
                }

                if let Some(_variants) = variants {
                    Self::Enum(Enum::from_dw_tag_structure_type(
                        dwarf, unit, entry,
                    )?)
                } else {
                    Self::Struct(Struct::from_dw_tag_structure_type(
                        dwarf, unit, entry,
                    )?)
                }
            }
            crate::gimli::DW_TAG_enumeration_type => Self::Enum(
                Enum::from_dw_tag_enumeration_type(dwarf, unit, entry)?,
            ),
            crate::gimli::DW_TAG_pointer_type => {
                let name = Name::from_die(dwarf, unit, &entry)
                    .map(Some)
                    .or_else(|err| {
                        if let crate::error::Kind::MissingAttr(_) = err.kind {
                            Ok(None)
                        } else {
                            Err(err)
                        }
                    })?;
                let target = crate::get_type_ref(&entry)?;
                if let Some(name) = name {
                    let name_as_slice = name.to_slice()?;
                    if name_as_slice.starts_with(b"*mut ") {
                        Self::MutPtr(Pointer::new(
                            dwarf,
                            unit,
                            entry.offset(),
                            Some(name),
                            target,
                        ))
                    } else if name_as_slice.starts_with(b"*const ") {
                        Self::ConstPtr(Pointer::new(
                            dwarf,
                            unit,
                            entry.offset(),
                            Some(name),
                            target,
                        ))
                    } else if name_as_slice.starts_with(b"&mut ") {
                        Self::UniqueRef(Pointer::new(
                            dwarf,
                            unit,
                            entry.offset(),
                            Some(name),
                            target,
                        ))
                    } else if name_as_slice.starts_with(b"&") {
                        Self::SharedRef(Pointer::new(
                            dwarf,
                            unit,
                            entry.offset(),
                            Some(name),
                            target,
                        ))
                    } else {
                        return Err(crate::error::Kind::invalid_attr(crate::gimli::DW_AT_name).into());
                    }
                } else {
                    // the `data_ptr` field of slices points to a pointer type that doesn't have a name.
                    Self::MutPtr(Pointer::new(dwarf, unit, entry.offset(), None, target))
                }
            }
            crate::gimli::DW_TAG_subroutine_type => {
                Self::Function(Function::from_dw_tag_subroutine_type(dwarf, unit, entry)?)
            }
            crate::gimli::DW_TAG_array_type => {
                Self::array(array::from_dw_tag_array_type(dwarf, unit, entry)?)
            }
            otherwise => {
                let _tree = unit.entries_tree(Some(entry.offset())).unwrap();
                //let _ = crate::debug::inspect_tree(&mut tree, dwarf, unit);
                panic!(
                    "{:#?}",
                    &crate::debug::DebugEntry::new(dwarf, unit, &entry,)
                );
            }
        })
    }

    pub fn size(&self) -> Result<std::primitive::u64, crate::error::Error> {
        match self {
            Self::bool(v) => Ok(v.size()),
            Self::char(v) => Ok(v.size()),
            Self::f32(v) => Ok(v.size()),
            Self::f64(v) => Ok(v.size()),
            Self::i8(v) => Ok(v.size()),
            Self::i16(v) => Ok(v.size()),
            Self::i32(v) => Ok(v.size()),
            Self::i64(v) => Ok(v.size()),
            Self::i128(v) => Ok(v.size()),
            Self::isize(v) => Ok(v.size()),
            Self::u8(v) => Ok(v.size()),
            Self::u16(v) => Ok(v.size()),
            Self::u32(v) => Ok(v.size()),
            Self::u64(v) => Ok(v.size()),
            Self::u128(v) => Ok(v.size()),
            Self::usize(v) => Ok(v.size()),
            Self::unit(v) => Ok(v.size()),
            Self::array(v) => v.bytes(),
            Self::slice(v) => v.size(),
            Self::Struct(v) => v.size(),
            Self::Enum(v) => v.size(),
            Self::Function(_) => Ok(0),
            Self::SharedRef(_) => Ok(std::mem::size_of::<std::primitive::usize>() as _),
            Self::UniqueRef(_) => Ok(std::mem::size_of::<std::primitive::usize>() as _),
            Self::ConstPtr(_) => Ok(std::mem::size_of::<std::primitive::usize>() as _),
            Self::MutPtr(_) => Ok(std::mem::size_of::<std::primitive::usize>() as _),
        }
    }
}

impl<'value, 'dwarf, R> fmt::Debug for Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::bool(v) => v.fmt(f),
            Self::char(v) => v.fmt(f),
            Self::f32(v) => v.fmt(f),
            Self::f64(v) => v.fmt(f),
            Self::i8(v) => v.fmt(f),
            Self::i16(v) => v.fmt(f),
            Self::i32(v) => v.fmt(f),
            Self::i64(v) => v.fmt(f),
            Self::i128(v) => v.fmt(f),
            Self::isize(v) => v.fmt(f),
            Self::u8(v) => v.fmt(f),
            Self::u16(v) => v.fmt(f),
            Self::u32(v) => v.fmt(f),
            Self::u64(v) => v.fmt(f),
            Self::u128(v) => v.fmt(f),
            Self::usize(v) => v.fmt(f),
            Self::unit(v) => v.fmt(f),
            Self::array(v) => v.fmt(f),
            Self::slice(v) => v.fmt(f),
            Self::Struct(v) => v.fmt(f),
            Self::Enum(v) => v.fmt(f),
            Self::Function(v) => v.fmt(f),
            Self::SharedRef(v) => v.fmt(f),
            Self::UniqueRef(v) => v.fmt(f),
            Self::ConstPtr(v) => v.fmt(f),
            Self::MutPtr(v) => v.fmt(f),
        }
    }
}

impl<'value, 'dwarf, R> fmt::Display for Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::bool(v) => v.fmt(f),
            Self::char(v) => v.fmt(f),
            Self::f32(v) => v.fmt(f),
            Self::f64(v) => v.fmt(f),
            Self::i8(v) => v.fmt(f),
            Self::i16(v) => v.fmt(f),
            Self::i32(v) => v.fmt(f),
            Self::i64(v) => v.fmt(f),
            Self::i128(v) => v.fmt(f),
            Self::isize(v) => v.fmt(f),
            Self::u8(v) => v.fmt(f),
            Self::u16(v) => v.fmt(f),
            Self::u32(v) => v.fmt(f),
            Self::u64(v) => v.fmt(f),
            Self::u128(v) => v.fmt(f),
            Self::usize(v) => v.fmt(f),
            Self::unit(v) => v.fmt(f),
            Self::array(v) => v.fmt(f),
            Self::slice(v) => v.fmt(f),
            Self::Struct(v) => v.fmt(f),
            Self::Enum(v) => v.fmt(f),
            Self::SharedRef(v) => v.fmt(f),
            Self::Function(v) => v.fmt(f),
            Self::SharedRef(v) => v.fmt(f),
            Self::UniqueRef(v) => v.fmt(f),
            Self::ConstPtr(v) => v.fmt(f),
            Self::MutPtr(v) => v.fmt(f),
        }
    }
}

macro_rules! generate_primitive {
    ($($t:ident,)*) => {
        $(
            generate_primitive!(@
                $t,
                concat!(
                    "A schema for [`",
                    stringify!($t),
                    "`][std::primitive::",
                    stringify!($t),
                    "]."
                )
            );
        )*
    };
    (@ $t:ident, $doc:expr) => {
        #[doc = $doc]
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct $t<'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            dwarf: &'dwarf crate::gimli::Dwarf<R>,
            unit: &'dwarf crate::gimli::Unit<R, std::primitive::usize>,
            entry: crate::gimli::UnitOffset,
        }

        impl<'dwarf, R> $t<'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            pub(crate) fn from_dw_tag_base_type(
                dwarf: &'dwarf crate::gimli::Dwarf<R>,
                unit: &'dwarf crate::gimli::Unit<R, std::primitive::usize>,
                entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
            ) -> Result<Self, crate::error::Error> {
                crate::check_tag(&entry, crate::gimli::DW_TAG_base_type)?;

                let name = Name::from_die(dwarf, unit, &entry)?;
                let expected = std::any::type_name::<std::primitive::$t>();
                if name.to_slice()? != expected.as_bytes() {
                    let actual = name.to_string_lossy()?.to_string();
                    Err(crate::error::Kind::name_mismatch(expected, actual))?;
                }

                let size: std::primitive::usize = crate::get_size(&entry)?
                    .try_into()
                    .map_err(crate::error::Kind::TryFromInt)?;
                let expected = core::mem::size_of::<std::primitive::$t>();
                if size != expected {
                    Err(crate::error::Kind::size_mismatch(expected, size))?;
                }

                Ok(Self {
                    dwarf,
                    unit,
                    entry: entry.offset()
                })
            }


            /// The size of this type.
            pub fn name(&self) -> &'static str {
                std::any::type_name::<std::primitive::$t>()
            }

            /// The size of this type.
            pub fn size(&self) -> std::primitive::u64 {
                std::mem::size_of::<std::primitive::$t>() as _
            }

            /// The minimum alignment of this type.
            pub fn align(&self) -> std::primitive::u64 {
                std::mem::align_of::<std::primitive::$t>() as _
            }
        }

        impl<'dwarf, R> std::fmt::Debug for $t<'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let entry = self.unit.entry(self.entry).map_err(crate::fmt_err)?;
                let mut debug_tuple = f.debug_tuple(stringify!($t));
                debug_tuple.field(&crate::debug::DebugEntry::new(
                    self.dwarf,
                    self.unit,
                    &entry,
                ));
                debug_tuple.finish()
            }
        }

        impl<'dwarf, R> std::fmt::Display for $t<'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.name().fmt(f)
            }
        }
    };
}

generate_primitive! {
    bool,
    char,
    f32,
    f64,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
}

/// A schema for [`()`][prim@unit].
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct unit<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, std::primitive::usize>,
    entry: crate::gimli::UnitOffset,
}

impl<'dwarf, R> unit<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) fn from_dw_tag_base_type(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, std::primitive::usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::error::Error> {
        crate::check_tag(&entry, crate::gimli::DW_TAG_base_type)?;

        let name = Name::from_die(dwarf, unit, &entry)?;
        let expected = std::any::type_name::<()>();
        if name.to_slice()? != expected.as_bytes() {
            let actual = name.to_string_lossy()?.to_string();
            Err(crate::error::Kind::name_mismatch(expected, actual))?;
        }

        let size: std::primitive::usize = crate::get_size(&entry)?
            .try_into()
            .map_err(crate::error::Kind::TryFromInt)?;
        let expected = core::mem::size_of::<()>();
        if size != expected {
            Err(crate::error::Kind::size_mismatch(expected, size))?;
        }

        Ok(Self {
            dwarf,
            unit,
            entry: entry.offset(),
        })
    }

    /// The size of this type.
    pub fn name(&self) -> &'static str {
        std::any::type_name::<()>()
    }

    /// The size of this type.
    pub fn size(&self) -> std::primitive::u64 {
        std::mem::size_of::<()>() as _
    }

    /// The minimum alignment of this type.
    pub fn align(&self) -> std::primitive::u64 {
        std::mem::align_of::<()>() as _
    }
}

impl<'dwarf, R> std::fmt::Debug for unit<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entry = self.unit.entry(self.entry).map_err(crate::fmt_err)?;
        let mut debug_tuple = f.debug_tuple(stringify!($t));
        debug_tuple.field(&crate::debug::DebugEntry::new(
            self.dwarf, self.unit, &entry,
        ));
        debug_tuple.finish()
    }
}

impl<'dwarf, R> std::fmt::Display for unit<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
    }
}
