/// Variants of an [enum][super::Enum].
///
/// Call [`iter`][Self::iter] to iterate over variants.
pub struct Variants<'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    tree: crate::gimli::EntriesTree<'dwarf, 'dwarf, R>,
    discriminant_type: super::Type<'dwarf, R>,
}

impl<'dwarf, R> Variants<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) fn from_tree(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        tree: crate::gimli::EntriesTree<'dwarf, 'dwarf, R>,
        discriminant_type: super::Type<'dwarf, R>,
    ) -> Self {
        Self {
            dwarf,
            unit,
            tree,
            discriminant_type,
        }
    }

    /// Produces an iterator over variants.
    pub fn iter(&mut self) -> Result<VariantsIter<'dwarf, '_, R>, crate::Error> {
        Ok(VariantsIter {
            dwarf: self.dwarf,
            unit: self.unit,
            iter: self.tree.root()?.children(),
            discriminant_type: &self.discriminant_type,
        })
    }
}

/// An iterator over variants.
pub struct VariantsIter<'dwarf, 'tree, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    iter: crate::gimli::EntriesTreeIter<'dwarf, 'dwarf, 'tree, R>,
    discriminant_type: &'tree super::Type<'dwarf, R>,
}

impl<'dwarf, 'tree, R: crate::gimli::Reader<Offset = usize>> VariantsIter<'dwarf, 'tree, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Produces the next variant, if any.
    pub fn try_next(&mut self) -> Result<Option<super::Variant<'dwarf, R>>, crate::Error> {
        loop {
            let Some(next) = self.iter.next()? else { return Ok(None) };
            let entry = next.entry();
            match entry.tag() {
                crate::gimli::DW_TAG_variant => {
                    let discriminant_value = entry
                        .attr_value(crate::gimli::DW_AT_discr_value)?
                        .and_then(|dw_at_discr_value| dw_at_discr_value.udata_value())
                        .map(|dw_at_discr_value| {
                            discriminant_value(self.discriminant_type, dw_at_discr_value)
                        });

                    let mut entry = next.children();
                    let entry = entry.next()?;
                    let entry = entry
                        .ok_or_else(|| crate::error::missing_child(crate::gimli::DW_TAG_member))?;
                    let entry = crate::get_type(entry.entry())?;
                    let entry = self.unit.entry(entry)?;
                    return Ok(Some(super::Variant::new(
                        self.dwarf,
                        self.unit,
                        entry,
                        discriminant_value,
                    )));
                }
                crate::gimli::DW_TAG_enumerator => {
                    let discriminant_value = entry
                        .attr_value(crate::gimli::DW_AT_discr_value)?
                        .and_then(|dw_at_discr_value| dw_at_discr_value.udata_value())
                        .map(|dw_at_discr_value| {
                            discriminant_value(self.discriminant_type, dw_at_discr_value)
                        });

                    return Ok(Some(super::Variant::new(
                        self.dwarf,
                        self.unit,
                        entry.clone(),
                        discriminant_value,
                    )));
                }
                crate::gimli::DW_TAG_member => continue,
                other => {
                    anyhow::bail!(
                        "Cannot find discriminant value in {:?} at {:x?}",
                        other.static_string(),
                        entry.offset()
                    );
                }
            }
        }
    }
}

impl<'dwarf, 'tree, R: crate::gimli::Reader<Offset = usize>> Iterator
    for VariantsIter<'dwarf, 'tree, R>
{
    type Item = super::Variant<'dwarf, R>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(next) => next,
            Err(err) => panic!("could not read next variant: {err}"),
        }
    }
}

fn discriminant_value<'dwarf, R>(ty: &super::Type<'dwarf, R>, v: u64) -> super::Data
where
    R: crate::gimli::Reader<Offset = usize>,
{
    match ty {
        super::Type::u8(_) => super::Data::u8(v as _),
        super::Type::u16(_) => super::Data::u16(v as _),
        super::Type::u32(_) => super::Data::u32(v as _),
        super::Type::u64(_) => super::Data::u64(v as _),
        _ => unimplemented!(),
    }
}
