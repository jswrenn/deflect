use std::fmt;

/// A value of a sum type; e.g., a Rust-style enum.
pub struct Enum<'value, 'dwarf, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    schema: crate::schema::Enum<'dwarf, P::Reader>,
    value: crate::Bytes<'value>,
    provider: &'dwarf P,
}

impl<'dwarf, R> crate::schema::Enum<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) unsafe fn with_bytes<'value, P>(
        self,
        provider: &'dwarf P,
        value: crate::Bytes<'value>,
    ) -> Result<Enum<'value, 'dwarf, P>, crate::Error>
    where
        P: crate::DebugInfoProvider<Reader = R>,
    {
        let size = self.size()?.try_into()?;
        let value = &value[..size];
        Ok(Enum {
            schema: self,
            value,
            provider,
        })
    }
}

impl<'value, 'dwarf, P> Enum<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// The schema of this value.
    pub fn schema(&self) -> &crate::schema::Enum<'dwarf, P::Reader> {
        &self.schema
    }

    /// The variant of this enum.
    pub fn variant(&self) -> Result<super::Variant<'value, 'dwarf, P>, crate::Error> {
        let mut default = None;
        let mut matched = None;

        let schema = self.schema();
        let discr_loc = schema.discriminant_location().clone();
        let enum_addr = self.value.as_ptr() as *const () as u64;
        let discr_addr = discr_loc.address(enum_addr)?;

        let mut variants = schema.variants()?;
        let mut variants = variants.iter()?;

        while let Some(variant) = variants.try_next()? {
            if let Some(discriminant) = variant.discriminant_value() {
                use crate::schema::Data;
                let matches = match discriminant {
                    Data::u8(v) => (unsafe { *(discr_addr as *const u8) } == *v),
                    Data::u16(v) => (unsafe { *(discr_addr as *const u16) } == *v),
                    Data::u32(v) => (unsafe { *(discr_addr as *const u32) } == *v),
                    Data::u64(v) => (unsafe { *(discr_addr as *const u64) } == *v),
                };
                if matches {
                    matched = Some(variant.clone());
                }
            } else {
                default = Some(variant.clone());
            }
        }

        let schema = matched
            .or(default)
            .ok_or_else(crate::error::enum_destructure)?;
        Ok(unsafe { super::Variant::new(schema, self.value, self.provider) })
    }
}

impl<'value, 'dwarf, P> fmt::Debug for Enum<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::Enum");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, P> fmt::Display for Enum<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.schema().name().fmt(f)?;
        f.write_str("::")?;
        self.variant().map_err(crate::fmt_err)?.fmt(f)
    }
}

impl<'value, 'dwarf, P> serde::Serialize for Enum<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStructVariant;

        let schema = self.schema();
        let offset = schema.entry().offset();
        let type_name = schema.name();
        let type_name = type_name
            .to_static_str(offset)
            .map_err(crate::ser_err)?
            .clone();

        let variant = self.variant().map_err(crate::ser_err)?;
        let variant_schema = variant.schema();
        let offset = variant_schema.entry().offset();
        let variant_name = variant_schema.name().map_err(crate::ser_err)?;
        let variant_name = variant_name
            .to_static_str(offset)
            .map_err(crate::ser_err)?
            .clone();
        let variant_index = variant_schema.index();

        let mut fields = variant.fields().map_err(crate::ser_err)?;
        let mut fields_iter = fields.iter().map_err(crate::ser_err)?;
        let mut fields = vec![];
        while let Some(f) = fields_iter.try_next().map_err(crate::ser_err)? {
            fields.push(f);
        }

        let mut s = serializer.serialize_struct_variant(
            &type_name,
            variant_index.try_into().map_err(crate::ser_err)?,
            &variant_name,
            fields.len(),
        )?;

        fields.into_iter().try_for_each(|field| {
            let schema = field.schema();
            let offset = schema.entry().offset();
            let field_name = schema.name().map_err(crate::ser_err)?;
            let field_name = field_name.to_static_str(offset).map_err(crate::ser_err)?;
            let field_value = field.value().map_err(crate::ser_err)?;
            s.serialize_field(&field_name, &field_value)
        })?;

        s.end()
    }
}
