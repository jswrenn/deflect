use std::{fmt, ops};

/// A function value.
pub struct Function<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    value: crate::Bytes<'value>,
    schema: crate::schema::Function<'dwarf, R>,
}

impl<'value, 'dwarf, R> Function<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::Function<'dwarf, R>,
    ) -> Result<Self, crate::error::Error> {
        Ok(Self { value, schema })
    }
}

impl<'value, 'dwarf, R> fmt::Display for Function<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.schema.fmt(f)
    }
}

impl<'value, 'dwarf, R> ops::Deref for Function<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Function<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
