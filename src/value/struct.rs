use std::{fmt, ops};

pub struct Struct<'dwarf, 'value, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Struct<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'dwarf, 'value, R> Struct<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Struct<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { schema, value }
    }

    pub fn fields<F>(&self, mut f: F)
    where
        F: FnMut(super::Field<'dwarf, 'value, R>),
    {
        self.schema
            .fields(|field_type| f(unsafe { super::Field::new(field_type, self.value) }));
    }
}

impl<'dwarf, 'value, R> ops::Deref for Struct<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Struct<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}

impl<'dwarf, 'value, R> fmt::Debug for Struct<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some(struct_name) = (match self.name() {
            Ok(name) => name,
            Err(err) => panic!("{:?}", err),
        }) else {
            panic!("field does not have a name");
        };
        let struct_name = match struct_name.to_string_lossy() {
            Ok(name) => name,
            Err(err) => panic!("{:?}", err),
        };
        let mut debug_struct = f.debug_struct(&struct_name);
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
            debug_struct.field(&field_name, &field_value);
        });
        debug_struct.finish()
    }
}
