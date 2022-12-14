use std::fmt;

/// A reflected pointer or reference.
pub struct Pointer<'value, 'dwarf, K, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    schema: crate::schema::Pointer<'dwarf, K, P::Reader>,
    value: crate::Bytes<'value>,
    provider: &'dwarf P,
}

impl<'dwarf, K, R> crate::schema::Pointer<'dwarf, K, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) unsafe fn with_bytes<'value, P>(
        self,
        provider: &'dwarf P,
        value: crate::Bytes<'value>,
    ) -> Result<Pointer<'value, 'dwarf, K, P>, crate::Error>
    where
        P: crate::DebugInfoProvider<Reader = R>,
    {
        Ok(Pointer {
            schema: self,
            value,
            provider,
        })
    }
}

impl<'value, 'dwarf, K, P> Pointer<'value, 'dwarf, K, P>
where
    K: crate::schema::Reference,
    P: crate::DebugInfoProvider,
{
    /// The reflected value behind this reference.
    pub fn deref(&self) -> Result<super::Value<'value, 'dwarf, P>, crate::Error>
    where
        K: crate::schema::Reference,
    {
        let value = unsafe { *(self.value.as_ptr() as *const *const crate::Byte) };
        let r#type = self.schema.r#type()?;
        let size = r#type.size()?;
        let size = size.try_into()?;
        let value = std::ptr::slice_from_raw_parts(value, size);
        let value = unsafe { &*value };
        unsafe { super::Value::with_type(r#type, value, self.provider) }
    }
}

impl<'value, 'dwarf, K, P> Pointer<'value, 'dwarf, K, P>
where
    P: crate::DebugInfoProvider,
{
    /// The schema of this value.
    pub fn schema(&self) -> &crate::schema::Pointer<'dwarf, K, P::Reader> {
        &self.schema
    }

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

    /// The unreflected value behind this reference.
    pub(crate) fn deref_raw_dyn(&self, size: usize) -> Result<crate::Bytes<'value>, crate::Error> {
        let value = unsafe { *(self.value.as_ptr() as *const *const crate::Byte) };
        let value = std::ptr::slice_from_raw_parts(value, size);
        let value = unsafe { &*value };
        Ok(value)
    }
}

impl<'value, 'dwarf, K, P> fmt::Debug for Pointer<'value, 'dwarf, K, P>
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

impl<'value, 'dwarf, P> fmt::Display for Pointer<'value, 'dwarf, crate::schema::Shared, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("&")?;
        self.deref().map_err(crate::fmt_err)?.fmt(f)
    }
}

impl<'value, 'dwarf, P> fmt::Display for Pointer<'value, 'dwarf, crate::schema::Unique, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("&mut ")?;
        self.deref().map_err(crate::fmt_err)?.fmt(f)
    }
}

impl<'value, 'dwarf, P> fmt::Display for Pointer<'value, 'dwarf, crate::schema::Const, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.deref_raw().map_err(crate::fmt_err)?;
        let addr = value.as_ptr() as usize;
        addr.fmt(f)?;
        f.write_str(" as *const _")
    }
}

impl<'value, 'dwarf, P> fmt::Display for Pointer<'value, 'dwarf, crate::schema::Mut, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.deref_raw().map_err(crate::fmt_err)?;
        let addr = value.as_ptr() as usize;
        addr.fmt(f)?;
        f.write_str(" as *mut _")
    }
}

