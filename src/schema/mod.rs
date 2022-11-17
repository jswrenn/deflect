use std::fmt;

mod atom;
mod discriminant;
mod r#enum;
mod field;
mod fields;
mod function;
mod name;
mod offset;
mod r#ref;
mod r#struct;
mod variant;
mod variants;

pub use atom::Atom;
pub use discriminant::{Discriminant, DiscriminantValue};
pub use fields::{Fields, FieldsIter};
pub use function::Function;
pub use name::Name;
pub use offset::Offset;
pub use r#enum::Enum;
pub use r#field::Field;
pub use r#ref::Ref;
pub use r#struct::Struct;
pub use r#variant::Variant;
pub use variants::{Variants, VariantsIter};

/// A language type.
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    bool(Atom<'dwarf, bool, R>),
    char(Atom<'dwarf, char, R>),
    f32(Atom<'dwarf, f32, R>),
    f64(Atom<'dwarf, f64, R>),
    i8(Atom<'dwarf, i8, R>),
    i16(Atom<'dwarf, i16, R>),
    i32(Atom<'dwarf, i32, R>),
    i64(Atom<'dwarf, i64, R>),
    i128(Atom<'dwarf, i128, R>),
    isize(Atom<'dwarf, isize, R>),
    u8(Atom<'dwarf, u8, R>),
    u16(Atom<'dwarf, u16, R>),
    u32(Atom<'dwarf, u32, R>),
    u64(Atom<'dwarf, u64, R>),
    u128(Atom<'dwarf, u128, R>),
    usize(Atom<'dwarf, usize, R>),
    unit(Atom<'dwarf, (), R>),
    Struct(r#struct::Struct<'dwarf, R>),
    Enum(r#enum::Enum<'dwarf, R>),
    Ref(r#ref::Ref<'dwarf, R>),
    Function(function::Function<'dwarf, R>),
}

#[derive(Copy, Clone)]
pub enum DiscriminantType {
    U8,
    U16,
    U32,
    U64,
}

impl DiscriminantType {
    pub(crate) fn from_die<'dwarf, R>(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Self
    where
        R: crate::gimli::Reader<Offset = usize>,
    {
        match entry.tag() {
            crate::gimli::DW_TAG_base_type => {
                let slice = crate::get_name(&entry, dwarf, unit).unwrap();
                let slice = slice.to_slice().unwrap();
                match slice.as_ref() {
                    b"u8" => Self::U8,
                    b"u16" => Self::U16,
                    b"u32" => Self::U32,
                    b"u64" => Self::U64,
                    _ => todo!(),
                }
            }
            _ => panic!(),
        }
    }
}

impl<'dwarf, R> Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) fn from_die(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        Ok(match entry.tag() {
            crate::gimli::DW_TAG_base_type => {
                if let Some(name) = Name::from_die(dwarf, unit, &entry)? {
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
                        // b"!" => Self::never(Atom::from_dw_tag_base_type(dwarf, unit, entry)?),
                        _ => todo!("unhandled primtive {:?}", String::from_utf8_lossy(&name)),
                    }
                } else {
                    todo!("unhandled primitive");
                }
            }
            crate::gimli::DW_TAG_structure_type => {
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
            otherwise @ _ => {
                let mut tree = unit.entries_tree(Some(entry.offset())).unwrap();
                let _ = crate::inspect_tree(&mut tree, dwarf, unit);
                panic!("{}", otherwise.to_string())
            }
        })
    }

    pub fn size(&self) -> Result<Option<u64>, crate::Error> {
        match self {
            Type::Struct(v) => v.size(),
            Type::Enum(v) => v.size(),
            Type::Ref(v) => v.size(),
            Type::Function(_) => Ok(Some(0)),
            _ => todo!(),
        }
    }
}

impl<'dwarf, R> fmt::Display for Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Struct(v) => v.fmt(f),
            Type::Enum(v) => v.fmt(f),
            Type::Ref(v) => v.fmt(f),
            Type::Function(v) => v.fmt(f),
            _ => todo!(),
        }
    }
}
