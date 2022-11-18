use std::{borrow::Cow, fmt};

use itertools::Itertools;

/// A sum type; e.g., a Rust-style enum.
pub struct Enum<'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    name: R,
    discriminant: super::Discriminant<'dwarf, R>,
}

impl<'dwarf, R> Enum<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Construct an `Enum` from a [`DW_TAG_enumeration_type`][crate::gimli::DW_TAG_enumeration_type].
    pub(crate) fn from_dw_tag_enumeration_type(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        crate::check_tag(&entry, crate::gimli::DW_TAG_enumeration_type)?;
        let name = crate::get_name(&entry, dwarf, unit)?;
        let discriminant = super::Discriminant::from_dw_tag_enumeration_type(dwarf, unit, &entry)?;

        Ok(Self {
            dwarf,
            unit,
            entry,
            name,
            discriminant,
        })
    }

    /// Construct an `Enum` from a [`DW_TAG_structure_type`][crate::gimli::DW_TAG_structure_type].
    pub(crate) fn from_dw_tag_structure_type(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        crate::check_tag(&entry, crate::gimli::DW_TAG_structure_type)?;
        let name = crate::get_name(&entry, dwarf, unit)?;

        let mut tree = unit.entries_tree(Some(entry.offset()))?;
        let root = tree.root()?;
        let mut variant_part = None;

        {
            let mut children = root.children();
            while let Some(child) = children.next()? {
                if child.entry().tag() == crate::gimli::DW_TAG_variant_part {
                    variant_part = Some(child.entry().clone());
                }
            }
        }

        let variant_part = variant_part.ok_or(crate::ErrorKind::MissingChild {
            tag: crate::gimli::DW_TAG_variant_part,
        })?;

        let discriminant =
            super::Discriminant::from_dw_tag_variant_part(dwarf, unit, &variant_part)?;

        Ok(Self {
            dwarf,
            unit,
            entry,
            name,
            discriminant,
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

    /// The name of this type.
    pub fn name(&self) -> Result<Cow<str>, crate::gimli::Error> {
        self.name.to_string_lossy()
    }

    /// The discriminant of this type.
    pub fn discriminant(&self) -> &super::Discriminant<R> {
        &self.discriminant
    }

    /// Variants of this type.
    pub fn variants(&self) -> Result<super::Variants<'dwarf, R>, crate::Error> {
        let mut tree = self.unit.entries_tree(Some(self.entry.offset()))?;
        let root = tree.root()?;
        let tree = match self.entry.tag() {
            crate::gimli::DW_TAG_enumeration_type => tree,
            crate::gimli::DW_TAG_structure_type => {
                let mut children = root.children();
                let mut variant_part = None;
                while let Some(child) = children.next()? {
                    if child.entry().tag() == crate::gimli::DW_TAG_variant_part {
                        variant_part = Some(child.entry().offset());
                    }
                }
                self.unit.entries_tree(variant_part)?
            }
            _ => todo!(),
        };
        Ok(super::Variants::from_tree(&self.dwarf, &self.unit, tree))
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

impl<'dwarf, R> fmt::Debug for Enum<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple = f.debug_tuple("deflect::schema::Enum");
        debug_tuple.field(&crate::debug::DebugEntry::new(
            self.dwarf,
            self.unit,
            &self.entry,
        ));
        debug_tuple.finish()
    }
}

impl<'dwarf, R> fmt::Display for Enum<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("enum ")?;
        match self.name() {
            Ok(enum_name) => enum_name.fmt(f)?,
            Err(err) => panic!("reader error: {:?}", err),
        };

        f.write_str(" {")?;

        if f.alternate() {
            f.write_str("\n    ")?;
        } else {
            f.write_str(" ")?;
        }

        let mut debug_struct = f.debug_tuple(&self.name().unwrap());
        let mut variants = self.variants().unwrap();
        let mut variants = variants.iter().unwrap();

        variants
            .format(if f.alternate() { ",\n    " } else { ", " })
            .fmt(f)?;

        if f.alternate() {
            f.write_str(",\n}")
        } else {
            f.write_str(" }")
        }
    }
}
