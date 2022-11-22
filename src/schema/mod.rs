use std::fmt;

mod array;
mod atom;
mod data;
mod r#enum;
mod field;
mod fields;
mod function;
mod name;
mod offset;
mod r#ref;
mod slice;
mod r#struct;
mod variant;
mod variants;

pub use array::Array;
pub use atom::Atom;
pub use data::Data;
pub use fields::{Fields, FieldsIter};
pub use function::Function;
pub use name::Name;
pub use offset::Offset;
pub use r#enum::Enum;
pub use r#field::Field;
pub use r#ref::Ref;
pub use r#struct::Struct;
pub use r#variant::Variant;
pub use slice::Slice;
pub use variants::{Variants, VariantsIter};

/// A language type.
#[allow(non_camel_case_types)]
#[derive(Clone)]
#[non_exhaustive]
pub enum Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// A [`bool`].
    bool(Atom<'dwarf, bool, R>),
    /// A [`char`].
    char(Atom<'dwarf, char, R>),
    /// A [`f32`].
    f32(Atom<'dwarf, f32, R>),
    /// A [`f64`].
    f64(Atom<'dwarf, f64, R>),
    /// A [`i8`].
    i8(Atom<'dwarf, i8, R>),
    /// A [`i16`].
    i16(Atom<'dwarf, i16, R>),
    /// A [`i32`].
    i32(Atom<'dwarf, i32, R>),
    /// A [`i64`].
    i64(Atom<'dwarf, i64, R>),
    /// A [`i128`].
    i128(Atom<'dwarf, i128, R>),
    /// A [`isize`].
    isize(Atom<'dwarf, isize, R>),
    /// A [`u8`].
    u8(Atom<'dwarf, u8, R>),
    /// A [`u16`].
    u16(Atom<'dwarf, u16, R>),
    /// A [`u32`].
    u32(Atom<'dwarf, u32, R>),
    /// A [`u64`].
    u64(Atom<'dwarf, u64, R>),
    /// A [`u128`].
    u128(Atom<'dwarf, u128, R>),
    /// A [`usize`].
    usize(Atom<'dwarf, usize, R>),
    /// A [`()`].
    unit(Atom<'dwarf, (), R>),
    /// An array.
    Array(Array<'dwarf, R>),
    /// A slice.
    Slice(Slice<'dwarf, R>),
    /// A `struct`.
    Struct(r#struct::Struct<'dwarf, R>),
    /// An `enum`.
    Enum(r#enum::Enum<'dwarf, R>),
    /// A reference type.
    Ref(r#ref::Ref<'dwarf, R>),
    /// A function type.
    Function(function::Function<'dwarf, R>),
}

impl<'dwarf, R> Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) fn from_die(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::err::Error> {
        Ok(match entry.tag() {
            crate::gimli::DW_TAG_base_type => {
                if let name = Name::from_die(dwarf, unit, &entry)? {
                    let name = name.to_slice()?;
                    match name.as_ref() {
                        b"bool" => Self::bool(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"char" => Self::char(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"f32" => Self::f32(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"f64" => Self::f64(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"i8" => Self::i8(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"i16" => Self::i16(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"i32" => Self::i32(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"i64" => Self::i64(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"i128" => Self::i128(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"isize" => Self::isize(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"u8" => Self::u8(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"u16" => Self::u16(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"u32" => Self::u32(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"u64" => Self::u64(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"u128" => Self::u128(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"usize" => Self::usize(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        b"()" => Self::unit(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        _ => unimplemented!(
                            "unhandled primitive: {:#?}",
                            crate::debug::DebugEntry::new(dwarf, unit, &entry)
                        ),
                    }
                } else {
                    unimplemented!(
                        "unhandled primitive: {:#?}",
                        crate::debug::DebugEntry::new(dwarf, unit, &entry)
                    )
                }
            }
            crate::gimli::DW_TAG_structure_type => {
                let name = Name::from_die(dwarf, unit, &entry)?;
                if name.to_slice()?.starts_with(b"&[") {
                    return Ok(Self::Slice(Slice::from_dw_tag_structure_type(
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
                    Self::Enum(r#enum::Enum::from_dw_tag_structure_type(
                        dwarf, unit, entry,
                    )?)
                } else {
                    Self::Struct(r#struct::Struct::from_dw_tag_structure_type(
                        dwarf, unit, entry,
                    )?)
                }
            }
            crate::gimli::DW_TAG_enumeration_type => {
                Self::Enum(r#enum::Enum::from_dw_tag_enumeration_type(dwarf, unit, entry).unwrap())
            }
            crate::gimli::DW_TAG_pointer_type => {
                Self::Ref(Ref::from_dw_pointer_type(dwarf, unit, entry))
            }
            crate::gimli::DW_TAG_subroutine_type => {
                Self::Function(Function::from_dw_tag_subroutine_type(dwarf, unit, entry)?)
            }
            crate::gimli::DW_TAG_array_type => {
                Self::Array(Array::from_dw_tag_subroutine_type(dwarf, unit, entry)?)
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

    pub fn size(&self) -> Result<u64, crate::err::Error> {
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
            Self::Array(v) => v.size(),
            Self::Slice(v) => v.size(),
            Self::Struct(v) => v.size(),
            Self::Enum(v) => v.size(),
            Self::Ref(v) => Ok(v.size()?),
            Self::Function(v) => Ok(0),
        }
    }
}

impl<'value, 'dwarf, R> fmt::Debug for Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
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
            Self::Array(v) => v.fmt(f),
            Self::Slice(v) => v.fmt(f),
            Self::Struct(v) => v.fmt(f),
            Self::Enum(v) => v.fmt(f),
            Self::Ref(v) => v.fmt(f),
            Self::Function(v) => v.fmt(f),
        }
    }
}

impl<'value, 'dwarf, R> fmt::Display for Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
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
            Self::Array(v) => v.fmt(f),
            Self::Slice(v) => v.fmt(f),
            Self::Struct(v) => v.fmt(f),
            Self::Enum(v) => v.fmt(f),
            Self::Ref(v) => v.fmt(f),
            Self::Function(v) => v.fmt(f),
        }
    }
}
