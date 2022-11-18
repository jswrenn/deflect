use super::Value;
use crate::DowncastErr;
use std::{fmt, marker::PhantomData, ops};

/// A primitive, non-compound value, like `u8` or `bool`.
pub struct Atom<'value, 'dwarf, T, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    value: &'value T,
    schema: crate::schema::Atom<'dwarf, T, R>,
}

impl<'value, 'dwarf, T, R> Atom<'value, 'dwarf, T, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Construct an `Atom`.
    pub(crate) unsafe fn new(
        value: crate::Bytes<'value>,
        schema: crate::schema::Atom<'dwarf, T, R>,
    ) -> Self {
        let value = &value[..std::mem::size_of::<T>()];
        let (&[], &[ref value], &[]) = value.align_to() else { panic!() };
        Self { schema, value }
    }

    pub fn value(&self) -> &'value T {
        self.value
    }

    pub fn schema(&self) -> &crate::schema::Atom<'dwarf, T, R> {
        &self.schema
    }
}

impl<'value, 'dwarf, T, R> fmt::Debug for Atom<'value, 'dwarf, T, R>
where
    T: fmt::Debug,
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::Atom");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, T, R> fmt::Display for Atom<'value, 'dwarf, T, R>
where
    T: fmt::Display,
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.value, f)
    }
}

impl<'value, 'dwarf, T, R> ops::Deref for Atom<'value, 'dwarf, T, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = T;

    fn deref(&self) -> &'value Self::Target {
        &self.value
    }
}
