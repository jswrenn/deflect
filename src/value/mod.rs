use std::fmt;

mod atom;
mod r#enum;
mod field;
mod r#ref;
mod r#struct;
mod variant;

pub use field::Field;
pub use r#enum::Enum;
pub use r#ref::Ref;
pub use r#struct::Struct;
pub use variant::Variant;

#[non_exhaustive]
pub enum Value<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    Bool(bool),
    Char(char),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    Str(&'value str),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Unit,
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
            crate::schema::Type::Bool => Self::Bool(unsafe { *(value as *const _ as *const _) }),
            crate::schema::Type::I8 => Self::I8(unsafe { *(value as *const _ as *const _) }),
            crate::schema::Type::I16 => Self::I16(unsafe { *(value as *const _ as *const _) }),
            crate::schema::Type::I32 => Self::I32(unsafe { *(value as *const _ as *const _) }),
            crate::schema::Type::I64 => Self::I64(unsafe { *(value as *const _ as *const _) }),
            crate::schema::Type::I128 => Self::I128(unsafe { *(value as *const _ as *const _) }),
            //crate::schema::Type::ISize => Self::I128(unsafe { *(value as *const _ as *const _) }),
            crate::schema::Type::F32 => Self::F32(unsafe { *(value as *const _ as *const _) }),
            crate::schema::Type::F64 => Self::F32(unsafe { *(value as *const _ as *const _) }),

            crate::schema::Type::U8 => Self::I8(unsafe { *(value as *const _ as *const _) }),
            crate::schema::Type::U16 => Self::I16(unsafe { *(value as *const _ as *const _) }),
            crate::schema::Type::U32 => Self::I32(unsafe { *(value as *const _ as *const _) }),
            crate::schema::Type::U64 => Self::I64(unsafe { *(value as *const _ as *const _) }),
            crate::schema::Type::U128 => Self::I128(unsafe { *(value as *const _ as *const _) }),
            //crate::schema::Type::USize => Self::I128(unsafe { *(value as *const _ as *const _) }),
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
            Self::Bool(v) => v.fmt(f),
            Self::Char(v) => v.fmt(f),
            Self::F32(v) => v.fmt(f),
            Self::F64(v) => v.fmt(f),
            Self::I8(v) => v.fmt(f),
            Self::I16(v) => v.fmt(f),
            Self::I32(v) => v.fmt(f),
            Self::I64(v) => v.fmt(f),
            Self::I128(v) => v.fmt(f),
            Self::Isize(v) => v.fmt(f),
            Self::Str(v) => v.fmt(f),
            Self::U8(v) => v.fmt(f),
            Self::U16(v) => v.fmt(f),
            Self::U32(v) => v.fmt(f),
            Self::U64(v) => v.fmt(f),
            Self::U128(v) => v.fmt(f),
            Self::Usize(v) => v.fmt(f),
            Self::Unit => ().fmt(f),
            Self::Struct(v) => v.fmt(f),
            Self::Enum(v) => v.fmt(f),
            Self::Ref(v) => v.fmt(f),
            Self::Function(s) => s.fmt(f),
        }
    }
}
