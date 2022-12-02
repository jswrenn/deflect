use std::fmt;

#[allow(non_camel_case_types)]
pub struct str<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    value: &'value std::primitive::str,
    schema: crate::schema::str<'dwarf, R>,
}

impl<'value, 'dwarf, R> str<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::str<'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        let data_ptr = unsafe { super::Field::new(schema.data_ptr().clone(), value) };
        let data_ptr = data_ptr.value()?;
        let data_ptr: super::Pointer<crate::schema::Mut, _> = data_ptr.try_into()?;
        let data = data_ptr.deref_raw()?.as_ptr();

        let length = unsafe { super::Field::new(schema.length().clone(), value) };
        let length = length.value()?;
        let length = length.try_into()?;

        let value = std::ptr::slice_from_raw_parts(data, length);
        let value = unsafe { &*(value as *const std::primitive::str) };

        Ok(Self { value, schema })
    }

    pub fn value(&self) -> &'value std::primitive::str {
        self.value
    }
}

impl<'value, 'dwarf, R> fmt::Debug for str<'value, 'dwarf, R>
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

impl<'value, 'dwarf, R> fmt::Display for str<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.value(), f)
    }
}

impl<'value, 'dwarf, R> From<str<'value, 'dwarf, R>> for super::Value<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn from(value: str<'value, 'dwarf, R>) -> Self {
        super::Value::str(value)
    }
}

impl<'value, 'dwarf, R> From<str<'value, 'dwarf, R>> for &'value std::primitive::str
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn from(atom: str<'value, 'dwarf, R>) -> Self {
        atom.value()
    }
}

impl<'a, 'value, 'dwarf, R> TryFrom<&'a super::Value<'value, 'dwarf, R>>
    for &'a str<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a super::Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let super::Value::str(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                &'a super::Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}

impl<'value, 'dwarf, R> TryFrom<super::Value<'value, 'dwarf, R>> for str<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: super::Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let super::Value::str(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                super::Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}

impl<'a, 'value, 'dwarf, R> TryFrom<&'a super::Value<'value, 'dwarf, R>>
    for &'value std::primitive::str
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a super::Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let super::Value::str(value) = value {
            Ok(value.value())
        } else {
            Err(crate::error::Downcast::new::<
                &'a super::Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}

impl<'value, 'dwarf, R> TryFrom<super::Value<'value, 'dwarf, R>> for &'value std::primitive::str
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    type Error = crate::error::Downcast;

    fn try_from(value: super::Value<'value, 'dwarf, R>) -> Result<Self, Self::Error> {
        if let super::Value::str(value) = value {
            Ok(value.value())
        } else {
            Err(crate::error::Downcast::new::<
                super::Value<'value, 'dwarf, R>,
                Self,
            >())
        }
    }
}
