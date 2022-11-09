mod discriminant;
mod r#enum;
mod field;
mod r#ref;
mod r#struct;
mod variant;

pub use discriminant::{Discriminant, DiscriminantValue};
pub use r#enum::Enum;
pub use r#field::Field;
pub use r#ref::Ref;
pub use r#struct::Struct;
pub use r#variant::Variant;

#[derive(Debug)]
#[non_exhaustive]
pub enum Type<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    Bool,
    Char,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    Str,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    Unit,
    Struct(r#struct::Struct<'dwarf, R>),
    Enum(r#enum::Enum<'dwarf, R>),
    Ref(r#ref::Ref<'dwarf, R>),
}

#[derive(Clone)]
pub enum DiscriminantType {
    U8,
    U16,
    U32,
    U64,
}

impl DiscriminantType {
    pub(crate) fn from_die<'dwarf, R>(
        dwarf: &'dwarf gimli::Dwarf<R>,
        unit: &'dwarf gimli::Unit<R>,
        entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Self
    where
        R: gimli::Reader<Offset = usize>,
    {
        match entry.tag() {
            gimli::DW_TAG_base_type => {
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
    R: gimli::Reader<Offset = usize>,
{
    pub(crate) fn from_die(
        dwarf: &'dwarf gimli::Dwarf<R>,
        unit: &'dwarf gimli::Unit<R>,
        entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        let mut tree = unit.entries_tree(Some(entry.offset())).unwrap();
        Ok(match entry.tag() {
            gimli::DW_TAG_base_type => {
                let slice = crate::get_name(&entry, dwarf, unit).unwrap();

                let slice = slice.to_slice().unwrap();

                match slice.as_ref() {
                    b"i8" => Self::I8,
                    b"i16" => Self::I16,
                    b"i32" => Self::I32,
                    b"i64" => Self::I64,
                    b"i128" => Self::I128,

                    b"u8" => Self::U8,
                    b"u16" => Self::U16,
                    b"u32" => Self::U32,
                    b"u64" => Self::U64,
                    b"u128" => Self::U128,
                    _ => todo!(),
                }
            }
            gimli::DW_TAG_structure_type => {
                let mut tree = unit.entries_tree(Some(entry.offset())).unwrap();
                let root = tree.root().unwrap();
                let mut children = root.children();
                let mut variants = None;

                while let Some(child) = children.next().unwrap() {
                    if child.entry().tag() == gimli::DW_TAG_variant_part {
                        variants = Some(child.entry().clone());
                        break;
                    }
                }

                if let Some(_variants) = variants {
                    Self::Enum(r#enum::Enum::from_dw_tag_structure_type(dwarf, unit, entry)?)
                } else {
                    Self::Struct(r#struct::Struct::from_dw_tag_structure_type(
                        dwarf, unit, entry,
                    )?)
                }
            }
            gimli::DW_TAG_enumeration_type => Self::Enum(r#enum::Enum::from_dw_tag_enumeration_type(dwarf, unit, entry).unwrap()),
            gimli::DW_TAG_pointer_type => {
                //let mut tree = unit.entries_tree(Some(entry.offset())).unwrap();
                //crate::inspect_tree(&mut tree, dwarf, unit);
                Self::Ref(Ref::from_dw_pointer_type(dwarf, unit, entry))
            }
            otherwise @ _ => panic!("{}", otherwise.to_string()),
        })
    }

    pub fn size(&self) -> usize {
        match self {
            Type::Bool => 1,
            Type::Char => 4,
            Type::F32 => 4,
            Type::F64 => 8,
            Type::I8 => 1,
            Type::I16 => 2,
            Type::I32 => 4,
            Type::I64 => 8,
            Type::I128 => 16,
            Type::Isize => todo!(),
            Type::Str => todo!(),
            Type::U8 => 1,
            Type::U16 => 2,
            Type::U32 => 4,
            Type::U64 => 8,
            Type::U128 => 16,
            Type::Usize => todo!(),
            Type::Unit => 0,
            Type::Struct(v) => v.size(),
            Type::Enum(v) => v.size(),
            Type::Ref(_) => 8,
        }
    }
}
