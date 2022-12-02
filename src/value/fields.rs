/// Fields of a [struct][super::Struct] or an [enum variant][super::Variant].
///
/// Call [`iter`][Self::iter] to iterate over variants.
pub struct Fields<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Fields<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'value, 'dwarf, R> Fields<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) fn new(
        schema: crate::schema::Fields<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { schema, value }
    }

    /// Produces an iterator over variants.
    pub fn iter<'tree>(
        &'tree mut self,
    ) -> Result<FieldsIter<'value, 'tree, 'dwarf, R>, crate::Error> {
        Ok(FieldsIter {
            schema: self.schema.iter()?,
            value: self.value,
        })
    }
}

/// An iterator over variants.
pub struct FieldsIter<'value, 'tree, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::FieldsIter<'dwarf, 'tree, R>,
    value: crate::Bytes<'value>,
}

impl<'value, 'tree, 'dwarf, R> FieldsIter<'value, 'tree, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Produces the next field, if any.
    pub fn try_next(
        &mut self,
    ) -> Result<Option<super::Field<'value, 'dwarf, R>>, crate::Error> {
        let Some(next) = self.schema.try_next()? else { return Ok(None) };
        Ok(Some(unsafe { super::field::Field::new(next, self.value) }))
    }
}

impl<'value, 'tree, 'dwarf, R> Iterator for FieldsIter<'value, 'tree, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Item = super::Field<'value, 'dwarf, R>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(next) => next,
            Err(err) => panic!("could not read next field: {}", err),
        }
    }
}
