#[derive(Clone)]
pub struct Discriminant<R>
where
    R: gimli::Reader<Offset = usize>,
{
    r#type: super::DiscriminantType,
    align: u64,
    location: gimli::AttributeValue<R>,
}

impl<R> Discriminant<R>
where
    R: gimli::Reader<Offset = usize>,
{
    pub(crate) fn from_dw_tag_enumeration_type<'dwarf>(
        dwarf: &'dwarf gimli::Dwarf<R>,
        unit: &'dwarf gimli::Unit<R, usize>,
        entry: &'dwarf gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        assert_eq!(entry.tag(), gimli::DW_TAG_enumeration_type);

        let r#type = crate::get_type(&entry)
            .map(|offset| unit.entry(offset).unwrap())
            .map(|entry| super::DiscriminantType::from_die(dwarf, unit, entry))
            .unwrap();

        let align = crate::get_align(&entry)?;
        let location = gimli::AttributeValue::Udata(0);

        Ok(Self {
            r#type,
            align,
            location,
        })
    }

    pub(crate) fn from_dw_tag_variant_part<'dwarf>(
        dwarf: &'dwarf gimli::Dwarf<R>,
        unit: &'dwarf gimli::Unit<R, usize>,
        entry: &'dwarf gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        assert_eq!(entry.tag(), gimli::DW_TAG_variant_part);

        let dw_tag_member = crate::get_attr_ref(&entry, gimli::DW_AT_discr)
            .unwrap()
            .and_then(|offset| unit.entry(offset).ok())
            .unwrap();

        let r#type = unit.entry(crate::get_type(&dw_tag_member).unwrap())
            .map(|entry| super::DiscriminantType::from_die(dwarf, unit, entry))
            .expect("no entry");

        let align = crate::get_align(&entry).unwrap_or(1);

        let location = dw_tag_member
            .attr_value(gimli::DW_AT_data_member_location)
            .unwrap()
            .unwrap();

        Ok(Self {
            r#type,
            align,
            location,
        })
    }

    pub fn r#type(&self) -> &super::DiscriminantType {
        &self.r#type
    }

    pub fn alignment(&self) -> usize {
        self.align as _
    }

    pub fn location(&self) -> &gimli::AttributeValue<R> {
        &self.location
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DiscriminantValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl<R> From<gimli::AttributeValue<R>> for DiscriminantValue
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
