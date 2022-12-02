use std::fmt;

pub struct Slice<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    value: crate::Bytes<'value>,
    schema: crate::schema::Slice<'dwarf, R>,
}

impl<'value, 'dwarf, R> Slice<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::Slice<'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        Ok(Self { value, schema })
    }

    pub fn data_ptr(&self) -> Result<crate::Bytes<'value>, crate::Error> {
        let field = unsafe { super::Field::new(self.schema.data_ptr().clone(), self.value) };
        let value = field.value()?;
        let value: super::Pointer<crate::schema::Mut, _> = value.try_into().unwrap();
        let ptr = value.deref_raw()?;
        Ok(ptr)
    }

    pub fn len(&self) -> Result<usize, crate::Error> {
        let field = unsafe { super::Field::new(self.schema.length().clone(), self.value) };
        let value = field.value()?;
        let len: usize = value.try_into()?;
        Ok(len)
    }

    pub fn iter(&self) -> Result<super::Iter<'value, 'dwarf, R>, crate::Error> {
        let elt_type = self.schema.elt()?;
        let elt_size = elt_type.size()?;
        let elt_size = usize::try_from(elt_size)?;

        let length = self.len()?;
        let bytes = elt_size * length;

        let value = self.data_ptr()?.as_ptr();
        let value = std::ptr::slice_from_raw_parts(value, bytes);
        let value = unsafe { &*value };

        Ok(unsafe { super::Iter::new(value, elt_size, elt_type, length) })
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

impl<'value, 'dwarf, R> From<Slice<'value, 'dwarf, R>> for super::Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn from(value: Slice<'value, 'dwarf, R>) -> Self {
        super::Value::Slice(value)
    }
}

impl<'a, 'value, 'dwarf, R> TryFrom<&'a super::Value<'value, 'dwarf, R>>
    for &'a Slice<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a super::Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let super::Value::Slice(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                &'a super::Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}

impl<'value, 'dwarf, R> TryFrom<super::Value<'value, 'dwarf, R>> for Slice<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: super::Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let super::Value::Slice(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                super::Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}
