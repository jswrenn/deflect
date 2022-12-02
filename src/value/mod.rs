//! Reflections of Rust values.

use std::fmt;

mod array;
mod r#enum;
mod field;
mod fields;
mod function;
mod iter;
mod pointer;
mod slice_impl;
mod str_impl;
mod r#struct;
mod variant;

pub use array::Array;
pub use field::Field;
pub use fields::{Fields, FieldsIter};
pub use function::Function;
pub use iter::Iter;
pub use pointer::Pointer;
pub use r#enum::Enum;
pub use r#struct::Struct;
pub use slice_impl::Slice;
pub use str_impl::str;
pub use variant::Variant;

use crate::schema::Shared;

pub use super::Value;

impl<'value, 'dwarf, R> Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    /// Safety: `value` absolutely must have the correct `type`.
    pub(crate) unsafe fn with_type(
        r#type: crate::schema::Type<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Result<Self, crate::Error> {
        match r#type {
            crate::schema::Type::bool(schema) => schema.with_bytes(value).map(Self::bool),
            crate::schema::Type::char(schema) => schema.with_bytes(value).map(Self::char),
            crate::schema::Type::f32(schema) => schema.with_bytes(value).map(Self::f32),
            crate::schema::Type::f64(schema) => schema.with_bytes(value).map(Self::f64),
            crate::schema::Type::i8(schema) => schema.with_bytes(value).map(Self::i8),
            crate::schema::Type::i16(schema) => schema.with_bytes(value).map(Self::i16),
            crate::schema::Type::i32(schema) => schema.with_bytes(value).map(Self::i32),
            crate::schema::Type::i64(schema) => schema.with_bytes(value).map(Self::i64),
            crate::schema::Type::i128(schema) => schema.with_bytes(value).map(Self::i128),
            crate::schema::Type::isize(schema) => schema.with_bytes(value).map(Self::isize),
            crate::schema::Type::u8(schema) => schema.with_bytes(value).map(Self::u8),
            crate::schema::Type::u16(schema) => schema.with_bytes(value).map(Self::u16),
            crate::schema::Type::u32(schema) => schema.with_bytes(value).map(Self::u32),
            crate::schema::Type::u64(schema) => schema.with_bytes(value).map(Self::u64),
            crate::schema::Type::u128(schema) => schema.with_bytes(value).map(Self::u128),
            crate::schema::Type::usize(schema) => schema.with_bytes(value).map(Self::usize),
            crate::schema::Type::unit(schema) => schema.with_bytes(value).map(Self::unit),
            crate::schema::Type::Slice(schema) => {
                Slice::with_schema(value, schema).map(Self::Slice)
            }
            crate::schema::Type::Array(schema) => {
                Array::with_schema(value, schema).map(Self::Array)
            }
            crate::schema::Type::str(schema) => str::with_schema(value, schema).map(Self::str),
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
            Self::Array(v) => v.fmt(f),
            Self::Slice(v) => v.fmt(f),
            Self::str(v) => v.fmt(f),
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
            Self::unit(_) => f.write_str("()"),
            Self::Slice(v) => v.fmt(f),
            Self::Array(v) => v.fmt(f),
            Self::str(v) => v.fmt(f),
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

/// Attempt to downcast a `Value<'value, 'dwarf, R>` into a `Struct<'value, 'dwarf, R>`.
impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>> for Struct<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::Struct(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
        }
    }
}

/// Upcast a `Pointer<'value, 'dwarf, Shared, R>` to a `Pointer<'value, 'dwarf, Shared, R>`.
impl<'value, 'dwarf, R> From<Pointer<'value, 'dwarf, crate::schema::Shared, R>>
    for Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn from(atom: Pointer<'value, 'dwarf, Shared, R>) -> Self {
        Value::SharedRef(atom)
    }
}

/// Upcast a `Pointer<'value, 'dwarf, Unique, R>` to a `Pointer<'value, 'dwarf, Unique, R>`.
impl<'value, 'dwarf, R> From<Pointer<'value, 'dwarf, crate::schema::Unique, R>>
    for Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn from(atom: Pointer<'value, 'dwarf, crate::schema::Unique, R>) -> Self {
        Value::UniqueRef(atom)
    }
}

