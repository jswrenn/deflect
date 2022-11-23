use std::fmt;

mod array;
mod atom;
mod r#enum;
mod field;
mod fields;
mod function;
mod pointer;
mod slice;
mod r#struct;
mod variant;

pub use array::Array;
pub use atom::Atom;
pub use field::Field;
pub use fields::{Fields, FieldsIter};
pub use function::Function;
pub use pointer::Pointer;
pub use r#enum::Enum;
pub use r#struct::Struct;
pub use slice::Slice;
pub use variant::Variant;

use crate::schema::Shared;

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
    Function(Function<'value, 'dwarf, R>),
    SharedRef(Pointer<'value, 'dwarf, crate::schema::Shared, R>),
    UniqueRef(Pointer<'value, 'dwarf, crate::schema::Unique, R>),
    ConstPtr(Pointer<'value, 'dwarf, crate::schema::Const, R>),
    MutPtr(Pointer<'value, 'dwarf, crate::schema::Mut, R>),
}

impl<'value, 'dwarf, R> Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Safety: `value` absolutely must have the correct `type`.
    pub(crate) unsafe fn with_type(
        r#type: crate::schema::Type<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Result<Self, crate::err::Error> {
        match r#type {
            crate::schema::Type::bool(schema) => Atom::with_schema(value, schema).map(Self::bool),
            crate::schema::Type::char(schema) => Atom::with_schema(value, schema).map(Self::char),
            crate::schema::Type::f32(schema) => Atom::with_schema(value, schema).map(Self::f32),
            crate::schema::Type::f64(schema) => Atom::with_schema(value, schema).map(Self::f64),
            crate::schema::Type::i8(schema) => Atom::with_schema(value, schema).map(Self::i8),
            crate::schema::Type::i16(schema) => Atom::with_schema(value, schema).map(Self::i16),
            crate::schema::Type::i32(schema) => Atom::with_schema(value, schema).map(Self::i32),
            crate::schema::Type::i64(schema) => Atom::with_schema(value, schema).map(Self::i64),
            crate::schema::Type::i128(schema) => Atom::with_schema(value, schema).map(Self::i128),
            crate::schema::Type::isize(schema) => Atom::with_schema(value, schema).map(Self::isize),
            crate::schema::Type::u8(schema) => Atom::with_schema(value, schema).map(Self::u8),
            crate::schema::Type::u16(schema) => Atom::with_schema(value, schema).map(Self::u16),
            crate::schema::Type::u32(schema) => Atom::with_schema(value, schema).map(Self::u32),
            crate::schema::Type::u64(schema) => Atom::with_schema(value, schema).map(Self::u64),
            crate::schema::Type::u128(schema) => Atom::with_schema(value, schema).map(Self::u128),
            crate::schema::Type::usize(schema) => Atom::with_schema(value, schema).map(Self::usize),
            crate::schema::Type::unit(schema) => Atom::with_schema(value, schema).map(Self::unit),
            crate::schema::Type::Slice(schema) => {
                Slice::with_schema(value, schema).map(Self::Slice)
            }
            crate::schema::Type::Array(schema) => {
                Array::with_schema(value, schema).map(Self::Array)
            }
            crate::schema::Type::Struct(schema) => {
                Struct::with_schema(value, schema).map(Self::Struct)
            }
            crate::schema::Type::Enum(schema) => Enum::with_schema(value, schema).map(Self::Enum),
            crate::schema::Type::SharedRef(schema) => {
                Pointer::with_schema(value, schema).map(Self::SharedRef)
            }
            crate::schema::Type::UniqueRef(schema) => {
                Pointer::with_schema(value, schema).map(Self::UniqueRef)
            }
            crate::schema::Type::ConstPtr(schema) => {
                Pointer::with_schema(value, schema).map(Self::ConstPtr)
            }
            crate::schema::Type::MutPtr(schema) => {
                Pointer::with_schema(value, schema).map(Self::MutPtr)
            }
            crate::schema::Type::Function(schema) => {
                Function::with_schema(value, schema).map(Self::Function)
            }
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
            Self::Function(v) => v.fmt(f),
            Self::SharedRef(v) => v.fmt(f),
            Self::UniqueRef(v) => v.fmt(f),
            Self::ConstPtr(v) => v.fmt(f),
            Self::MutPtr(v) => v.fmt(f),
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
            Self::Function(v) => v.fmt(f),
            Self::SharedRef(v) => v.fmt(f),
            Self::UniqueRef(v) => v.fmt(f),
            Self::ConstPtr(v) => v.fmt(f),
            Self::MutPtr(v) => v.fmt(f),
        }
    }
}

/// Upcast a `Pointer<'value, 'dwarf, Shared, R>` to a `Pointer<'value, 'dwarf, Shared, R>`.
impl<'value, 'dwarf, R> From<Pointer<'value, 'dwarf, crate::schema::Shared, R>>
    for Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn from(atom: Pointer<'value, 'dwarf, Shared, R>) -> Self {
        Value::SharedRef(atom)
    }
}

