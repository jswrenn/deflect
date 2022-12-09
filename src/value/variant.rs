use std::{fmt, ops};

/// A reflected enum variant value.
pub struct Variant<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    schema: crate::schema::Variant<'dwarf, P::Reader>,
    value: crate::Bytes<'value>,
    provider: &'dwarf P,
}

impl<'value, 'dwarf, P> Variant<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Variant<'dwarf, P::Reader>,
        value: crate::Bytes<'value>,
        provider: &'dwarf P,
    ) -> Self {
        Self {
            schema,
            value,
            provider,
        }
    }

    /// The fields of this variant.
    pub fn fields(&self) -> Result<super::Fields<'value, 'dwarf, P>, crate::Error> {
        let fields = self.schema.fields()?;
        Ok(super::Fields::new(fields, self.value, self.provider))
    }
}

impl<'value, 'dwarf, P> ops::Deref for Variant<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Target = crate::schema::Variant<'dwarf, P::Reader>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}

impl<'value, 'dwarf, P> fmt::Display for Variant<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_name = self.name().map_err(crate::fmt_err)?;
        let variant_name = variant_name.to_string_lossy().map_err(crate::fmt_err)?;
        let mut debug_struct = f.debug_struct(&variant_name);
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
