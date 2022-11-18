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
}

impl<'dwarf, R> Variants<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) fn from_tree(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        tree: crate::gimli::EntriesTree<'dwarf, 'dwarf, R>,
    ) -> Self {
        Self { dwarf, unit, tree }
    }

    /// Produces an iterator over variants.
    pub fn iter(&mut self) -> Result<VariantsIter<'dwarf, '_, R>, crate::Error> {
        Ok(VariantsIter {
            dwarf: self.dwarf,
            unit: self.unit,
            iter: self.tree.root()?.children(),
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
                        .map(|value| value.into());

                    let mut entry = next.children();
                    let entry = entry.next()?.ok_or(crate::ErrorKind::MissingChild {
                        tag: crate::gimli::DW_TAG_member,
                    })?;
                    let entry = self.unit.entry(crate::get_type(entry.entry())?)?;
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
                        .map(|value| value.into());

                    return Ok(Some(super::Variant::new(
                        self.dwarf,
                        self.unit,
                        entry.clone(),
                        discriminant_value,
                    )));
                }
                crate::gimli::DW_TAG_member => continue,
                other => panic!(
                    "Cannot find discriminant value in {:?} at {:x?}",
                    other.static_string(),
                    entry.offset()
                ),
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
            Err(err) => panic!("could not read next variant: {}", err),
        }
    }
}
