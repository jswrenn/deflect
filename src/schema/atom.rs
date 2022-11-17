use super::Name;
use std::{fmt, marker::PhantomData};

/// A primitive, non-compound (i.e., "atomic") type, like `u8` or `bool`.
pub struct Atom<'dwarf, T, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    r#type: PhantomData<T>,
}

impl<'dwarf, T, R> Atom<'dwarf, T, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Construct an `Atom` from a [`DW_TAG_base_type`][crate::gimli::DW_TAG_base_type].
    pub(crate) fn from_dw_tag_base_type(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        crate::check_tag(&entry, crate::gimli::DW_TAG_base_type)?;
        let name = Name::from_die(dwarf, unit, &entry)?.ok_or(crate::ErrorKind::MissingAttr {
            attr: crate::gimli::DW_AT_name,
        })?;
        let name = name.to_slice()?;

        if name != std::any::type_name::<T>().as_bytes() {
            Err(crate::ErrorKind::ValueMismatch)?;
        }

        let size = crate::get_size(&entry)?.ok_or(crate::ErrorKind::MissingAttr {
            attr: crate::gimli::DW_AT_byte_size,
        })?;
        if size != core::mem::size_of::<T>() as _ {
            Err(crate::ErrorKind::ValueMismatch)?;
        }

        Ok(Self {
            dwarf,
            unit,
            entry,
            r#type: PhantomData,
        })
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

    /// The size of this type, in bytes.
    pub fn size(&self) -> Result<Option<u64>, crate::Error> {
        Ok(crate::get_size(self.entry())?)
    }

    /// The alignment of this type, in bytes.
    pub fn align(&self) -> Result<Option<u64>, crate::Error> {
        Ok(crate::get_align(self.entry())?)
    }
}

impl<'dwarf, T, R> fmt::Display for Atom<'dwarf, T, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Ok(Some(type_name)) => type_name.fmt(f),
            Ok(None) => panic!("type does not have a name"),
            Err(err) => panic!("reader error: {:?}", err),
        }
    }
}
