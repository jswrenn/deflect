/// An iterator over items in an [array][super::Array] or [slice][super::Slice].
pub struct Iter<'value, 'dwarf, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    value: crate::Bytes<'value>,
    elt_size: usize,
    elt_type: crate::schema::Type<'dwarf, P::Reader>,
    length: usize,
    provider: &'dwarf P,
}

impl<'value, 'dwarf, P> Iter<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    pub(crate) unsafe fn new(
        value: crate::Bytes<'value>,
        elt_size: usize,
        elt_type: crate::schema::Type<'dwarf, P::Reader>,
        length: usize,
        provider: &'dwarf P,
    ) -> Self {
        Self {
            value,
            elt_size,
            elt_type,
            length,
            provider,
        }
    }
}

impl<'value, 'dwarf, P> Iterator for Iter<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Item = Result<crate::Value<'value, 'dwarf, P>, crate::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            return None;
        }

        let (elt, rest) = self.value.split_at(self.elt_size);
        self.value = rest;
        self.length -= 1;

        Some(unsafe { super::Value::with_type(self.elt_type.clone(), elt, self.provider) })
    }
}