/// Upcast a `Pointer<'value, 'dwarf, Unique, R>` to a `Pointer<'value, 'dwarf, Unique, R>`.
impl<'value, 'dwarf, R> From<Pointer<'value, 'dwarf, crate::schema::Unique, R>>
    for Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn from(atom: Pointer<'value, 'dwarf, crate::schema::Unique, R>) -> Self {
        Value::UniqueRef(atom)
    }
}

/// Upcast a `Pointer<'value, 'dwarf, Const, R>` to a `Pointer<'value, 'dwarf, Const, R>`.
impl<'value, 'dwarf, R> From<Pointer<'value, 'dwarf, crate::schema::Const, R>>
    for Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn from(atom: Pointer<'value, 'dwarf, crate::schema::Const, R>) -> Self {
        Value::ConstPtr(atom)
    }
}

/// Upcast a `Pointer<'value, 'dwarf, Mut, R>` to a `Pointer<'value, 'dwarf, Mut, R>`.
impl<'value, 'dwarf, R> From<Pointer<'value, 'dwarf, crate::schema::Mut, R>>
    for Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn from(atom: Pointer<'value, 'dwarf, crate::schema::Mut, R>) -> Self {
        Value::MutPtr(atom)
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Pointer<'value, 'dwarf, Shared, R>`.
impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>>
    for &'a Pointer<'value, 'dwarf, crate::schema::Shared, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Error = crate::err::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::SharedRef(value) = value {
            Ok(value)
        } else {
            Err(crate::err::Downcast::new::<
                &'a Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Pointer<'value, 'dwarf, Unique, R>`.
impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>>
    for &'a Pointer<'value, 'dwarf, crate::schema::Unique, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Error = crate::err::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::UniqueRef(value) = value {
            Ok(value)
        } else {
            Err(crate::err::Downcast::new::<
                &'a Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Pointer<'value, 'dwarf, Const, R>`.
impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>>
    for &'a Pointer<'value, 'dwarf, crate::schema::Const, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Error = crate::err::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::ConstPtr(value) = value {
            Ok(value)
        } else {
            Err(crate::err::Downcast::new::<
                &'a Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Pointer<'value, 'dwarf, Mut, R>`.
impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>>
    for &'a Pointer<'value, 'dwarf, crate::schema::Mut, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Error = crate::err::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::MutPtr(value) = value {
            Ok(value)
        } else {
            Err(crate::err::Downcast::new::<
                &'a Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}

// ----
/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Pointer<'value, 'dwarf, Shared, R>`.
impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>>
    for Pointer<'value, 'dwarf, crate::schema::Shared, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Error = crate::err::Downcast;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::SharedRef(value) = value {
            Ok(value)
        } else {
            Err(crate::err::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Pointer<'value, 'dwarf, Unique, R>`.
impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>>
    for Pointer<'value, 'dwarf, crate::schema::Unique, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Error = crate::err::Downcast;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::UniqueRef(value) = value {
            Ok(value)
        } else {
            Err(crate::err::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Pointer<'value, 'dwarf, Const, R>`.
impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>>
    for Pointer<'value, 'dwarf, crate::schema::Const, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Error = crate::err::Downcast;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::ConstPtr(value) = value {
            Ok(value)
        } else {
            Err(crate::err::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Pointer<'value, 'dwarf, Mut, R>`.
impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>>
    for Pointer<'value, 'dwarf, crate::schema::Mut, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Error = crate::err::Downcast;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::MutPtr(value) = value {
            Ok(value)
        } else {
            Err(crate::err::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
        }
    }
}

// ----

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
                type Error = crate::err::Downcast;

                fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                    if let Value::$t(value) = value {
                        Ok(value)
                    } else {
                        Err(crate::err::Downcast::new::<&'a Value<'value, 'dwarf, R>, Self>())
                    }
                }
            }

            /// Attempt to downcast a `Value<'value, 'dwarf, R>` into a `Atom<'value, 'dwarf, T, R>`.
            impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>> for Atom<'value, 'dwarf, $t, R>
            where
                R: crate::gimli::Reader<Offset = usize>,
            {
                type Error = crate::err::Downcast;

                fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                    if let Value::$t(value) = value {
                        Ok(value)
                    } else {
                        Err(crate::err::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
                    }
                }
            }

            /// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'value T`.
            impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>> for &'value $t
            where
                R: crate::gimli::Reader<Offset = usize>,
            {
                type Error = crate::err::Downcast;

                fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                    if let Value::$t(atom) = value {
                        Ok(atom.value())
                    } else {
                        Err(crate::err::Downcast::new::<&'a Value<'value, 'dwarf, R>, Self>())
                    }
                }
            }

            /// Attempt to downcast a `Value<'value, 'dwarf, R>` into a `&'value T`.
            impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>> for &'value $t
            where
                R: crate::gimli::Reader<Offset = usize>,
            {
                type Error = crate::err::Downcast;

                fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                    if let Value::$t(atom) = value {
                        Ok(atom.value())
                    } else {
                        Err(crate::err::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
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
