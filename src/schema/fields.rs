/// Fields of a [struct][super::Struct] or an [enum variant][super::Variant].
///
/// Call [`iter`][Self::iter] to iterate over fields.
pub struct Fields<'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    tree: crate::gimli::EntriesTree<'dwarf, 'dwarf, R>,
}

impl<'dwarf, R> Fields<'dwarf, R>
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

    /// Produces an iterator over fields.
    pub fn iter(&mut self) -> Result<FieldsIter<'dwarf, '_, R>, crate::Error> {
        Ok(FieldsIter {
            dwarf: self.dwarf,
            unit: self.unit,
            iter: self.tree.root()?.children(),
        })
    }
}

/// An iterator over fields.
pub struct FieldsIter<'dwarf, 'tree, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    iter: crate::gimli::EntriesTreeIter<'dwarf, 'dwarf, 'tree, R>,
}

impl<'dwarf, 'tree, R: crate::gimli::Reader<Offset = usize>> FieldsIter<'dwarf, 'tree, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Produces the next field, if any.
    pub fn try_next(&mut self) -> Result<Option<super::Field<'dwarf, R>>, crate::Error> {
        loop {
            let Some(next) = self.iter.next()? else { return Ok(None) };
            let entry = next.entry();
            if entry.tag() != crate::gimli::DW_TAG_member {
                continue;
            }
            return Ok(Some(super::Field::from_dw_tag_member(
                self.dwarf,
                self.unit,
                entry.clone(),
            )?));
        }
    }
}

impl<'dwarf, 'tree, R: crate::gimli::Reader<Offset = usize>> Iterator
    for FieldsIter<'dwarf, 'tree, R>
{
    type Item = super::Field<'dwarf, R>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(next) => next,
            Err(err) => panic!("could not read next field: {err}"),
        }
    }
}
