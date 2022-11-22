use std::fmt;

mod array;
mod atom;
mod r#enum;
mod field;
mod fields;
mod function;
mod r#ref;
mod slice;
mod r#struct;
mod variant;

pub use array::Array;
pub use atom::Atom;
pub use field::Field;
pub use fields::{Fields, FieldsIter};
pub use function::Function;
pub use r#enum::Enum;
pub use r#ref::Ref;
pub use r#struct::Struct;
pub use slice::Slice;
pub use variant::Variant;

#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    bool(Atom<'value, 'dwarf, bool, R>),
    char(Atom<'value, 'dwarf, char, R>),
    f32(Atom<'value, 'dwarf, f32, R>),
    f64(Atom<'value, 'dwarf, f64, R>),
    i8(Atom<'value, 'dwarf, i8, R>),
    i16(Atom<'value, 'dwarf, i16, R>),
    i32(Atom<'value, 'dwarf, i32, R>),
    i64(Atom<'value, 'dwarf, i64, R>),
    i128(Atom<'value, 'dwarf, i128, R>),
    isize(Atom<'value, 'dwarf, isize, R>),
    u8(Atom<'value, 'dwarf, u8, R>),
    u16(Atom<'value, 'dwarf, u16, R>),
    u32(Atom<'value, 'dwarf, u32, R>),
    u64(Atom<'value, 'dwarf, u64, R>),
    u128(Atom<'value, 'dwarf, u128, R>),
    usize(Atom<'value, 'dwarf, usize, R>),
    unit(Atom<'value, 'dwarf, (), R>),
    Array(Array<'value, 'dwarf, R>),
    Slice(Slice<'value, 'dwarf, R>),
    Struct(r#struct::Struct<'value, 'dwarf, R>),
    Enum(r#enum::Enum<'value, 'dwarf, R>),
    Ref(r#ref::Ref<'value, 'dwarf, R>),
    Function(Function<'value, 'dwarf, R>),
}

impl<'value, 'dwarf, R> Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Safety: `value` absolutely must have the correct `type`.
    pub(crate) unsafe fn with_type(
        r#type: crate::schema::Type<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        match r#type {
            crate::schema::Type::bool(schema) => Self::bool(Atom::new(value, schema)),
            crate::schema::Type::char(schema) => Self::char(Atom::new(value, schema)),
            crate::schema::Type::f32(schema) => Self::f32(Atom::new(value, schema)),
            crate::schema::Type::f64(schema) => Self::f64(Atom::new(value, schema)),
            crate::schema::Type::i8(schema) => Self::i8(Atom::new(value, schema)),
            crate::schema::Type::i16(schema) => Self::i16(Atom::new(value, schema)),
            crate::schema::Type::i32(schema) => Self::i32(Atom::new(value, schema)),
            crate::schema::Type::i64(schema) => Self::i64(Atom::new(value, schema)),
            crate::schema::Type::i128(schema) => Self::i128(Atom::new(value, schema)),
            crate::schema::Type::isize(schema) => Self::isize(Atom::new(value, schema)),
            crate::schema::Type::u8(schema) => Self::u8(Atom::new(value, schema)),
            crate::schema::Type::u16(schema) => Self::u16(Atom::new(value, schema)),
            crate::schema::Type::u32(schema) => Self::u32(Atom::new(value, schema)),
            crate::schema::Type::u64(schema) => Self::u64(Atom::new(value, schema)),
            crate::schema::Type::u128(schema) => Self::u128(Atom::new(value, schema)),
            crate::schema::Type::usize(schema) => Self::usize(Atom::new(value, schema)),
            crate::schema::Type::unit(schema) => Self::unit(Atom::new(value, schema)),
            crate::schema::Type::Slice(schema) => Self::Slice(Slice::new(value, schema).unwrap()),
            crate::schema::Type::Array(schema) => Self::Array(Array::new(value, schema).unwrap()),
            crate::schema::Type::Struct(r#type) => Self::Struct(Struct::new(r#type, value)),
            crate::schema::Type::Enum(r#type) => Self::Enum(Enum::new(r#type, value)),
            crate::schema::Type::Ref(r#type) => Self::Ref(Ref::new(r#type, value)),
            crate::schema::Type::Function(r#type) => Self::Function(Function::new(value, r#type)),
        }
    }
}

impl<'value, 'dwarf, R> fmt::Debug for Value<'value, 'dwarf, R>
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

impl<'value, 'dwarf, R> fmt::Display for Value<'value, 'dwarf, R>
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
            Self::unit(_) => f.write_str("()"),
            Self::Slice(v) => v.fmt(f),
            Self::Array(v) => v.fmt(f),
            Self::Struct(v) => v.fmt(f),
            Self::Enum(v) => v.fmt(f),
            Self::Ref(v) => v.fmt(f),
            Self::Function(v) => v.fmt(f),
        }
    }
}

/// Upcast a `Ref<'value, 'dwarf, R>` to a `Ref<'value, 'dwarf, R>`.
impl<'value, 'dwarf, R> From<Ref<'value, 'dwarf, R>> for Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn from(atom: Ref<'value, 'dwarf, R>) -> Self {
        Value::Ref(atom)
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Ref<'value, 'dwarf, R>`.
impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>> for &'a Ref<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Error = crate::DowncastErr<&'a Value<'value, 'dwarf, R>, Self>;

    fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::Ref(value) = value {
            Ok(value)
        } else {
            Err(crate::DowncastErr::new(value))
        }
    }
}

/// Attempt to downcast a `Value<'value, 'dwarf, R>` into a `Ref<'value, 'dwarf, R>`.
impl<'a, 'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>> for Ref<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Error = crate::DowncastErr<Value<'value, 'dwarf, R>, Self>;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::Ref(value) = value {
            Ok(value)
        } else {
            Err(crate::DowncastErr::new(value))
        }
    }
}

macro_rules! generate_primitive_conversions {
    ($($t:ident,)*) => {
        $(
            /// Upcast a `Atom<'value, 'dwarf, $t, R>` to a `Value<'value, 'dwarf, R>`.
            impl<'value, 'dwarf, R> From<Atom<'value, 'dwarf, $t, R>> for Value<'value, 'dwarf, R>
            where
                R: crate::gimli::Reader<Offset = usize>,
            {
                fn from(atom: Atom<'value, 'dwarf, $t, R>) -> Self {
                    Value::$t(atom)
                }
            }

            /// Downcast an `&Atom<'value, 'dwarf, T, R>` into a `&'value T`.
            impl<'a, 'value, 'dwarf, R> From<&'a Atom<'value, 'dwarf, $t, R>> for &'value $t
            where
                R: crate::gimli::Reader<Offset = usize>,
            {
                fn from(atom: &'a Atom<'value, 'dwarf, $t, R>) -> Self {
                    atom.value()
                }
            }

            /// Downcast an `Atom<'value, 'dwarf, T, R>` into a `&'value T`.
            impl<'value, 'dwarf, R> From<Atom<'value, 'dwarf, $t, R>> for &'value $t
            where
                R: crate::gimli::Reader<Offset = usize>,
            {
                fn from(atom: Atom<'value, 'dwarf, $t, R>) -> Self {
                    atom.value()
                }
            }

            /// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Atom<'value, 'dwarf, T, R>`.
            impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>> for &'a Atom<'value, 'dwarf, $t, R>
            where
                R: crate::gimli::Reader<Offset = usize>,
            {
                type Error = crate::DowncastErr<&'a Value<'value, 'dwarf, R>, Self>;

                fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                    if let Value::$t(value) = value {
                        Ok(value)
                    } else {
                        Err(crate::DowncastErr::new(value))
                    }
                }
            }

            /// Attempt to downcast a `Value<'value, 'dwarf, R>` into a `Atom<'value, 'dwarf, T, R>`.
            impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>> for Atom<'value, 'dwarf, $t, R>
            where
                R: crate::gimli::Reader<Offset = usize>,
            {
                type Error = crate::DowncastErr<Value<'value, 'dwarf, R>, Self>;

                fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                    if let Value::$t(value) = value {
                        Ok(value)
                    } else {
                        Err(crate::DowncastErr::new(value))
                    }
                }
            }

            /// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'value T`.
            impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>> for &'value $t
            where
                R: crate::gimli::Reader<Offset = usize>,
            {
                type Error = crate::DowncastErr<&'a Value<'value, 'dwarf, R>, Self>;

                fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                    if let Value::$t(atom) = value {
                        Ok(atom.value())
                    } else {
                        Err(crate::DowncastErr::new(value))
                    }
                }
            }

            /// Attempt to downcast a `Value<'value, 'dwarf, R>` into a `&'value T`.
            impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>> for &'value $t
            where
                R: crate::gimli::Reader<Offset = usize>,
            {
                type Error = crate::DowncastErr<Value<'value, 'dwarf, R>, Self>;

                fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                    if let Value::$t(atom) = value {
                        Ok(atom.value())
                    } else {
                        Err(crate::DowncastErr::new(value))
                    }
                }
            }
        )*
    }
}

#[allow(non_camel_case_types)]
type unit = ();

generate_primitive_conversions! {
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
    unit,
}
