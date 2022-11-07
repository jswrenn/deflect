use std::fmt;

pub struct Field<'dwarf, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf gimli::Dwarf<R>,
    pub(crate) unit: &'dwarf gimli::Unit<R, usize>,
    entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
}

impl<'dwarf, 'value, R> fmt::Debug for Field<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(self.name().as_str()).finish()
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
    ) -> Self {
        assert_eq!(entry.tag(), gimli::DW_TAG_member);
        Self { dwarf, unit, entry }
    }

    pub fn name(&self) -> String {
        crate::get_name(&self.entry, self.dwarf, self.unit)
            .unwrap()
            .unwrap()
            .to_string_lossy()
            .unwrap()
            .to_owned()
            .to_string()
    }

    pub fn offset(&self) -> usize {
        self.entry
            .attr(gimli::DW_AT_data_member_location)
            .unwrap()
            .unwrap()
            .udata_value()
            .unwrap() as _
    }

    pub fn r#type(&self) -> super::Type<'dwarf, R> {
        let offset = crate::get_type(&self.entry).unwrap().unwrap();
        let entry = self.unit.entry(offset).unwrap();
        super::Type::from_die(self.dwarf, self.unit, entry)
    }
}
