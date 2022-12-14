use std::fmt;

/// A reflected struct value.
pub struct Struct<'value, 'dwarf, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    schema: crate::schema::Struct<'dwarf, P::Reader>,
    value: crate::Bytes<'value>,
    provider: &'dwarf P,
}

impl<'dwarf, R> crate::schema::Struct<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) unsafe fn with_bytes<'value, P>(
        self,
        provider: &'dwarf P,
        value: crate::Bytes<'value>,
    ) -> Result<Struct<'value, 'dwarf, P>, crate::Error>
    where
        P: crate::DebugInfoProvider<Reader = R>,
    {
        Ok(Struct {
            schema: self,
            value,
            provider,
        })
    }
}

impl<'value, 'dwarf, P> Struct<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// The schema of this value.
    pub fn schema(&self) -> &crate::schema::Struct<'dwarf, P::Reader> {
        &self.schema
    }

    /// Get a field of this struct by name.
    pub fn field<N>(&self, field_name: N) -> Result<Option<super::Field<'value, 'dwarf, P>>, crate::Error>
    where
        N: AsRef<[u8]>,
    {
        let target_name = field_name.as_ref();
        let mut fields = self.fields()?;
        let mut fields = fields.iter()?;
        while let Some(field) = fields.try_next()? {
            let field_name = field.schema().name()?;
            let field_name = field_name.to_slice()?;
            if target_name == field_name.as_ref() {
                return Ok(Some(field));
            }
        }
        Ok(None)
    }

    /// The fields of this struct.
    pub fn fields(&self) -> Result<super::Fields<'value, 'dwarf, P>, crate::Error> {
        let fields = self.schema.fields()?;
        Ok(super::Fields::new(fields, self.value, self.provider))
    }
}

impl<'value, 'dwarf, P> fmt::Debug for Struct<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::Struct");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, P> fmt::Display for Struct<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let schema = self.schema();
        let type_name = schema.name().map_err(crate::fmt_err)?;
        let type_name = type_name.to_string_lossy().map_err(crate::fmt_err)?;
        let mut debug_struct = f.debug_struct(&type_name);
        let mut fields = self.fields().map_err(crate::fmt_err)?;
        let mut fields = fields.iter().map_err(crate::fmt_err)?;
        while let Some(field) = fields.try_next().map_err(crate::fmt_err)? {
            let field_name = field.schema().name().map_err(crate::fmt_err)?;
            let field_name = field_name.to_string_lossy().map_err(crate::fmt_err)?;
            let field_value = field.value().map_err(crate::fmt_err)?;
            debug_struct.field(&field_name, &crate::DebugDisplay(field_value));
        }
        debug_struct.finish()
    }
}
