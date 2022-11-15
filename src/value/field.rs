use std::{
    fmt::{self, Write},
    ops,
};

pub struct Field<'dwarf, 'value, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Field<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'dwarf, 'value, R> Field<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Field<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { schema, value }
    }

    /// Get the value of this field.
    pub fn value(&'dwarf self) -> Result<super::Value<'dwarf, 'value, R>, crate::Error> {
        let r#type = self
            .r#type()
            .transpose()
            .ok_or(
                crate::ErrorKind::MissingAttr {
                    attr: crate::gimli::DW_AT_type,
                }
                .into(),
            )
            .flatten()?;
        Ok(unsafe { super::Value::with_type(r#type, self.value) })
    }
}

impl<'dwarf, 'value, R> fmt::Debug for Field<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("field");
        let Some(field_name) = (match self.name() {
            Ok(name) => name,
            Err(err) => panic!("{:?}", err),
        }) else {
            panic!("field does not have a name");
        };
        let field_name = match field_name.to_string_lossy() {
            Ok(name) => name,
            Err(err) => panic!("{:?}", err),
        };
        f.write_str(&field_name)?;
        f.write_str(" : ")?;
        self.value().fmt(f)
    }
}

impl<'dwarf, 'value, R> ops::Deref for Field<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Field<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
