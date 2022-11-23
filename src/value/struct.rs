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
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::Struct<'dwarf, R>,
    ) -> Result<Self, crate::err::Error> {
        Ok(Self { schema, value })
    }

    /// The fields of this struct.
    pub fn fields(&self) -> Result<super::Fields<'value, 'dwarf, R>, crate::err::Error> {
        let fields = self.schema.fields()?;
        Ok(super::Fields::new(fields, self.value))
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
        let mut debug_struct = f.debug_struct("deflect::value::Struct");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, R> fmt::Display for Struct<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_name = self.name().map_err(crate::fmt_err)?;
        let type_name = type_name.to_string_lossy().map_err(crate::fmt_err)?;
        let mut debug_struct = f.debug_struct(&type_name);
        let mut fields = self.fields().map_err(crate::fmt_err)?;
        let mut fields = fields.iter().map_err(crate::fmt_err)?;
        while let Some(field) = fields.try_next().map_err(crate::fmt_err)? {
            let field_name = self.name().map_err(crate::fmt_err)?;
            let field_name = field_name.to_string_lossy().map_err(crate::fmt_err)?;
            let field_value = field.value().map_err(crate::fmt_err)?;
            debug_struct.field(&field_name, &crate::DebugDisplay(field_value));
        }
        debug_struct.finish()
    }
}
