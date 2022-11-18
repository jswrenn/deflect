use std::{fmt, ops};

pub struct Ref<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Ref<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'value, 'dwarf, R> Ref<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Ref<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { schema, value }
    }

    /// The value behind this reference.
    pub fn value(&'dwarf self) -> Result<super::Value<'value, 'dwarf, R>, crate::Error> {
        let value = unsafe { *(self.value.as_ptr() as *const *const crate::Byte) };
        let r#type = self.schema.r#type()?;
        let r#type = r#type.ok_or(crate::ErrorKind::MissingAttr {
            attr: crate::gimli::DW_AT_type,
        })?;
        let size = r#type.size()?.unwrap_or(0).try_into().unwrap();
        let value = std::ptr::slice_from_raw_parts(value, size);
        let value = unsafe { &*value };
        Ok(unsafe { super::Value::with_type(r#type, value) })
    }
}

impl<'value, 'dwarf, R> fmt::Debug for Ref<'value, 'dwarf, R>
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

impl<'value, 'dwarf, R> fmt::Display for Ref<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.schema.fmt(f)
    }
}

impl<'value, 'dwarf, R> ops::Deref for Ref<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Ref<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
