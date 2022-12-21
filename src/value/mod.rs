//! Reflections of Rust values.

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

/// A reflected shared reference value.
pub type SharedRef<'value, 'dwarf, P = crate::DefaultProvider> =
    crate::value::Pointer<'value, 'dwarf, crate::schema::Shared, P>;

/// A reflected unique reference value.
pub type UniqueRef<'value, 'dwarf, P = crate::DefaultProvider> =
    crate::value::Pointer<'value, 'dwarf, crate::schema::Unique, P>;

/// A reflected `const` pointer value.
pub type ConstPtr<'value, 'dwarf, P = crate::DefaultProvider> =
    crate::value::Pointer<'value, 'dwarf, crate::schema::Const, P>;

/// A reflected `mut` pointer value.
pub type MutPtr<'value, 'dwarf, P = crate::DefaultProvider> =
    crate::value::Pointer<'value, 'dwarf, crate::schema::Mut, P>;

pub use super::Value;

macro_rules! generate_primitive_conversions {
    ($t:ident) => {
        impl<'value, 'dwarf, P> From<$t<'value, 'dwarf, P>> for &'value std::primitive::$t
        where
            P: crate::DebugInfoProvider,
        {
            fn from(atom: $t<'value, 'dwarf, P>) -> Self {
                atom.value()
            }
        }

        impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>>
            for &'value std::primitive::$t
        where
            P: crate::DebugInfoProvider,
        {
            type Error = crate::DowncastErr;

            fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(value.value())
                } else {
                    Err(crate::DowncastErr::new::<&'a Value<'value, 'dwarf, P>, Self>())
                }
            }
        }

        impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>> for std::primitive::$t
        where
            P: crate::DebugInfoProvider,
        {
            type Error = crate::DowncastErr;

            fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(*value.value())
                } else {
                    Err(crate::DowncastErr::new::<&'a Value<'value, 'dwarf, P>, Self>())
                }
            }
        }

        impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>> for &'value std::primitive::$t
        where
            P: crate::DebugInfoProvider,
        {
            type Error = crate::DowncastErr;

            fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(value.value())
                } else {
                    Err(crate::DowncastErr::new::<Value<'value, 'dwarf, P>, Self>())
                }
            }
        }

        impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>> for std::primitive::$t
        where
            P: crate::DebugInfoProvider,
        {
            type Error = crate::DowncastErr;

            fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
                if let Value::$t(value) = value {
                    Ok(*value.value())
                } else {
                    Err(crate::DowncastErr::new::<Value<'value, 'dwarf, P>, Self>())
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
        pub struct $t<'value, 'dwarf, P = crate::DefaultProvider>
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
            pub(crate) unsafe fn with_bytes<'value, P>(self, _provider: &'dwarf P, bytes: crate::Bytes<'value>) -> Result<$t<'value, 'dwarf, P>, crate::Error>
            where
                P: crate::DebugInfoProvider<Reader = R>,
            {
                let size = self.size() as std::primitive::usize;
                let value = &bytes[..size];
                let (&[], [value], &[]) = value.align_to() else {
                    bail!("primitive is misaligned")
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
            /// The schema of this value.
            pub fn schema(&self) -> &crate::schema::$t<'dwarf, P::Reader> {
                &self.schema
            }

            /// The rust value of this reflected value.
            pub fn value(&self) -> &'value std::primitive::$t {
                self.value
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

        impl<'value, 'dwarf, P> serde::Serialize for $t<'value, 'dwarf, P>
        where
            P: crate::DebugInfoProvider,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.value().serialize(serializer)
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
pub struct unit<'value, 'dwarf, P = crate::DefaultProvider>
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
    pub(crate) unsafe fn with_bytes<'value, P>(
        self,
        _provider: &'dwarf P,
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
    /// The Rust value of this reflected value.
    pub fn value(&self) -> &'value () {
        self.value
    }

    /// The schema of this reflected value.
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

impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>> for &'value ()
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::DowncastErr;

    fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            Ok(value.value())
        } else {
            Err(crate::DowncastErr::new::<&'a Value<'value, 'dwarf, P>, Self>())
        }
    }
}

impl<'a, 'value, 'dwarf, P> TryFrom<&'a Value<'value, 'dwarf, P>> for ()
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::DowncastErr;

    fn try_from(value: &'a Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            value.value();
            Ok(())
        } else {
            Err(crate::DowncastErr::new::<&'a Value<'value, 'dwarf, P>, Self>())
        }
    }
}

impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>> for &'value ()
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::DowncastErr;

    fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            Ok(value.value())
        } else {
            Err(crate::DowncastErr::new::<Value<'value, 'dwarf, P>, Self>())
        }
    }
}

impl<'value, 'dwarf, P> TryFrom<Value<'value, 'dwarf, P>> for ()
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::DowncastErr;

    fn try_from(value: Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let Value::unit(value) = value {
            value.value();
            Ok(())
        } else {
            Err(crate::DowncastErr::new::<Value<'value, 'dwarf, P>, Self>())
        }
    }
}

impl<'value, 'dwarf, P> serde::Serialize for unit<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value().serialize(serializer)
    }
}
