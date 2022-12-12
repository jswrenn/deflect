use std::{fmt, ops};

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
    /// The fields of this struct.
    pub fn fields(&self) -> Result<super::Fields<'value, 'dwarf, P>, crate::Error> {
        let fields = self.schema.fields()?;
        Ok(super::Fields::new(fields, self.value, self.provider))
    }
}

impl<'value, 'dwarf, P> ops::Deref for Struct<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Target = crate::schema::Struct<'dwarf, P::Reader>;

    fn deref(&self) -> &Self::Target {
        &self.schema
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
        let type_name = self.name().map_err(crate::fmt_err)?;
        let type_name = type_name.to_string_lossy().map_err(crate::fmt_err)?;
        let mut debug_struct = f.debug_struct(&type_name);
        let mut fields = self.fields().map_err(crate::fmt_err)?;
        let mut fields = fields.iter().map_err(crate::fmt_err)?;
        while let Some(field) = fields.try_next().map_err(crate::fmt_err)? {
            let field_name = field.name().map_err(crate::fmt_err)?;
            let field_name = field_name.to_string_lossy().map_err(crate::fmt_err)?;
            let field_value = field.value().map_err(crate::fmt_err)?;
            debug_struct.field(&field_name, &crate::DebugDisplay(field_value));
        }
        debug_struct.finish()
    }
}
