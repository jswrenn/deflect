use std::{fmt, ops};

/// A value of a sum type; e.g., a Rust-style enum.
pub struct Enum<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Enum<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'value, 'dwarf, R> Enum<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Enum<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        let size = schema.size().unwrap() as usize;
        let value = &value[..size];
        Self { schema, value }
    }

    /// The variant of this enum.
    pub fn variant(&self) -> Result<super::Variant<'value, 'dwarf, R>, crate::Error> {
        let mut default = None;
        let mut matched = None;

        let discr_loc = self.discriminant_location().clone();
        let enum_addr = self.value.as_ptr() as *const () as u64;
        let discr_addr = discr_loc.address(enum_addr).unwrap();

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

        Ok(unsafe { super::Variant::new(matched.or(default).unwrap(), self.value) })
    }
}

impl<'value, 'dwarf, R> fmt::Debug for Enum<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::Enum");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, R> fmt::Display for Enum<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name().unwrap())?;
        f.write_str("::")?;
        self.variant()
            .expect("could not reflect into variant")
            .fmt(f)
    }
}

impl<'value, 'dwarf, R> ops::Deref for Enum<'value, 'dwarf, R>
where
    R: 'dwarf + crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Enum<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
