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
    discr_type_offset: crate::gimli::UnitOffset,
    name: R,
    location: super::Offset<'dwarf, R>,
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
        let discr_type_offset = crate::get_type(&entry)?;
        let location = super::Offset::zero(unit);

        Ok(Self {
            dwarf,
            unit,
            entry,
            discr_type_offset,
            name,
            location,
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

        let dw_tag_variant_part = variant_part.ok_or(crate::ErrorKind::MissingChild {
            tag: crate::gimli::DW_TAG_variant_part,
        })?;

        let dw_at_discr = crate::get_attr_ref(&dw_tag_variant_part, crate::gimli::DW_AT_discr)?.ok_or(
            crate::ErrorKind::MissingAttr {
                attr: crate::gimli::DW_AT_discr,
            },
        )?;

        let dw_tag_member = unit
            .entry(dw_at_discr)
            .or(Err(crate::ErrorKind::MissingEntry {
                offset: dw_at_discr,
            }))?;

        let discr_type_offset = crate::get_type(&dw_tag_member)?;

        let location = super::Offset::from_die(unit, &dw_tag_member)?.ok_or(
            crate::ErrorKind::MissingAttr {
                attr: crate::gimli::DW_AT_data_member_location,
            },
        )?;

        Ok(Self {
            dwarf,
            unit,
            entry,
            discr_type_offset,
            name,
            location,
        })
    }

    /// The [DWARF](crate::gimli::Dwarf) sections that this `Enum`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn dwarf(&self) -> &'dwarf crate::gimli::Dwarf<R> {
        self.dwarf
    }

    /// The DWARF [unit][crate::gimli::Unit] that this `Enum`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn unit(&self) -> &crate::gimli::Unit<R, usize> {
        self.unit
    }

    /// The [debugging information entry][crate::gimli::DebuggingInformationEntry] this `Enum` abstracts over.
    #[allow(dead_code)]
    pub(crate) fn entry(&self) -> &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R> {
        &self.entry
    }

    /// The name of this type.
    pub fn name(&self) -> Result<Cow<str>, crate::gimli::Error> {
        self.name.to_string_lossy()
    }

    /// The discriminant of this type.
    pub fn discriminant_type(&self) -> Result<super::Type::<'dwarf, R>, crate::Error> {
        let entry = self.unit.entry(self.discr_type_offset)?;
        super::Type::from_die(self.dwarf, self.unit, entry)
    }

    /// The discriminant of this type.
    pub fn discriminant_location(&self) -> &super::Offset<R> {
        &self.location
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
        Ok(super::Variants::from_tree(self.dwarf, self.unit, tree))
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

        let _debug_struct = f.debug_tuple(&self.name().unwrap());
        let mut variants = self.variants().unwrap();
        let variants = variants.iter().unwrap();

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
