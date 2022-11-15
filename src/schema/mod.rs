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

pub use atom::{Atom, RustAtom};
pub use discriminant::{Discriminant, DiscriminantValue};
pub use function::Function;
pub use name::Name;
pub use offset::Offset;
pub use r#enum::Enum;
pub use r#field::Field;
pub use fields::{Fields, FieldsIter};
pub use r#ref::Ref;
pub use r#struct::Struct;
pub use r#variant::Variant;
pub use variants::{Variants, VariantsIter};

/// A language type.
#[derive(Debug)]
#[non_exhaustive]
pub enum Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    Atom(Atom<'dwarf, R>),
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
                Self::Atom(Atom::from_dw_tag_base_type(dwarf, unit, entry)?)
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
            Type::Atom(v) => v.size(),
            Type::Struct(v) => v.size(),
            Type::Enum(v) => v.size(),
            Type::Ref(v) => v.size(),
            Type::Function(_) => Ok(Some(0)),
            _ => todo!(),
        }
    }
}
