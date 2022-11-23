use std::fmt;

pub struct Slice<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    value: crate::Bytes<'value>,
    schema: crate::schema::slice<'dwarf, R>,
}

impl<'value, 'dwarf, R> Slice<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::slice<'dwarf, R>,
    ) -> Result<Self, crate::error::Error> {
        Ok(Self { value, schema })
    }

    pub fn data_ptr(&self) -> Result<crate::Bytes<'value>, crate::error::Error> {
        let field = unsafe { super::Field::new(self.schema.data_ptr().clone(), self.value) };
        let value = field.value()?;
        let value: super::Pointer<crate::schema::Mut, _> = value.try_into().unwrap();
        let ptr = value.deref_raw()?;
        Ok(ptr)
    }

    pub fn len(&self) -> Result<usize, crate::error::Error> {
        let field = unsafe { super::Field::new(self.schema.length().clone(), self.value) };
        let value = field.value()?;
        let len: usize = value.try_into()?;
        Ok(len)
    }

    pub fn iter(
        &self,
    ) -> Result<
        impl Iterator<Item = Result<super::Value<'value, 'dwarf, R>, crate::error::Error>>,
        crate::error::Error,
    > {
        let elt_type = self.schema.elt()?;
        let elt_size = elt_type.size()?;
        let elt_size = usize::try_from(elt_size)?;

        let len = self.len()?;
        let bytes = elt_size * len;

        let value = self.data_ptr()?;
        let value = unsafe { &*std::ptr::slice_from_raw_parts(value.as_ptr(), bytes) };

        Ok(value
            .chunks(elt_size)
            .take(len)
            .map(move |chunk| unsafe { super::Value::with_type(elt_type.clone(), chunk) }))
    }
}

impl<'value, 'dwarf, R> fmt::Debug for Slice<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::Slice");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, R> fmt::Display for Slice<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("&")?;
        let mut debug_list = f.debug_list();
        for maybe_elt in self.iter().map_err(crate::fmt_err)? {
            let elt = maybe_elt.map_err(crate::fmt_err)?;
            debug_list.entry(&crate::DebugDisplay(elt));
        }
        debug_list.finish()
    }
}
