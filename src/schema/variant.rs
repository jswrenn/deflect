use super::Name;
use std::{borrow::Cow, fmt};

/// A variant of an [enum][super::Enum].
#[derive(Clone)]
pub struct Variant<'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    discriminant_value: Option<super::discriminant::DiscriminantValue>,
}

impl<'dwarf, 'value, R> fmt::Debug for Variant<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = format!(
            "{} = {:?}",
            self.name().unwrap().unwrap(),
            &self.discriminant_value
        );
        let mut ds = f.debug_struct(name.as_str());
        let mut fields = self.fields().unwrap();
        let mut fields = fields.iter().unwrap();
        while let Some(field) = fields.next().unwrap() {
            ds.field(
                &field.name().unwrap().unwrap().to_string().unwrap(),
                &field.r#type(),
            );
        }
        ds.finish()
    }
}

impl<'dwarf, 'value, R> Variant<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) fn new(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
        discriminant_value: Option<super::discriminant::DiscriminantValue>,
    ) -> Self {
        Self {
            dwarf,
            unit,
            entry,
            discriminant_value,
        }
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

    /// The name of this primitive type.
    pub fn name(&self) -> Result<Option<Name<R>>, crate::Error> {
        Ok(Name::from_die(self.dwarf(), self.unit(), self.entry())?)
    }

    /// The size of this field, in bytes.
    pub fn size(&self) -> Result<Option<u64>, crate::Error> {
        Ok(crate::get_size(self.entry())?)
    }

    /// The alignment of this field, in bytes.
    pub fn align(&self) -> Result<Option<u64>, crate::Error> {
        Ok(crate::get_align(self.entry())?)
    }

    pub fn file(&self) -> Result<Option<Cow<'_, str>>, crate::Error> {
        crate::get_file(self.dwarf, self.unit, &self.entry)
    }

    pub fn discriminant_value(&self) -> Option<super::discriminant::DiscriminantValue> {
        self.discriminant_value
    }

    /// The fields of this struct.
    pub fn fields(&self) -> Result<super::Fields<'dwarf, R>, crate::Error> {
        let tree = self.unit.entries_tree(Some(self.entry.offset()))?;
        Ok(super::Fields::from_tree(self.dwarf, self.unit, tree))
    }
}
