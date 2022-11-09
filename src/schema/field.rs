use std::{borrow::Cow, fmt};

use gimli::UnitOffset;
pub struct Field<'dwarf, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf gimli::Dwarf<R>,
    unit: &'dwarf gimli::Unit<R, usize>,

    name: R,
    offset: gimli::AttributeValue<R>,
    r#type: UnitOffset,
}

impl<'dwarf, 'value, R> fmt::Debug for Field<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&self.name().unwrap().to_string()).finish()
    }
}

impl<'dwarf, R> Field<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    pub(crate) fn from_dw_tag_member(
        dwarf: &'dwarf gimli::Dwarf<R>,
        unit: &'dwarf gimli::Unit<R, usize>,
        entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        crate::check_tag(&entry, gimli::DW_TAG_member)?;
        let name = crate::get_name(&entry, dwarf, unit)?;
        let offset = crate::get_data_member_location(&entry)?;
        let r#type = crate::get_type(&entry)?;
        Ok(Self { dwarf, unit, name, offset, r#type })
    }

    pub fn name(&self) -> Result<Cow<str>, gimli::Error> {
        self.name.to_string_lossy()
    }

    pub fn offset(&self) -> usize {
        self.offset.udata_value().unwrap() as usize
    }

    pub fn r#type(&self) -> super::Type<'dwarf, R> {
        let entry = self.unit.entry(self.r#type).unwrap();
        super::Type::from_die(self.dwarf, self.unit, entry).unwrap()
    }
}
