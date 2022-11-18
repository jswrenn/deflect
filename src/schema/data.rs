#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum Data {
    u8(u8),
    u16(u16),
    u32(u32),
    u64(u64),
}

impl<R> TryFrom<crate::gimli::AttributeValue<R>> for Data
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Error = crate::DowncastErr<crate::gimli::AttributeValue<R>, Self>;

    fn try_from(value: crate::gimli::AttributeValue<R>) -> Result<Self, Self::Error> {
        Ok(match value {
            crate::gimli::AttributeValue::Data1(value)  => Self::u8(value),
            crate::gimli::AttributeValue::Data2(value) => Self::u16(value),
            crate::gimli::AttributeValue::Data4(value) => Self::u32(value),
            crate::gimli::AttributeValue::Data8(value) => Self::u64(value),
            crate::gimli::AttributeValue::Udata(value) => Self::u8(value as _),
            _ => todo!(),
        })
    }
}
