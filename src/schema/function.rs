use super::Name;
use std::{borrow::Cow, fmt};

/// A function.
pub struct Function<'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
}

impl<'dwarf, 'value, R> Function<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Construct a new `Function` from a [`DW_TAG_subroutine_type`][crate::gimli::DW_TAG_subroutine_type].
    pub(crate) fn from_dw_tag_subroutine_type(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        crate::check_tag(&entry, crate::gimli::DW_TAG_subroutine_type)?;
        Ok(Self { dwarf, unit, entry })
    }

    /// The [DWARF](crate::gimli::Dwarf) sections that this debuginfo entry belongs to.
    pub(crate) fn dwarf(&self) -> &'dwarf crate::gimli::Dwarf<R> {
        self.dwarf
    }

    /// The DWARF [unit][gimli::Unit] that this debuginfo entry belongs to.
    pub(crate) fn unit(&self) -> &crate::gimli::Unit<R, usize> {
        self.unit
    }

    /// The [debugging information entry][crate::gimli::DebuggingInformationEntry] this type abstracts over.
    pub(crate) fn entry(&self) -> &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R> {
        &self.entry
    }

    /// The name of this type.
    pub fn name(&self) -> Result<Option<Name<R>>, crate::Error> {
        Ok(Name::from_die(self.dwarf(), self.unit(), self.entry())?)
    }
}

impl<'dwarf, 'value, R> fmt::Debug for Function<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Can we get back function names?
        if let Some(name) = self.name().expect("gimli err") {
            let name = name.to_string_lossy().expect("gimli err");
            write!(f, "fn {name}<todo>(todo) -> (todo)")
        } else {
            write!(f, "fn ()<todo>(todo) -> (todo)")
        }
    }
}
