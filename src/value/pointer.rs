use std::{fmt, ops};

pub struct Pointer<'value, 'dwarf, K, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Pointer<'dwarf, K, R>,
    value: crate::Bytes<'value>,
}

impl<'value, 'dwarf, K, R> Pointer<'value, 'dwarf, K, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::Pointer<'dwarf, K, R>,
    ) -> Result<Self, crate::Error> {
        Ok(Self { schema, value })
    }
}

impl<'value, 'dwarf, K, R> Pointer<'value, 'dwarf, K, R>
where
    K: crate::schema::Reference,
    R: crate::gimli::Reader<Offset = usize>,
{
    /// The reflected value behind this reference.
    pub fn deref(&self) -> Result<super::Value<'value, 'dwarf, R>, crate::Error>
    where
        K: crate::schema::Reference,
    {
        let value = unsafe { *(self.value.as_ptr() as *const *const crate::Byte) };
        let r#type = self.schema.r#type()?;
        let size = r#type.size()?;
        let size = size.try_into().map_err(crate::error::Kind::TryFromInt)?;
        let value = std::ptr::slice_from_raw_parts(value, size);
        let value = unsafe { &*value };
        unsafe { super::Value::with_type(r#type, value) }
    }
}

impl<'value, 'dwarf, K, R> Pointer<'value, 'dwarf, K, R>
where
    R: crate::gimli::Reader<Offset = usize>,
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

impl<'value, 'dwarf, K, R> fmt::Debug for Pointer<'value, 'dwarf, K, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::Ref");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, R> fmt::Display for Pointer<'value, 'dwarf, crate::schema::Shared, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("&")?;
        self.deref().unwrap().fmt(f)
    }
}

impl<'value, 'dwarf, R> fmt::Display for Pointer<'value, 'dwarf, crate::schema::Unique, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("&mut ")?;
        self.deref().unwrap().fmt(f)
    }
}

impl<'value, 'dwarf, R> fmt::Display for Pointer<'value, 'dwarf, crate::schema::Const, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.deref_raw().map_err(crate::fmt_err)?;
        let addr = value.as_ptr() as usize;
        addr.fmt(f)?;
        f.write_str(" as *const _")
    }
}

impl<'value, 'dwarf, R> fmt::Display for Pointer<'value, 'dwarf, crate::schema::Mut, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.deref_raw().map_err(crate::fmt_err)?;
        let addr = value.as_ptr() as usize;
        addr.fmt(f)?;
        f.write_str(" as *mut _")
    }
}

impl<'value, 'dwarf, K, R> ops::Deref for Pointer<'value, 'dwarf, K, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Pointer<'dwarf, K, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