/// Upcast a `Pointer<'value, 'dwarf, Const, R>` to a `Pointer<'value, 'dwarf, Const, R>`.
impl<'value, 'dwarf, R> From<Pointer<'value, 'dwarf, crate::schema::Const, R>>
    for Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn from(atom: Pointer<'value, 'dwarf, crate::schema::Const, R>) -> Self {
        Value::ConstPtr(atom)
    }
}

/// Upcast a `Pointer<'value, 'dwarf, Mut, R>` to a `Pointer<'value, 'dwarf, Mut, R>`.
impl<'value, 'dwarf, R> From<Pointer<'value, 'dwarf, crate::schema::Mut, R>>
    for Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn from(atom: Pointer<'value, 'dwarf, crate::schema::Mut, R>) -> Self {
        Value::MutPtr(atom)
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Pointer<'value, 'dwarf, Shared, R>`.
impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>>
    for &'a Pointer<'value, 'dwarf, crate::schema::Shared, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::SharedRef(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
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
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::UniqueRef(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
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
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::ConstPtr(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
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
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::MutPtr(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
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
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::SharedRef(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Pointer<'value, 'dwarf, Unique, R>`.
impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>>
    for Pointer<'value, 'dwarf, crate::schema::Unique, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::UniqueRef(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Pointer<'value, 'dwarf, Const, R>`.
impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>>
    for Pointer<'value, 'dwarf, crate::schema::Const, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::ConstPtr(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, R>` into a `&'a Pointer<'value, 'dwarf, Mut, R>`.
impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>>
    for Pointer<'value, 'dwarf, crate::schema::Mut, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::MutPtr(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
        }
    }
}

macro_rules! generate_primitive_conversions {
    ($t:ident) => {
        impl<'value, 'dwarf, R> From<$t<'value, 'dwarf, R>> for Value<'value, 'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            fn from(value: $t<'value, 'dwarf, R>) -> Self {
                crate::Value::$t(value)
            }
        }

        impl<'value, 'dwarf, R> From<$t<'value, 'dwarf, R>> for &'value std::primitive::$t
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            fn from(atom: $t<'value, 'dwarf, R>) -> Self {
                atom.value()
            }
        }

        impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>>
            for &'a $t<'value, 'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            type Error = crate::error::Downcast;

            fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(value)
                } else {
                    Err(crate::error::Downcast::new::<
                        &'a Value<'value, 'dwarf, R>,
                        Self,
                    >())
                }
            }
        }

        impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>> for $t<'value, 'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            type Error = crate::error::Downcast;

            fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(value)
                } else {
                    Err(crate::error::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
                }
            }
        }

        impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>>
            for &'value std::primitive::$t
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            type Error = crate::error::Downcast;

            fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(value.value())
                } else {
                    Err(crate::error::Downcast::new::<
                        &'a Value<'value, 'dwarf, R>,
                        Self,
                    >())
                }
            }
        }

        impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>> for std::primitive::$t
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            type Error = crate::error::Downcast;

            fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(*value.value())
                } else {
                    Err(crate::error::Downcast::new::<
                        &'a Value<'value, 'dwarf, R>,
                        Self,
                    >())
                }
            }
        }

        impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>> for &'value std::primitive::$t
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            type Error = crate::error::Downcast;

            fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(value.value())
                } else {
                    Err(crate::error::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
                }
            }
        }

        impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>> for std::primitive::$t
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            type Error = crate::error::Downcast;

            fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(*value.value())
                } else {
                    Err(crate::error::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
                }
            }
        }
    };
}

