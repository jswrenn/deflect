use std::{fmt, ops};

pub struct Box<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Box<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'value, 'dwarf, R> Box<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::Box<'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        Ok(Self { schema, value })
    }
}

impl<'value, 'dwarf, R> Box<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// The reflected value behind this reference.
    pub fn deref(&self) -> Result<super::Value<'value, 'dwarf, R>, crate::Error>
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

impl<'value, 'dwarf, R> Box<'value, 'dwarf, R>
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

impl<'value, 'dwarf, R> fmt::Debug for Box<'value, 'dwarf, R>
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

impl<'value, 'dwarf, R> fmt::Display for Box<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("box ")?;
        self.deref().map_err(crate::fmt_err)?.fmt(f)
    }
}

impl<'value, 'dwarf, R> ops::Deref for Box<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Box<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
