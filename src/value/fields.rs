/// Fields of a [struct][super::Struct] or an [enum variant][super::Variant].
/// 
/// Call [`iter`][Self::iter] to iterate over variants.
pub struct Fields<'dwarf, 'value, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Field<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'dwarf, 'value, R> Fields<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) fn new(
        schema: crate::schema::Field<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { schema, value }
    }

    /// Produces an iterator over variants.
    pub fn iter<'tree>(&'tree mut self) -> Result<FieldsIter<'dwarf, 'tree, 'value, R>, crate::Error> {
        Ok(FieldsIter {
            schema: self.schema,
            value: self.value,
        })
    }
}

/// An iterator over variants.
pub struct FieldsIter<'dwarf, 'tree, 'value, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::FieldsIter<'dwarf, 'tree, R>,
    value: crate::Bytes<'value>,
}

impl<'dwarf, 'tree, 'value, R: crate::gimli::Reader<Offset = usize>> FieldsIter<'dwarf, 'tree, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Produces the next field, if any.
    pub fn next(&mut self) -> Result<Option<super::Field<'dwarf, 'value, R>>, crate::Error> {
        let Some(next) = self.schema.next()? else { return Ok(None) };
        Some(unsafe { super::field::Field::new(next, self.value) })
    }
}
