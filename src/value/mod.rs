//! Reflections of Rust values.

use std::fmt;

mod array;
mod r#box;
mod boxed_dyn;
mod boxed_slice;
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
pub use boxed_dyn::BoxedDyn;
pub use boxed_slice::BoxedSlice;
pub use field::Field;
pub use fields::{Fields, FieldsIter};
pub use function::Function;
pub use iter::Iter;
pub use pointer::Pointer;
pub use r#box::Box;
pub use r#enum::Enum;
pub use r#struct::Struct;
pub use slice_impl::Slice;
pub use str_impl::str;
pub use variant::Variant;

use crate::schema::Shared;

pub use super::Value;

impl<'value, 'dwarf, P> Value<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// Safety: `value` absolutely must have the correct `type`.
    pub(crate) unsafe fn with_type(
        r#type: crate::schema::Type<'dwarf, P::Reader>,
        value: crate::Bytes<'value>,
        provider: &'dwarf P,
    ) -> Result<Self, crate::Error> {
        match r#type {
            crate::schema::Type::bool(schema) => schema.with_bytes(provider, value).map(Self::bool),
            crate::schema::Type::char(schema) => schema.with_bytes(provider, value).map(Self::char),
            crate::schema::Type::f32(schema) => schema.with_bytes(provider, value).map(Self::f32),
            crate::schema::Type::f64(schema) => schema.with_bytes(provider, value).map(Self::f64),
            crate::schema::Type::i8(schema) => schema.with_bytes(provider, value).map(Self::i8),
            crate::schema::Type::i16(schema) => schema.with_bytes(provider, value).map(Self::i16),
            crate::schema::Type::i32(schema) => schema.with_bytes(provider, value).map(Self::i32),
            crate::schema::Type::i64(schema) => schema.with_bytes(provider, value).map(Self::i64),
            crate::schema::Type::i128(schema) => schema.with_bytes(provider, value).map(Self::i128),
            crate::schema::Type::isize(schema) => {
                schema.with_bytes(provider, value).map(Self::isize)
            }
            crate::schema::Type::u8(schema) => schema.with_bytes(provider, value).map(Self::u8),
            crate::schema::Type::u16(schema) => schema.with_bytes(provider, value).map(Self::u16),
            crate::schema::Type::u32(schema) => schema.with_bytes(provider, value).map(Self::u32),
            crate::schema::Type::u64(schema) => schema.with_bytes(provider, value).map(Self::u64),
            crate::schema::Type::u128(schema) => schema.with_bytes(provider, value).map(Self::u128),
            crate::schema::Type::usize(schema) => {
                schema.with_bytes(provider, value).map(Self::usize)
            }
            crate::schema::Type::unit(schema) => schema.with_bytes(provider, value).map(Self::unit),
            crate::schema::Type::Slice(schema) => {
                Slice::with_schema(value, schema, provider).map(Self::Slice)
            }
            crate::schema::Type::Array(schema) => {
                Array::with_schema(value, schema, provider).map(Self::Array)
            }
            crate::schema::Type::Box(schema) => {
                Box::with_schema(value, schema, provider).map(Self::Box)
            }
            crate::schema::Type::BoxedSlice(schema) => {
                BoxedSlice::with_schema(value, schema, provider).map(Self::BoxedSlice)
            }
            crate::schema::Type::BoxedDyn(schema) => {
                BoxedDyn::with_schema(value, schema, provider).map(Self::BoxedDyn)
            }
            crate::schema::Type::str(schema) => {
                str::with_schema(value, schema, provider).map(Self::str)
            }
            crate::schema::Type::Struct(schema) => {
                Struct::with_schema(value, schema, provider).map(Self::Struct)
            }
            crate::schema::Type::Enum(schema) => {
                Enum::with_schema(value, schema, provider).map(Self::Enum)
            }
            crate::schema::Type::SharedRef(schema) => {
                Pointer::with_schema(value, schema, provider).map(Self::SharedRef)
            }
            crate::schema::Type::UniqueRef(schema) => {
                Pointer::with_schema(value, schema, provider).map(Self::UniqueRef)
            }
            crate::schema::Type::ConstPtr(schema) => {
                Pointer::with_schema(value, schema, provider).map(Self::ConstPtr)
            }
            crate::schema::Type::MutPtr(schema) => {
                Pointer::with_schema(value, schema, provider).map(Self::MutPtr)
            }
            crate::schema::Type::Function(schema) => {
                Function::with_schema(value, schema, provider).map(Self::Function)
            }
        }
    }
}

impl<'value, 'dwarf, P> fmt::Debug for Value<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
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
            Self::Box(v) => v.fmt(f),
            Self::BoxedSlice(v) => v.fmt(f),
            Self::BoxedDyn(v) => v.fmt(f),
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

impl<'value, 'dwarf, P> fmt::Display for Value<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
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
            Self::Box(v) => v.fmt(f),
            Self::BoxedSlice(v) => v.fmt(f),
            Self::BoxedDyn(v) => v.fmt(f),
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

/// Attempt to downcast a `Value<'value, 'dwarf, P>` into a `Struct<'value, 'dwarf, P>`.
impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>> for Struct<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::Struct(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, P>, Self>())
        }
    }
}

