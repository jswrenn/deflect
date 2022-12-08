use std::fmt;

#[allow(non_camel_case_types)]
pub struct str<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    value: &'value std::primitive::str,
    schema: crate::schema::str<'dwarf, P::Reader>,
    provider: &'dwarf P,
}

impl<'value, 'dwarf, P> str<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::str<'dwarf, P::Reader>,
        provider: &'dwarf P,
    ) -> Result<Self, crate::Error> {
        let data_ptr = unsafe { super::Field::new(schema.data_ptr().clone(), value, provider) };
        let data_ptr = data_ptr.value()?;
        let data_ptr: super::Pointer<crate::schema::Mut, _> = data_ptr.try_into()?;
        let data = data_ptr.deref_raw()?.as_ptr();

        let length = unsafe { super::Field::new(schema.length().clone(), value, provider) };
        let length = length.value()?;
        let length = length.try_into()?;

        let value = std::ptr::slice_from_raw_parts(data, length);
        let value = unsafe { &*(value as *const std::primitive::str) };

        Ok(Self {
            value,
            schema,
            provider,
        })
    }

    pub fn value(&self) -> &'value std::primitive::str {
        self.value
    }
}

impl<'value, 'dwarf, P> fmt::Debug for str<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::Slice");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, P> fmt::Display for str<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.value(), f)
    }
}

impl<'value, 'dwarf, P> From<str<'value, 'dwarf, P>> for super::Value<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn from(value: str<'value, 'dwarf, P>) -> Self {
        super::Value::str(value)
    }
}

impl<'value, 'dwarf, P> From<str<'value, 'dwarf, P>> for &'value std::primitive::str
where
    P: crate::DebugInfoProvider,
{
    fn from(atom: str<'value, 'dwarf, P>) -> Self {
        atom.value()
    }
}

impl<'a, 'value, 'dwarf, P> TryFrom<&'a super::Value<'value, 'dwarf, P>>
    for &'a str<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a super::Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let super::Value::str(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                &'a super::Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}

impl<'value, 'dwarf, P> TryFrom<super::Value<'value, 'dwarf, P>> for str<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: super::Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let super::Value::str(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                super::Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}

impl<'a, 'value, 'dwarf, P> TryFrom<&'a super::Value<'value, 'dwarf, P>>
    for &'value std::primitive::str
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a super::Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let super::Value::str(value) = value {
            Ok(value.value())
        } else {
            Err(crate::error::Downcast::new::<
                &'a super::Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}

impl<'value, 'dwarf, P> TryFrom<super::Value<'value, 'dwarf, P>> for &'value std::primitive::str
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: super::Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let super::Value::str(value) = value {
            Ok(value.value())
        } else {
            Err(crate::error::Downcast::new::<
                super::Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}
