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
    discriminant_val: Option<super::Data>,
}

impl<'value, 'dwarf, R> Variant<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) fn new(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
        discriminant_val: Option<super::Data>,
    ) -> Self {
        Self {
            dwarf,
            unit,
            entry,
            discriminant_val,
        }
    }

    /// The [DWARF](crate::gimli::Dwarf) sections that this `Variant`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn dwarf(&self) -> &'dwarf crate::gimli::Dwarf<R> {
        self.dwarf
    }

    /// The DWARF [unit][crate::gimli::Unit] that this `Variant`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn unit(&self) -> &crate::gimli::Unit<R, usize> {
        self.unit
    }

    /// The [debugging information entry][crate::gimli::DebuggingInformationEntry] this `Variant` abstracts over.
    #[allow(dead_code)]
    pub(crate) fn entry(&self) -> &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R> {
        &self.entry
    }

    /// The name of this variant.
    pub fn name(&self) -> Result<Name<R>, crate::err::Error> {
        Name::from_die(self.dwarf(), self.unit(), self.entry())
    }

    /// The size of this variant, in bytes.
    pub fn size(&self) -> Result<Option<u64>, crate::err::Error> {
        Ok(crate::get_size_opt(self.entry())?)
    }

    /// The alignment of this variant, in bytes.
    pub fn align(&self) -> Result<Option<u64>, crate::err::Error> {
        Ok(crate::get_align(self.entry())?)
    }

    pub fn file(&self) -> Result<Option<Cow<'_, str>>, crate::err::Error> {
        crate::get_file(self.dwarf, self.unit, &self.entry)
    }

    pub fn discriminant_value(&self) -> &Option<super::Data> {
        &self.discriminant_val
    }

    /// The fields of this variant.
    pub fn fields(&self) -> Result<super::Fields<'dwarf, R>, crate::err::Error> {
        let tree = self.unit.entries_tree(Some(self.entry.offset()))?;
        Ok(super::Fields::from_tree(self.dwarf, self.unit, tree))
    }
}

impl<'value, 'dwarf, R> fmt::Display for Variant<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_name = self.name().map_err(crate::fmt_err)?;
        let variant_name = variant_name.to_string_lossy().map_err(crate::fmt_err)?;
        let mut debug_struct = f.debug_struct(&variant_name);
        let mut fields = self.fields().map_err(crate::fmt_err)?;
        let mut fields = fields.iter().map_err(crate::fmt_err)?;
        while let Some(field) = fields.try_next().map_err(crate::fmt_err)? {
            let field_type = field.r#type().map_err(crate::fmt_err)?;
            let field_name = self.name().map_err(crate::fmt_err)?;
            let field_name = field_name.to_string_lossy().map_err(crate::fmt_err)?;
            debug_struct.field(&field_name, &crate::DebugDisplay(field_type));
        }
        debug_struct.finish()
    }
}
