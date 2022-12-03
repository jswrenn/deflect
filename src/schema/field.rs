use super::{Name, Offset, Type};
use std::fmt;

/// A field of a [struct][super::Struct] or [variant][super::Variant].
#[derive(Clone)]
pub struct Field<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
}

impl<'dwarf, R> Field<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Construct a new `Field` from a [`DW_TAG_member`][crate::gimli::DW_TAG_member].
    pub(crate) fn from_dw_tag_member(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        let _tree = unit.entries_tree(Some(entry.offset()))?;
        //crate::debug::inspect_tree(&mut tree, dwarf, unit);
        crate::check_tag(&entry, crate::gimli::DW_TAG_member)?;
        Ok(Self { dwarf, unit, entry })
    }

    /// The [DWARF](crate::gimli::Dwarf) sections that this `Field`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn dwarf(&self) -> &'dwarf crate::gimli::Dwarf<R> {
        self.dwarf
    }

    /// The DWARF [unit][crate::gimli::Unit] that this `Field`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn unit(&self) -> &crate::gimli::Unit<R, usize> {
        self.unit
    }

    /// The [debugging information entry][crate::gimli::DebuggingInformationEntry] this `Field` abstracts over.
    #[allow(dead_code)]
    pub(crate) fn entry(&self) -> &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R> {
        &self.entry
    }

    /// The name of this primitive type.
    pub fn name(&self) -> Result<Name<R>, crate::Error> {
        Name::from_die(self.dwarf(), self.unit(), self.entry())
    }

    /// The size of this field, in bytes.
    pub fn size(&self) -> Result<Option<u64>, crate::Error> {
        Ok(crate::get_size_opt(self.entry())?)
    }

    /// The alignment of this field, in bytes.
    pub fn align(&self) -> Result<Option<u64>, crate::Error> {
        Ok(crate::get_align(self.entry())?)
    }

    /// The offset at which this field occurs.
    pub fn offset(&'dwarf self) -> Result<Offset<'dwarf, R>, crate::Error> {
        Offset::from_die(self.unit(), self.entry())
    }

    /// The type of the field.
    pub fn r#type(&self) -> Result<Type<'dwarf, R>, crate::Error> {
        let r#type = crate::get_type_res(self.unit, &self.entry)?;
        super::Type::from_die(self.dwarf, self.unit, r#type)
    }
}

impl<'dwarf, R> fmt::Display for Field<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name().map_err(crate::fmt_err)?.fmt(f)
    }
}
