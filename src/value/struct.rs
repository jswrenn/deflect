use std::fmt;
pub struct Struct<'dwarf, 'value, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    r#type: crate::schema::Struct<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'dwarf, 'value, R> fmt::Debug for Struct<'dwarf, 'value, R>
where
    R: gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct(&self.name());
        self.fields(|field| {
            let value = field.value();
            ds.field(field.name().as_str(), &value);
        });
        ds.finish()
    }
}

impl<'dwarf, 'value, R> Struct<'dwarf, 'value, R>
where
    R: gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        r#type: crate::schema::Struct<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { r#type, value }
    }

    pub fn name(&self) -> String {
        self.r#type.name().unwrap().to_string()
    }

    pub fn fields<F>(&self, mut f: F)
    where
        F: FnMut(super::Field<'dwarf, 'value, R>),
    {
        self.r#type
            .fields(|field_type| f(unsafe { super::Field::new(field_type, self.value) }));
    }

    pub fn size(&self) -> usize {
        self.r#type.size()
    }

    pub fn align(&self) -> usize {
        self.r#type.align()
    }
}
