use std::fmt;

/// A reflected [`Box`] value.
pub struct Box<'value, 'dwarf, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    schema: crate::schema::Box<'dwarf, P::Reader>,
    value: crate::Bytes<'value>,
    provider: &'dwarf P,
}

impl<'dwarf, R> crate::schema::Box<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) unsafe fn with_bytes<'value, P>(
        self,
        provider: &'dwarf P,
        value: crate::Bytes<'value>,
    ) -> Result<Box<'value, 'dwarf, P>, crate::Error>
    where
        P: crate::DebugInfoProvider<Reader = R>,
    {
        Ok(Box {
            schema: self,
            value,
            provider,
        })
    }
}

impl<'value, 'dwarf, P> Box<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// The schema of this value.
    pub fn schema(&self) -> &crate::schema::Box<'dwarf, P::Reader> {
        &self.schema
    }

    /// The reflected value behind this reference.
    pub fn deref(&self) -> Result<super::Value<'value, 'dwarf, P>, crate::Error> {
        let value = unsafe { *(self.value.as_ptr() as *const *const crate::Byte) };
        let r#type = self.schema.r#type()?;
        let size = r#type.size()?;
        let size = size.try_into()?;
        let value = std::ptr::slice_from_raw_parts(value, size);
        let value = unsafe { &*value };
        unsafe { super::Value::with_type(r#type, value, self.provider) }
    }
}

/*
impl<'value, 'dwarf, P> Box<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// The unreflected value behind this reference.
    pub(crate) fn deref_raw(&self) -> Result<crate::Bytes<'value>, crate::Error> {
        let value = unsafe { *(self.value.as_ptr() as *const *const crate::Byte) };
        let r#type = self.schema.r#type()?;
        let size = r#type.size()?;
        let size = size.try_into()?;
        let value = std::ptr::slice_from_raw_parts(value, size);
        let value = unsafe { &*value };
        Ok(value)
    }
}*/

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

impl<'value, 'dwarf, P> serde::Serialize for Box<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = self.deref().map_err(crate::ser_err)?;
        value.serialize(serializer)
    }
}
