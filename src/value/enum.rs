use std::fmt;

use crate::schema::DiscriminantValue;
pub struct Enum<'dwarf, 'value, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    r#type: crate::schema::Enum<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'dwarf, 'value, R> fmt::Debug for Enum<'dwarf, 'value, R>
where
    R: gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::", &self.name())?;
        self.variant().fmt(f)
    }
}

impl<'dwarf, 'value, R> Enum<'dwarf, 'value, R>
where
    R: gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        r#type: crate::schema::Enum<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { r#type, value }
    }

    pub fn name(&self) -> String {
        self.r#type.name()
    }

    pub fn variant(&self) -> super::Variant<'dwarf, 'value, R> {
        let mut default = None;
        let mut matched = None;

        let discriminant = self.r#type.discriminant();
        let location = discriminant.location();
        let ptr = self.value.as_ptr() as u64;
        let disr_addr = crate::eval_addr(&self.r#type.unit, location.clone(), ptr)
            .unwrap()
            .unwrap();

        self.r#type.variants(|variant| {
            if let Some(discriminant) = variant.discriminant_value() {
                let matches = match discriminant {
                    DiscriminantValue::U8(v) => (unsafe { *(disr_addr as *const u8) } == v),
                    DiscriminantValue::U16(v) => (unsafe { *(disr_addr as *const u16) } == v),
                    DiscriminantValue::U32(v) => (unsafe { *(disr_addr as *const u32) } == v),
                    DiscriminantValue::U64(v) => (unsafe { *(disr_addr as *const u64) } == v),
                };
                if matches {
                    matched = Some(variant.clone());
                }
            } else {
                default = Some(variant.clone());
            }
        });
        unsafe { super::Variant::new(matched.or(default).unwrap(), self.value) }
    }

    pub fn size(&self) -> usize {
        self.r#type.size()
    }

    pub fn align(&self) -> usize {
        self.r#type.align()
    }
}
