use std::fmt;

/// A reflected [`[T; N]`][prim@array] value.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Array<'value, 'dwarf, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    value: crate::Bytes<'value>,
    schema: crate::schema::Array<'dwarf, P::Reader>,
    provider: &'dwarf P,
}

impl<'dwarf, R> crate::schema::Array<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) unsafe fn with_bytes<'value, P>(
        self,
        provider: &'dwarf P,
        value: crate::Bytes<'value>,
    ) -> Result<Array<'value, 'dwarf, P>, crate::Error>
    where
        P: crate::DebugInfoProvider<Reader = R>,
    {
        let len = self.len()?;
        let elt_size = self.elt_type()?.size()?;
        let bytes = len
            .checked_mul(elt_size)
            .ok_or(crate::error::Kind::arithmetic_overflow())?;
        let bytes = usize::try_from(bytes)?;
        let value = &value[..bytes];
        Ok(Array {
            value,
            schema: self,
            provider,
        })
    }
}

impl<'value, 'dwarf, P> Array<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// An iterator over values in the array.
    pub fn iter(&self) -> Result<super::Iter<'value, 'dwarf, P>, crate::Error> {
        let elt_type = self.schema.elt_type()?;
        let elt_size = elt_type.size()?;
        let elt_size = usize::try_from(elt_size)?;
        let length = self.schema.len()?;
        let length: usize = length.try_into()?;
        Ok(unsafe { super::Iter::new(self.value, elt_size, elt_type, length, self.provider) })
    }
}

impl<'value, 'dwarf, P> fmt::Debug for Array<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::Array");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, P> fmt::Display for Array<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_list = f.debug_list();
        for maybe_elt in self.iter().map_err(crate::fmt_err)? {
            let elt = maybe_elt.map_err(crate::fmt_err)?;
            debug_list.entry(&crate::DebugDisplay(elt));
        }
        debug_list.finish()
    }
}
