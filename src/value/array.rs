use std::fmt;

/// A reflected [`[T; N]`][prim@array] value.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Array<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    value: crate::Bytes<'value>,
    schema: crate::schema::Array<'dwarf, R>,
}

impl<'value, 'dwarf, R> Array<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::Array<'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        let len = schema.len()?;
        let elt_size = schema.elt_type()?.size()?;
        let bytes = len.checked_mul(elt_size).ok_or(crate::error::Kind::Other)?;
        let bytes = usize::try_from(bytes)?;
        let value = &value[..bytes];
        Ok(Self { value, schema })
    }

    pub fn iter(&self) -> Result<super::Iter<'value, 'dwarf, R>, crate::Error> {
        let elt_type = self.schema.elt_type()?;
        let elt_size = elt_type.size()?;
        let elt_size = usize::try_from(elt_size)?;
        let length = self.schema.len()?;
        let length: usize = length.try_into()?;
        Ok(unsafe { super::Iter::new(self.value, elt_size, elt_type, length) })
    }
}

impl<'value, 'dwarf, R> fmt::Debug for Array<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::Array");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, R> fmt::Display for Array<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
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
