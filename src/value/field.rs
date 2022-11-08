use std::fmt;

pub struct Field<'dwarf, 'value, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    r#type: crate::schema::Field<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'dwarf, 'value, R> fmt::Debug for Field<'dwarf, 'value, R>
where
    R: gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(self.name().as_str()).finish()
    }
}

impl<'dwarf, 'value, R> Field<'dwarf, 'value, R>
where
    R: gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        r#type: crate::schema::Field<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { r#type, value }
    }

    pub fn name(&self) -> String {
        self.r#type.name()
    }

    pub fn r#type(&self) -> crate::schema::Type<'dwarf, R> {
        self.r#type.r#type()
    }

    pub fn value(&self) -> super::Value<'dwarf, 'value, R> {
        unsafe { super::Value::with_type(self.r#type(), self.value) }
    }
}