/// Upcast a `Pointer<'value, 'dwarf, Shared, R>` to a `Pointer<'value, 'dwarf, Shared, R>`.
impl<'value, 'dwarf, P> From<Pointer<'value, 'dwarf, crate::schema::Shared, P>>
    for Value<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn from(atom: Pointer<'value, 'dwarf, Shared, P>) -> Self {
        Value::SharedRef(atom)
    }
}

/// Upcast a `Pointer<'value, 'dwarf, Unique, R>` to a `Pointer<'value, 'dwarf, Unique, R>`.
impl<'value, 'dwarf, P> From<Pointer<'value, 'dwarf, crate::schema::Unique, P>>
    for Value<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn from(atom: Pointer<'value, 'dwarf, crate::schema::Unique, P>) -> Self {
        Value::UniqueRef(atom)
    }
}

/// Upcast a `Pointer<'value, 'dwarf, Const, R>` to a `Pointer<'value, 'dwarf, Const, R>`.
impl<'value, 'dwarf, P> From<Pointer<'value, 'dwarf, crate::schema::Const, P>>
    for Value<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn from(atom: Pointer<'value, 'dwarf, crate::schema::Const, P>) -> Self {
        Value::ConstPtr(atom)
    }
}

/// Upcast a `Pointer<'value, 'dwarf, Mut, R>` to a `Pointer<'value, 'dwarf, Mut, R>`.
impl<'value, 'dwarf, P> From<Pointer<'value, 'dwarf, crate::schema::Mut, P>>
    for Value<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn from(atom: Pointer<'value, 'dwarf, crate::schema::Mut, P>) -> Self {
        Value::MutPtr(atom)
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, P>` into a `&'a Pointer<'value, 'dwarf, Shared, R>`.
impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>>
    for &'a Pointer<'value, 'dwarf, crate::schema::Shared, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::SharedRef(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                &'a Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, P>` into a `&'a Pointer<'value, 'dwarf, Unique, R>`.
impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>>
    for &'a Pointer<'value, 'dwarf, crate::schema::Unique, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::UniqueRef(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                &'a Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, P>` into a `&'a Pointer<'value, 'dwarf, Const, R>`.
impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>>
    for &'a Pointer<'value, 'dwarf, crate::schema::Const, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::ConstPtr(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                &'a Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, P>` into a `&'a Pointer<'value, 'dwarf, Mut, R>`.
impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>>
    for &'a Pointer<'value, 'dwarf, crate::schema::Mut, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::MutPtr(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                &'a Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}

// ----
/// Attempt to downcast a `&'a Value<'value, 'dwarf, P>` into a `&'a Pointer<'value, 'dwarf, Shared, R>`.
impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>>
    for Pointer<'value, 'dwarf, crate::schema::Shared, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::SharedRef(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, P>, Self>())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, P>` into a `&'a Pointer<'value, 'dwarf, Unique, R>`.
impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>>
    for Pointer<'value, 'dwarf, crate::schema::Unique, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::UniqueRef(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, P>, Self>())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, P>` into a `&'a Pointer<'value, 'dwarf, Const, R>`.
impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>>
    for Pointer<'value, 'dwarf, crate::schema::Const, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::ConstPtr(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, P>, Self>())
        }
    }
}

/// Attempt to downcast a `&'a Value<'value, 'dwarf, P>` into a `&'a Pointer<'value, 'dwarf, Mut, R>`.
impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>>
    for Pointer<'value, 'dwarf, crate::schema::Mut, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::MutPtr(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, P>, Self>())
        }
    }
}

