use std::fmt;

mod atom;
mod r#enum;
mod field;
mod fields;
mod r#ref;
mod r#struct;
mod variant;

pub use atom::{Atom, RustAtom};
pub use field::Field;
pub use fields::{Fields, FieldsIter};
pub use r#enum::Enum;
pub use r#ref::Ref;
pub use r#struct::Struct;
pub use variant::Variant;

#[non_exhaustive]
pub enum Value<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    Atom(Atom<'dwarf, 'value, R>),
    Struct(r#struct::Struct<'dwarf, 'value, R>),
    Enum(r#enum::Enum<'dwarf, 'value, R>),
    Ref(r#ref::Ref<'dwarf, 'value, R>),
    Function(crate::schema::Function<'dwarf, R>),
}

impl<'dwarf, 'value, R> Value<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Safety: `value` absolutely must have the correct `type`.
    pub(crate) unsafe fn with_type(
        r#type: crate::schema::Type<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        match r#type {
            crate::schema::Type::Atom(r#type) => Self::Atom(Atom::new(r#type, value)),
            crate::schema::Type::Struct(r#type) => Self::Struct(Struct::new(r#type, value)),
            crate::schema::Type::Enum(r#type) => Self::Enum(Enum::new(r#type, value)),
            crate::schema::Type::Ref(r#type) => Self::Ref(Ref::new(r#type, value)),
            crate::schema::Type::Function(r#type) => Self::Function(r#type),
            _ => todo!(),
        }
    }
}

impl<'dwarf, 'value, R> fmt::Debug for Value<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(v) => v.fmt(f),
            Self::Struct(v) => v.fmt(f),
            Self::Enum(v) => v.fmt(f),
            Self::Ref(v) => v.fmt(f),
            Self::Function(s) => s.fmt(f),
        }
    }
}
