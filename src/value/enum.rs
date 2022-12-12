use std::{fmt, ops};

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
    /// The variant of this enum.
    pub fn variant(&self) -> Result<super::Variant<'value, 'dwarf, P>, crate::Error> {
        let mut default = None;
        let mut matched = None;

        let discr_loc = self.discriminant_location().clone();
        let enum_addr = self.value.as_ptr() as *const () as u64;
        let discr_addr = discr_loc.address(enum_addr)?;

        let mut variants = self.variants()?;
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
            .ok_or(crate::error::Kind::enum_destructure())?;
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
        self.name().fmt(f)?;
        f.write_str("::")?;
        self.variant().map_err(crate::fmt_err)?.fmt(f)
    }
}

impl<'value, 'dwarf, P> ops::Deref for Enum<'value, 'dwarf, P>
where
    P: 'dwarf + crate::DebugInfoProvider,
{
    type Target = crate::schema::Enum<'dwarf, P::Reader>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
