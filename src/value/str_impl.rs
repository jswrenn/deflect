use std::fmt;

/// A reflected `&str` value.
#[allow(non_camel_case_types)]
pub struct str<'value, 'dwarf, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    value: &'value std::primitive::str,
    schema: crate::schema::str<'dwarf, P::Reader>,
    _provider: &'dwarf P,
}

impl<'dwarf, R> crate::schema::str<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) unsafe fn with_bytes<'value, P>(
        self,
        provider: &'dwarf P,
        value: crate::Bytes<'value>,
    ) -> Result<str<'value, 'dwarf, P>, crate::Error>
    where
        P: crate::DebugInfoProvider<Reader = R>,
    {
        let data_ptr = unsafe { super::Field::new(self.data_ptr().clone(), value, provider) };
        let data_ptr = data_ptr.value()?;
        let data_ptr: super::Pointer<crate::schema::Mut, _> = data_ptr.try_into()?;
        let data = data_ptr.deref_raw()?.as_ptr();

        let length = unsafe { super::Field::new(self.length().clone(), value, provider) };
        let length = length.value()?;
        let length = length.try_into()?;

        let value = std::ptr::slice_from_raw_parts(data, length);
        let value = unsafe { &*(value as *const std::primitive::str) };

        Ok(str {
            value,
            schema: self,
            _provider: provider,
        })
    }
}

impl<'value, 'dwarf, P> str<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// The schema of this value.
    pub fn schema(&self) -> &crate::schema::str<'dwarf, P::Reader> {
        &self.schema
    }

    /// The Rust value corresponding to this reflected value.
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

impl<'value, 'dwarf, P> From<str<'value, 'dwarf, P>> for &'value std::primitive::str
where
    P: crate::DebugInfoProvider,
{
    fn from(atom: str<'value, 'dwarf, P>) -> Self {
        atom.value()
    }
}

impl<'a, 'value, 'dwarf, P> TryFrom<&'a super::Value<'value, 'dwarf, P>>
    for &'value std::primitive::str
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::DowncastErr;

    fn try_from(value: &'a super::Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let super::Value::str(value) = value {
            Ok(value.value())
        } else {
            Err(crate::error::DowncastErr::new::<
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
    type Error = crate::error::DowncastErr;

    fn try_from(value: super::Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let super::Value::str(value) = value {
            Ok(value.value())
        } else {
            Err(crate::error::DowncastErr::new::<
                super::Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}
