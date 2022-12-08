use std::{fmt, ops};

/// A function value.
pub struct Function<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    _value: crate::Bytes<'value>,
    schema: crate::schema::Function<'dwarf, P::Reader>,
    provider: &'dwarf P,
}

impl<'value, 'dwarf, P> Function<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::Function<'dwarf, P::Reader>,
        provider: &'dwarf P,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            _value: value,
            schema,
            provider,
        })
    }
}

impl<'value, 'dwarf, P> fmt::Display for Function<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.schema.fmt(f)
    }
}

impl<'value, 'dwarf, P> ops::Deref for Function<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Target = crate::schema::Function<'dwarf, P::Reader>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
