use std::{borrow::Cow, fmt};

/// The name associated with a debuginfo entry.
#[derive(Clone)]
pub struct Name<R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    name: R,
}

impl<R> Name<R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Constructs a `Name` from the [`DW_AT_name`][crate::gimli::DW_AT_name]
    /// attribute of the given `entry`.
    pub(crate) fn from_die(
        dwarf: &crate::gimli::Dwarf<R>,
        unit: &crate::gimli::Unit<R, usize>,
        entry: &crate::gimli::DebuggingInformationEntry<'_, '_, R>,
    ) -> Result<Self, crate::Error> {
        let name = crate::get(entry, crate::gimli::DW_AT_name)?;
        let name = dwarf.attr_string(unit, name)?;
        Ok(Self { name })
    }

    /// Constructs a `Name` from the [`DW_AT_name`][crate::gimli::DW_AT_name]
    /// attribute of the given `entry`.
    pub(crate) fn from_die_opt(
        dwarf: &crate::gimli::Dwarf<R>,
        unit: &crate::gimli::Unit<R, usize>,
        entry: &crate::gimli::DebuggingInformationEntry<'_, '_, R>,
    ) -> Result<Option<Self>, crate::Error> {
        let name = crate::get_opt(entry, crate::gimli::DW_AT_name)?;
        Ok(if let Some(name) = name {
            let name = dwarf.attr_string(unit, name)?;
            Some(Self { name })
        } else {
            None
        })
    }

    /// Convert all remaining data to a clone-on-write string.
    ///
    /// The string will be borrowed where possible, but some readers may always
    /// return an owned string.
    ///
    /// Returns an error if the data contains invalid characters.
    pub fn to_string(&self) -> Result<Cow<'_, str>, crate::Error> {
        Ok(self.name.to_string()?)
    }

    /// Convert all remaining data to a clone-on-write string, including invalid
    /// characters.
    ///
    /// The string will be borrowed where possible, but some readers may always
    /// return an owned string.
    pub fn to_string_lossy(&self) -> Result<Cow<'_, str>, crate::Error> {
        Ok(self.name.to_string_lossy()?)
    }

    /// Return all remaining data as a clone-on-write slice of bytes.
    ///
    /// The slice will be borrowed where possible, but some readers may always
    /// return an owned vector.
    pub fn to_slice(&self) -> Result<Cow<'_, [u8]>, crate::Error> {
        Ok(self.name.to_slice()?)
    }

    pub(crate) fn to_static_str(
        &self,
        key: crate::gimli::UnitOffset,
    ) -> Result<&'static str, crate::Error> {
        use dashmap::DashMap;
        use once_cell::sync::Lazy;

        static NAME_CACHE: Lazy<DashMap<crate::gimli::UnitOffset, &'static str>> =
            Lazy::new(DashMap::new);

        let name = NAME_CACHE.entry(key).or_try_insert_with(|| {
            let name = self.to_string_lossy()?;
            let name = name.to_string();
            let name = Box::leak(name.into_boxed_str());
            Ok::<_, crate::Error>(name)
        })?;

        Ok(*name)
    }
}

impl<R> fmt::Debug for Name<R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string_lossy().map_err(crate::fmt_err)?.fmt(f)
    }
}

impl<R> fmt::Display for Name<R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string_lossy().map_err(crate::fmt_err)?.fmt(f)
    }
}
