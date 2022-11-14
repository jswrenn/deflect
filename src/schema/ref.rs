use std::fmt;

pub struct Ref<'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
}

impl<'dwarf, 'value, R> fmt::Debug for Ref<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "&")?;
        self.r#type().fmt(f)
    }
}

impl<'dwarf, R> Ref<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) fn from_dw_pointer_type(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Self {
        assert_eq!(entry.tag(), crate::gimli::DW_TAG_pointer_type);
        Self { dwarf, unit, entry }
    }

    /// The [DWARF](crate::gimli::Dwarf) sections that this debuginfo entry belongs to.
    pub fn dwarf(&self) -> &'dwarf crate::gimli::Dwarf<R> {
        self.dwarf
    }

    /// The DWARF [unit][gimli::Unit] that this debuginfo entry belongs to.
    pub fn unit(&self) -> &crate::gimli::Unit<R, usize> {
        self.unit
    }

    pub fn r#type(&self) -> super::Type<'dwarf, R> {
        let offset = crate::get_type(&self.entry).unwrap();
        let entry = self.unit.entry(offset).unwrap();
        super::Type::from_die(self.dwarf, self.unit, entry).unwrap()
    }
}
