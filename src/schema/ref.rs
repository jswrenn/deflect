use std::fmt;

pub struct Ref<'dwarf, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf gimli::Dwarf<R>,
    pub(crate) unit: &'dwarf gimli::Unit<R, usize>,
    entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
}

impl<'dwarf, 'value, R> fmt::Debug for Ref<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "&")?;
        self.r#type().fmt(f)
    }
}

impl<'dwarf, R> Ref<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    pub(crate) fn from_dw_pointer_type(
        dwarf: &'dwarf gimli::Dwarf<R>,
        unit: &'dwarf gimli::Unit<R, usize>,
        entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Self {
        assert_eq!(entry.tag(), gimli::DW_TAG_pointer_type);
        Self { dwarf, unit, entry }
    }

    pub fn r#type(&self) -> super::Type<'dwarf, R> {
        let offset = crate::get_type(&self.entry).unwrap();
        let entry = self.unit.entry(offset).unwrap();
        super::Type::from_die(self.dwarf, self.unit, entry).unwrap()
    }
}
