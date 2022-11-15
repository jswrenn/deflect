use std::fmt;

use crate::schema::DiscriminantValue;
pub struct Enum<'dwarf, 'value, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    r#type: crate::schema::Enum<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'dwarf, 'value, R> fmt::Debug for Enum<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name())?;
        f.write_str("::")?;
        self.variant()
            .expect("Could not reflect into variant.")
            .fmt(f)
    }
}

impl<'dwarf, 'value, R> Enum<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        r#type: crate::schema::Enum<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { r#type, value }
    }

    pub fn name(&self) -> String {
        self.r#type.name().unwrap().to_string()
    }

    pub fn variant(&self) -> Result<super::Variant<'dwarf, 'value, R>, crate::Error> {
        let mut default = None;
        let mut matched = None;

        let discr = self.r#type.discriminant();
        let discr_loc = discr.location().clone();
        let enum_addr = self.value.as_ptr() as *const () as u64;
        let discr_addr = discr_loc.address(enum_addr).unwrap();

        let mut variants = self.r#type.variants()?;
        let mut variants = variants.iter()?;

        while let Some(variant) = variants.next()? {
            if let Some(discriminant) = variant.discriminant_value() {
                let matches = match discriminant {
                    DiscriminantValue::U8(v) => (unsafe { *(discr_addr as *const u8) } == v),
                    DiscriminantValue::U16(v) => (unsafe { *(discr_addr as *const u16) } == v),
                    DiscriminantValue::U32(v) => (unsafe { *(discr_addr as *const u32) } == v),
                    DiscriminantValue::U64(v) => (unsafe { *(discr_addr as *const u64) } == v),
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
