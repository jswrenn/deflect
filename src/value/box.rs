use std::{fmt, ops};

pub struct Box<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    schema: crate::schema::Box<'dwarf, P::Reader>,
    value: crate::Bytes<'value>,
    provider: &'dwarf P,
}

impl<'value, 'dwarf, P> Box<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::Box<'dwarf, P::Reader>,
        provider: &'dwarf P,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            schema,
            value,
            provider,
        })
    }
}

impl<'value, 'dwarf, P> Box<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// The reflected value behind this reference.
    pub fn deref(&self) -> Result<super::Value<'value, 'dwarf, P>, crate::Error> {
        let value = unsafe { *(self.value.as_ptr() as *const *const crate::Byte) };
        let r#type = self.schema.r#type()?;
        let size = r#type.size()?;
        let size = size.try_into().map_err(crate::error::Kind::TryFromInt)?;
        let value = std::ptr::slice_from_raw_parts(value, size);
        let value = unsafe { &*value };
        unsafe { super::Value::with_type(r#type, value, self.provider) }
    }
}

impl<'value, 'dwarf, P> Box<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// The unreflected value behind this reference.
    pub(crate) fn deref_raw(&self) -> Result<crate::Bytes<'value>, crate::Error> {
        let value = unsafe { *(self.value.as_ptr() as *const *const crate::Byte) };
        let r#type = self.schema.r#type()?;
        let size = r#type.size()?;
        let size = size.try_into().map_err(crate::error::Kind::TryFromInt)?;
        let value = std::ptr::slice_from_raw_parts(value, size);
        let value = unsafe { &*value };
        Ok(value)
    }
}

impl<'value, 'dwarf, P> fmt::Debug for Box<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::Ref");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, P> fmt::Display for Box<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("box ")?;
        self.deref().map_err(crate::fmt_err)?.fmt(f)
    }
}

impl<'value, 'dwarf, P> ops::Deref for Box<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Target = crate::schema::Box<'dwarf, P::Reader>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
