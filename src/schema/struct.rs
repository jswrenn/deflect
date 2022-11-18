use super::Name;
use std::fmt;

/// A Rust-like `struct`.
pub struct Struct<'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
}

impl<'value, 'dwarf, R> Struct<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Construct a new `Struct` from a [`DW_TAG_structure_type`][crate::gimli::DW_TAG_structure_type].
    pub(crate) fn from_dw_tag_structure_type(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        crate::check_tag(&entry, crate::gimli::DW_TAG_structure_type)?;
        Ok(Self { dwarf, unit, entry })
    }

    /// The [DWARF](crate::gimli::Dwarf) sections that this `Struct`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn dwarf(&self) -> &'dwarf crate::gimli::Dwarf<R> {
        self.dwarf
    }

    /// The DWARF [unit][crate::gimli::Unit] that this `Struct`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn unit(&self) -> &crate::gimli::Unit<R, usize> {
        self.unit
    }

    /// The [debugging information entry][crate::gimli::DebuggingInformationEntry] this `Struct` abstracts over.
    #[allow(dead_code)]
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

    /// The fields of this struct.
    pub fn fields(&self) -> Result<super::Fields<'dwarf, R>, crate::Error> {
        let tree = self.unit.entries_tree(Some(self.entry.offset()))?;
        Ok(super::Fields::from_tree(self.dwarf, self.unit, tree))
    }
}

impl<'dwarf, R> fmt::Debug for Struct<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple = f.debug_tuple("deflect::schema::Struct");
        debug_tuple.field(&crate::debug::DebugEntry::new(
            self.dwarf,
            self.unit,
            &self.entry,
        ));
        debug_tuple.finish()
    }
}

impl<'value, 'dwarf, R> fmt::Display for Struct<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = match self.name() {
            Ok(Some(type_name)) => match type_name.to_string_lossy() {
                Ok(type_name) => f.debug_struct(&type_name),
                Err(err) => panic!("reader error: {err}"),
            },
            Ok(None) => panic!("type does not have a name"),
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
