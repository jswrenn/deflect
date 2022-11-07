use std::{fmt, mem::MaybeUninit};

pub struct Ref<'dwarf, 'value, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    r#type: crate::r#type::Ref<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'dwarf, 'value, R> fmt::Debug for Ref<'dwarf, 'value, R>
where
    R: gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "&")?;
        self.value().fmt(f)
    }
}

impl<'dwarf, 'value, R> Ref<'dwarf, 'value, R>
where
    R: gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        r#type: crate::r#type::Ref<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { r#type, value }
    }

    pub fn name(&self) -> String {
        todo!()
    }

    pub fn r#type(&self) -> crate::r#type::Type<'dwarf, R> {
        self.r#type.r#type()
    }

    pub fn value(&self) -> super::Value<'dwarf, 'value, R> {
        let value = unsafe { *(self.value.as_ptr() as *const *const crate::Byte) };
        let value = std::ptr::slice_from_raw_parts(value, self.r#type.r#type().size());
        let value = unsafe { &*value };
        unsafe { super::Value::with_type(self.r#type(), value) }
    }
}
