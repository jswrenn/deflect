use std::{fmt, ops};

pub struct Struct<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Struct<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'value, 'dwarf, R> Struct<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Struct<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { schema, value }
    }

    /// The fields of this struct.
    pub fn fields<F>(&self, mut f: F)
    where
        F: FnMut(super::Field<'value, 'dwarf, R>),
    {
        let mut fields = self.schema.fields().unwrap();
        let mut fields = fields.iter().unwrap();
        while let Some(field) = fields.try_next().unwrap() {
            f(unsafe { super::Field::new(field, self.value) })
        }
    }
}

impl<'value, 'dwarf, R> ops::Deref for Struct<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Struct<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}

impl<'value, 'dwarf, R> fmt::Debug for Struct<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct(std::any::type_name::<Self>());
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        Ok(())
    }
}

impl<'value, 'dwarf, R> fmt::Display for Struct<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = match self.name() {
            Ok(Some(type_name)) => match type_name.to_string_lossy() {
                Ok(type_name) => f.debug_struct(&type_name),
                Err(err) => panic!("reader error: {}", err),
            },
            Ok(None) => panic!("type does not have a name"),
            Err(err) => panic!("reader error: {}", err),
        };
        self.fields(|field| {
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
        });
        debug_struct.finish()
    }
}
