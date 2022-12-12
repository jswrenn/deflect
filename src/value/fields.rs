/// Fields of a [struct][super::Struct] or an [enum variant][super::Variant].
///
/// Call [`iter`][Self::iter] to iterate over variants.
pub struct Fields<'value, 'dwarf, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    schema: crate::schema::Fields<'dwarf, P::Reader>,
    value: crate::Bytes<'value>,
    provider: &'dwarf P,
}

impl<'value, 'dwarf, P> Fields<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    pub(crate) fn new(
        schema: crate::schema::Fields<'dwarf, P::Reader>,
        value: crate::Bytes<'value>,
        provider: &'dwarf P,
    ) -> Self {
        Self {
            schema,
            value,
            provider,
        }
    }

    /// Produces an iterator over variants.
    pub fn iter<'tree>(
        &'tree mut self,
    ) -> Result<FieldsIter<'value, 'tree, 'dwarf, P>, crate::Error> {
        Ok(FieldsIter {
            schema: self.schema.iter()?,
            value: self.value,
            provider: self.provider,
        })
    }
}

/// An iterator over variants.
pub struct FieldsIter<'value, 'tree, 'dwarf, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    schema: crate::schema::FieldsIter<'dwarf, 'tree, P::Reader>,
    value: crate::Bytes<'value>,
    provider: &'dwarf P,
}

impl<'value, 'tree, 'dwarf, P> FieldsIter<'value, 'tree, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// Produces the next field, if any.
    pub fn try_next(&mut self) -> Result<Option<super::Field<'value, 'dwarf, P>>, crate::Error> {
        let Some(next) = self.schema.try_next()? else { return Ok(None) };
        Ok(Some(unsafe {
            super::field::Field::new(next, self.value, self.provider)
        }))
    }
}

impl<'value, 'tree, 'dwarf, P> Iterator for FieldsIter<'value, 'tree, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Item = super::Field<'value, 'dwarf, P>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(next) => next,
            Err(err) => panic!("could not read next field: {}", err),
        }
    }
}
