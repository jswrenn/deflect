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
    pub fn name(&self) -> Result<Option<Name<R>>, crate::Error> {
        Ok(Name::from_die(self.dwarf(), self.unit(), self.entry())?)
    }

    /// The size of this variant, in bytes.
    pub fn size(&self) -> Result<Option<u64>, crate::Error> {
        Ok(crate::get_size(self.entry())?)
    }

    /// The alignment of this variant, in bytes.
    pub fn align(&self) -> Result<Option<u64>, crate::Error> {
        Ok(crate::get_align(self.entry())?)
    }

    pub fn file(&self) -> Result<Option<Cow<'_, str>>, crate::Error> {
        crate::get_file(self.dwarf, self.unit, &self.entry)
    }

    pub fn discriminant_value(&self) -> &Option<super::Data> {
        &self.discriminant_val
    }

    /// The fields of this variant.
    pub fn fields(&self) -> Result<super::Fields<'dwarf, R>, crate::Error> {
        let tree = self.unit.entries_tree(Some(self.entry.offset()))?;
        Ok(super::Fields::from_tree(self.dwarf, self.unit, tree))
    }
}

impl<'value, 'dwarf, R> fmt::Display for Variant<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = match self.name() {
            Ok(Some(variant_name)) => match variant_name.to_string_lossy() {
                Ok(variant_name) => f.debug_struct(&variant_name),
                Err(err) => panic!("reader error: {err}"),
            },
            Ok(None) => panic!("variant does not have a name"),
            Err(err) => panic!("reader error: {err}"),
        };
        let mut fields = self.fields().unwrap();
        let mut fields = fields.iter().unwrap();
        while let Some(field) = fields.try_next().unwrap() {
            let field_type = match field.r#type() {
                Ok(Some(field_type)) => field_type,
                Ok(None) => panic!("field does not have a name"),
                Err(err) => panic!("reader error: {err}"),
            };
            match field.name() {
                Ok(Some(field_name)) => match field_name.to_string_lossy() {
                    Ok(field_name) => {
                        debug_struct.field(&field_name, &crate::DebugDisplay(field_type))
                    }
                    Err(err) => panic!("reader error: {}", err),
                },
                Ok(None) => panic!("type does not have a name"),
                Err(err) => panic!("reader error: {}", err),
            };
        }
        debug_struct.finish()
    }
}
