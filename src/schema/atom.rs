use super::Name;
use std::{fmt, marker::PhantomData};

/// A primitive, non-compound (i.e., "atomic") type, like [`u8`] or [`bool`].
#[derive(Clone)]
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
    ) -> Result<Self, crate::err::Error> {
        crate::check_tag(&entry, crate::gimli::DW_TAG_base_type)?;

        let name = Name::from_die(dwarf, unit, &entry)?;
        let expected = std::any::type_name::<T>();
        if name.to_slice()? != expected.as_bytes() {
            let actual = name.to_string_lossy()?.to_string();
            Err(crate::err::Kind::name_mismatch(expected, actual))?;
        }

        let size: usize = crate::get_size(&entry)?
            .try_into()
            .map_err(crate::err::Kind::TryFromInt)?;
        let expected = core::mem::size_of::<T>();
        if size != expected {
            Err(crate::err::Kind::size_mismatch(expected, size))?;
        }

        Ok(Self {
            dwarf,
            unit,
            entry,
            r#type: PhantomData,
        })
    }

    /// The [DWARF](crate::gimli::Dwarf) sections that this `Atom`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn dwarf(&self) -> &'dwarf crate::gimli::Dwarf<R> {
        self.dwarf
    }

    /// The DWARF [unit][crate::gimli::Unit] that this `Atom`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn unit(&self) -> &crate::gimli::Unit<R, usize> {
        self.unit
    }

    /// The [debugging information entry][crate::gimli::DebuggingInformationEntry] this `Atom` abstracts over.
    #[allow(dead_code)]
    pub(crate) fn entry(&self) -> &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R> {
        &self.entry
    }

    /// The name of this primitive type.
    pub fn name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    /// The size of this type, in bytes.
    pub fn size(&self) -> u64 {
        std::mem::size_of::<T>() as _
    }

    /// The alignment of this type, in bytes.
    pub fn align(&self) -> u64 {
        std::mem::size_of::<T>() as _
    }
}

impl<'dwarf, T, R> fmt::Debug for Atom<'dwarf, T, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple = f.debug_tuple("deflect::schema::Atom");
        debug_tuple.field(&crate::debug::DebugEntry::new(
            self.dwarf,
            self.unit,
            &self.entry,
        ));
        debug_tuple.finish()
    }
}

impl<'dwarf, T, R> fmt::Display for Atom<'dwarf, T, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::any::type_name::<T>().fmt(f)
    }
}
