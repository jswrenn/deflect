use std::{fmt, ops};

/// A field of a [struct][super::Struct] or [variant][super::Variant].
pub struct Field<'value, 'dwarf, P: crate::DebugInfoProvider>
where
    P: crate::DebugInfoProvider,
{
    schema: crate::schema::Field<'dwarf, P::Reader>,
    value: crate::Bytes<'value>,
    provider: &'dwarf P,
}

impl<'value, 'dwarf, P> Field<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Field<'dwarf, P::Reader>,
        value: crate::Bytes<'value>,
        provider: &'dwarf P,
    ) -> Self {
        Self {
            schema,
            value,
            provider,
        }
    }

    /// The value of this field.
    pub fn value(&self) -> Result<super::Value<'value, 'dwarf, P>, crate::Error> {
        let r#type = self.r#type()?;
        let offset = self.offset()?.address(0)? as usize;
        let value = &self.value[offset..];
        unsafe { super::Value::with_type(r#type, value, self.provider) }
    }
}

impl<'value, 'dwarf, P> fmt::Display for Field<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name().map_err(crate::fmt_err)?.fmt(f)?;
        f.write_str(" : ")?;
        self.value().map_err(crate::fmt_err)?.fmt(f)
    }
}

impl<'value, 'dwarf, P> ops::Deref for Field<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Target = crate::schema::Field<'dwarf, P::Reader>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
