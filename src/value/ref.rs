use std::{fmt::{self, Write}, ops, mem::MaybeUninit};

pub struct Ref<'dwarf, 'value, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Ref<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'dwarf, 'value, R> Ref<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Ref<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { schema, value }
    }

    pub fn value(&'dwarf self) -> Result<super::Value<'dwarf, 'value, R>, crate::Error> {
        let value = unsafe { *(self.value.as_ptr() as *const *const crate::Byte) };
        let r#type = self.schema.r#type()?;
        let r#type = r#type.ok_or(crate::ErrorKind::MissingAttr { attr: crate::gimli::DW_AT_type })?;
        let size = r#type.size();
        let value = std::ptr::slice_from_raw_parts(value, size);
        let value = unsafe { &*value };
        Ok(unsafe { super::Value::with_type(r#type, value) })
    }
}

impl<'dwarf, 'value, R> fmt::Debug for Ref<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.schema.fmt(f)
    }
}

impl<'dwarf, 'value, R> ops::Deref for Ref<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Ref<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}