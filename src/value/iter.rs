/// An iterator over items in an [array][super::Array] or [slice][super::Slice].
pub struct Iter<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    value: crate::Bytes<'value>,
    elt_size: usize,
    elt_type: crate::schema::Type<'dwarf, R>,
    length: usize,
}

impl<'value, 'dwarf, R> Iter<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub unsafe fn new(
        value: crate::Bytes<'value>,
        elt_size: usize,
        elt_type: crate::schema::Type<'dwarf, R>,
        length: usize,
    ) -> Self {
        Self {
            value,
            elt_size,
            elt_type,
            length,
        }
    }
}

impl<'value, 'dwarf, R> Iterator for Iter<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Item = Result<crate::Value<'value, 'dwarf, R>, crate::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            return None;
        }

        let (elt, rest) = self.value.split_at(self.elt_size);
        self.value = rest;
        self.length -= 1;

        Some(unsafe { super::Value::with_type(self.elt_type.clone(), elt) })
    }
}
