use std::fmt;

/// A function value.
pub struct Function<'value, 'dwarf, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    _value: crate::Bytes<'value>,
    schema: crate::schema::Function<'dwarf, P::Reader>,
    _provider: &'dwarf P,
}

impl<'dwarf, R> crate::schema::Function<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) unsafe fn with_bytes<'value, P>(
        self,
        provider: &'dwarf P,
        value: crate::Bytes<'value>,
    ) -> Result<Function<'value, 'dwarf, P>, crate::Error>
    where
        P: crate::DebugInfoProvider<Reader = R>,
    {
        Ok(Function {
            schema: self,
            _value: value,
            _provider: provider,
        })
    }
}

impl<'value, 'dwarf, P> Function<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// The schema of this value.
    pub fn schema(&self) -> &crate::schema::Function<'dwarf, P::Reader> {
        &self.schema
    }
}

impl<'value, 'dwarf, P> fmt::Debug for Function<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("fn(){}")
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

impl<'value, 'dwarf, P> serde::Serialize for Function<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Err(crate::ser_err("cannot serialize function"))
    }
}