macro_rules! generate_primitive {
    ($($t:ident,)*) => {
        $(
            generate_primitive!(@
                $t,
                concat!(
                    "A reflected [`",
                    stringify!($t),
                    "`][std::primitive::",
                    stringify!($t),
                    "] value."
                )
            );
        )*
    };
    (@ $t:ident, $doc:expr) => {
        #[doc = $doc]
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct $t<'value, 'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            value: &'value std::primitive::$t,
            schema: crate::schema::$t<'dwarf, R>,
        }

        impl<'dwarf, R> crate::schema::$t<'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            unsafe fn with_bytes<'value>(self, bytes: crate::Bytes<'value>) -> Result<$t<'value, 'dwarf, R>, crate::Error> {
                let size = self.size() as std::primitive::usize;
                let value = &bytes[..size];
                let (&[], [value], &[]) = value.align_to() else { panic!() };
                Ok($t {
                    value,
                    schema: self,
                })
            }
        }

        impl<'value, 'dwarf, R> $t<'value, 'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            pub(crate) unsafe fn with_schema(
                value: crate::Bytes<'value>,
                schema: crate::schema::$t<'dwarf, R>,
            ) -> Result<Self, crate::Error> {
                let size = schema.size() as std::primitive::usize;
                let value = &value[..size];
                let (&[], [value], &[]) = value.align_to() else { panic!() };
                Ok(Self { schema, value })
            }

            pub fn value(&self) -> &'value std::primitive::$t {
                self.value
            }

            pub fn schema(&self) -> &crate::schema::$t<'dwarf, R> {
                &self.schema
            }
        }

        impl<'value, 'dwarf, R> std::fmt::Debug for $t<'value, 'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut debug_struct = f.debug_struct(concat!("deflect::value::", stringify!($t)));
                debug_struct.field("schema", &self.schema);
                debug_struct.field("value", &self.value);
                debug_struct.finish()
            }
        }

        impl<'value, 'dwarf, R> std::fmt::Display for $t<'value, 'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.value.fmt(f)
            }
        }

        generate_primitive_conversions!($t);
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

/// A reflected [`()`][prim@unit] value.
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct unit<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    value: &'value (),
    schema: crate::schema::unit<'dwarf, R>,
}

impl<'dwarf, R> crate::schema::unit<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    unsafe fn with_bytes<'value>(
        self,
        bytes: crate::Bytes<'value>,
    ) -> Result<unit<'value, 'dwarf, R>, crate::Error> {
        let size = self.size() as std::primitive::usize;
        let value = &bytes[..size];
        let value = &*(value.as_ptr() as *const _);
        Ok(unit {
            value,
            schema: self,
        })
    }
}

impl<'value, 'dwarf, R> unit<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::unit<'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        let size = schema.size() as std::primitive::usize;
        let value = &value[..size];
        let value = &*(value.as_ptr() as *const _);
        Ok(Self { schema, value })
    }

    pub fn value(&self) -> &'value () {
        self.value
    }

    pub fn schema(&self) -> &crate::schema::unit<'dwarf, R> {
        &self.schema
    }
}

impl<'value, 'dwarf, R> std::fmt::Debug for unit<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct(concat!("deflect::value::", stringify!($t)));
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, R> std::fmt::Display for unit<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("()")
    }
}

impl<'value, 'dwarf, R> From<unit<'value, 'dwarf, R>> for Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn from(value: unit<'value, 'dwarf, R>) -> Self {
        crate::Value::unit(value)
    }
}

impl<'value, 'dwarf, R> From<unit<'value, 'dwarf, R>> for &'value ()
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn from(atom: unit<'value, 'dwarf, R>) -> Self {
        atom.value()
    }
}

impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>> for &'a unit<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                &'a Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}

impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>> for unit<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
        }
    }
}

impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>> for &'value ()
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            Ok(value.value())
        } else {
            Err(crate::error::Downcast::new::<
                &'a Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}

impl<'a, 'value, 'dwarf, R> TryFrom<&'a Value<'value, 'dwarf, R>> for ()
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            Ok(*value.value())
        } else {
            Err(crate::error::Downcast::new::<
                &'a Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}

impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>> for &'value ()
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            Ok(value.value())
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
        }
    }
}

impl<'value, 'dwarf, R> TryFrom<Value<'value, 'dwarf, R>> for ()
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            Ok(*value.value())
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, R>, Self>())
        }
    }
}
