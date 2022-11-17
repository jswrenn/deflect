use std::{fmt, ops};

/// A function value.
pub struct Function<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Function<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'value, 'dwarf, R> Function<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Function<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { schema, value }
    }
}

impl<'value, 'dwarf, R> fmt::Display for Function<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Can we get back function names?
        if let Some(name) = self.name().expect("gimli err") {
            let name = name.to_string_lossy().expect("gimli err");
            write!(f, "fn {name}<todo>(todo) -> (todo)")
        } else {
            write!(f, "fn ()<todo>(todo) -> (todo)")
        }
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
