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
        let Some(name) = (match self.name() {
            Ok(name) => name,
            Err(err) => panic!("{:?}", err),
        }) else {
            panic!("type does not have a name");
        };
        let name = match name.to_string_lossy() {
            Ok(name) => name,
            Err(err) => panic!("{:?}", err),
        };
        f.write_str(&name)
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

    /// The [debugging information entry][crate::gimli::DebuggingInformationEntry] this type abstracts over.
    pub fn entry(&self) -> &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R> {
        &self.entry
    }

    /// The name of this primitive type.
    pub fn name(&self) -> Result<Option<super::Name<R>>, crate::Error> {
        Ok(super::Name::from_die(self.dwarf(), self.unit(), self.entry())?)
    }

    /// The type of the field.
    pub fn r#type(&'dwarf self) -> Result<Option<super::Type<'dwarf, R>>, crate::Error> {
        let maybe_type = crate::get_type_opt(self.unit(), self.entry())?;
        Ok(if let Some(r#type) = maybe_type {
            Some(super::Type::from_die(self.dwarf, self.unit, r#type)?)
        } else {
            None
        })
    }
}
