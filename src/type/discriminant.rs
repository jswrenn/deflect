#[derive(Debug, Copy, Clone)]
pub enum Discriminant {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl<R> From<gimli::AttributeValue<R>> for Discriminant
where
    R: gimli::Reader<Offset = usize>,
{
    fn from(value: gimli::AttributeValue<R>) -> Self {
        match value {
            gimli::AttributeValue::Data1(value) => Self::U8(value),
            gimli::AttributeValue::Data2(value) => Self::U16(value),
            gimli::AttributeValue::Data4(value) => Self::U32(value),
            gimli::AttributeValue::Data8(value) => Self::U64(value),
            gimli::AttributeValue::Udata(value) => Self::U8(value as _),
            _ => todo!(),
        }
    }
}
