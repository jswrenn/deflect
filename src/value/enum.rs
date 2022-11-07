use std::fmt;

use crate::r#type::Discriminant;
pub struct Enum<'dwarf, 'value, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    r#type: crate::r#type::Enum<'dwarf, R>,
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
        r#type: crate::r#type::Enum<'dwarf, R>,
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
        self.r#type.variants(|variant| {
            if let Some(discriminant) = variant.discriminant() {
                let matches = match discriminant {
                    Discriminant::U8(v) => (unsafe { *(self.value as *const _ as *const u8) } == v),
                    Discriminant::U16(v) => {
                        (unsafe { *(self.value as *const _ as *const u16) } == v)
                    }
                    Discriminant::U32(v) => {
                        (unsafe { *(self.value as *const _ as *const u32) } == v)
                    }
                    Discriminant::U64(v) => {
                        (unsafe { *(self.value as *const _ as *const u64) } == v)
                    }
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
