use std::{fmt, ops};

pub struct Variant<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Variant<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'value, 'dwarf, R> Variant<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Variant<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { schema, value }
    }

    /// The fields of this variant.
    pub fn fields(&self) -> Result<super::Fields<'value, 'dwarf, R>, crate::Error> {
        let fields = self.schema.fields()?;
        Ok(super::Fields::new(fields, self.value))
    }
}

impl<'value, 'dwarf, R> ops::Deref for Variant<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Variant<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}

impl<'value, 'dwarf, R> fmt::Display for Variant<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = match self.name() {
            Ok(Some(type_name)) => match type_name.to_string_lossy() {
                Ok(type_name) => f.debug_struct(&type_name),
                Err(err) => panic!("reader error: {}", err),
            },
            Ok(None) => panic!("variant does not have a name"),
            Err(err) => panic!("reader error: {}", err),
        };
        let mut fields = self.fields().unwrap();
        let mut fields = fields.iter().unwrap();
        while let Some(field) = fields.try_next().unwrap() {
            let Some(field_name) = (match field.name() {
                Ok(name) => name,
                Err(err) => panic!("{:?}", err),
            }) else {
                panic!("field does not have a name");
            };
            let field_name = match field_name.to_string_lossy() {
                Ok(name) => name,
                Err(err) => panic!("{:?}", err),
            };
            let field_value = match field.value() {
                Ok(value) => value,
                Err(err) => panic!("{:?}", err),
            };
            debug_struct.field(&field_name, &crate::DebugDisplay(field_value));
        }
        debug_struct.finish()
    }
}