macro_rules! generate_primitive_conversions {
    ($t:ident) => {
        impl<'value, 'dwarf, P> From<$t<'value, 'dwarf, P>> for Value<'value, 'dwarf, P>
        where
            P: crate::DebugInfoProvider,
        {
            fn from(value: $t<'value, 'dwarf, P>) -> Self {
                crate::Value::$t(value)
            }
        }

        impl<'value, 'dwarf, P> From<$t<'value, 'dwarf, P>> for &'value std::primitive::$t
        where
            P: crate::DebugInfoProvider,
        {
            fn from(atom: $t<'value, 'dwarf, P>) -> Self {
                atom.value()
            }
        }

        impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>>
            for &'a $t<'value, 'dwarf, P>
        where
            P: crate::DebugInfoProvider,
        {
            type Error = crate::error::Downcast;

            fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(value)
                } else {
                    Err(crate::error::Downcast::new::<
                        &'a Value<'value, 'dwarf, P>,
                        Self,
                    >())
                }
            }
        }

        impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>> for $t<'value, 'dwarf, P>
        where
            P: crate::DebugInfoProvider,
        {
            type Error = crate::error::Downcast;

            fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(value)
                } else {
                    Err(crate::error::Downcast::new::<Value<'value, 'dwarf, P>, Self>())
                }
            }
        }

        impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>>
            for &'value std::primitive::$t
        where
            P: crate::DebugInfoProvider,
        {
            type Error = crate::error::Downcast;

            fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(value.value())
                } else {
                    Err(crate::error::Downcast::new::<
                        &'a Value<'value, 'dwarf, P>,
                        Self,
                    >())
                }
            }
        }

        impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>> for std::primitive::$t
        where
            P: crate::DebugInfoProvider,
        {
            type Error = crate::error::Downcast;

            fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(*value.value())
                } else {
                    Err(crate::error::Downcast::new::<
                        &'a Value<'value, 'dwarf, P>,
                        Self,
                    >())
                }
            }
        }

        impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>> for &'value std::primitive::$t
        where
            P: crate::DebugInfoProvider,
        {
            type Error = crate::error::Downcast;

            fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(value.value())
                } else {
                    Err(crate::error::Downcast::new::<Value<'value, 'dwarf, P>, Self>())
                }
            }
        }

        impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>> for std::primitive::$t
        where
            P: crate::DebugInfoProvider,
        {
            type Error = crate::error::Downcast;

            fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(*value.value())
                } else {
                    Err(crate::error::Downcast::new::<Value<'value, 'dwarf, P>, Self>())
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
        pub struct $t<'value, 'dwarf, P>
        where
            P: crate::DebugInfoProvider,
        {
            value: &'value std::primitive::$t,
            schema: crate::schema::$t<'dwarf, P::Reader>,
            provider: std::marker::PhantomData<P>,
        }

        impl<'dwarf, R> crate::schema::$t<'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>
        {
            unsafe fn with_bytes<'value, P>(self, provider: &'dwarf P, bytes: crate::Bytes<'value>) -> Result<$t<'value, 'dwarf, P>, crate::Error>
            where
                P: crate::DebugInfoProvider<Reader = R>,
            {
                let size = self.size() as std::primitive::usize;
                let value = &bytes[..size];
                let (&[], [value], &[]) = value.align_to() else {
                    return Err(crate::error::Kind::Other.into());
                };
                Ok($t {
                    value,
                    schema: self,
                    provider: std::marker::PhantomData,
                })
            }
        }

        impl<'value, 'dwarf, P> $t<'value, 'dwarf, P>
        where
            P: crate::DebugInfoProvider
        {
            pub fn value(&self) -> &'value std::primitive::$t {
                self.value
            }

            pub fn schema(&self) -> &crate::schema::$t<'dwarf, P::Reader> {
                &self.schema
            }
        }

        impl<'value, 'dwarf, P> std::fmt::Debug for $t<'value, 'dwarf, P>
        where
            P: crate::DebugInfoProvider
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut debug_struct = f.debug_struct(concat!("deflect::value::", stringify!($t)));
                debug_struct.field("schema", &self.schema);
                debug_struct.field("value", &self.value);
                debug_struct.finish()
            }
        }

        impl<'value, 'dwarf, P> std::fmt::Display for $t<'value, 'dwarf, P>
        where
            P: crate::DebugInfoProvider
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
pub struct unit<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    value: &'value (),
    schema: crate::schema::unit<'dwarf, P::Reader>,
}

impl<'dwarf, R> crate::schema::unit<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    unsafe fn with_bytes<'value, P>(
        self,
        provider: &'dwarf P,
        bytes: crate::Bytes<'value>,
    ) -> Result<unit<'value, 'dwarf, P>, crate::Error>
    where
        P: crate::DebugInfoProvider<Reader = R>,
    {
        let size = self.size() as std::primitive::usize;
        let value = &bytes[..size];
        let value = &*(value.as_ptr() as *const _);
        Ok(unit {
            value,
            schema: self,
        })
    }
}

impl<'value, 'dwarf, P> unit<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    pub fn value(&self) -> &'value () {
        self.value
    }

    pub fn schema(&self) -> &crate::schema::unit<'dwarf, P::Reader> {
        &self.schema
    }
}

impl<'value, 'dwarf, P> std::fmt::Debug for unit<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct(concat!("deflect::value::", stringify!($t)));
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, P> std::fmt::Display for unit<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("()")
    }
}

impl<'value, 'dwarf, P> From<unit<'value, 'dwarf, P>> for Value<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn from(value: unit<'value, 'dwarf, P>) -> Self {
        crate::Value::unit(value)
    }
}

impl<'value, 'dwarf, P> From<unit<'value, 'dwarf, P>> for &'value ()
where
    P: crate::DebugInfoProvider,
{
    fn from(atom: unit<'value, 'dwarf, P>) -> Self {
        atom.value()
    }
}

impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>> for &'a unit<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                &'a Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}

impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>> for unit<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, P>, Self>())
        }
    }
}

impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>> for &'value ()
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            Ok(value.value())
        } else {
            Err(crate::error::Downcast::new::<
                &'a Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}

impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>> for ()
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            value.value();
            Ok(())
        } else {
            Err(crate::error::Downcast::new::<
                &'a Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}

impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>> for &'value ()
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            Ok(value.value())
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, P>, Self>())
        }
    }
}

impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>> for ()
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            value.value();
            Ok(())
        } else {
            Err(crate::error::Downcast::new::<Value<'value, 'dwarf, P>, Self>())
        }
    }
}
