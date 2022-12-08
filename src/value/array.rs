use std::fmt;

/// A reflected [`[T; N]`][prim@array] value.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Array<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    value: crate::Bytes<'value>,
    schema: crate::schema::Array<'dwarf, P::Reader>,
    provider: &'dwarf P,
}

impl<'value, 'dwarf, P> Array<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::Array<'dwarf, P::Reader>,
        provider: &'dwarf P,
    ) -> Result<Self, crate::Error> {
        let len = schema.len()?;
        let elt_size = schema.elt_type()?.size()?;
        let bytes = len.checked_mul(elt_size).ok_or(crate::error::Kind::Other)?;
        let bytes = usize::try_from(bytes)?;
        let value = &value[..bytes];
        Ok(Self {
            value,
            schema,
            provider,
        })
    }

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

/// Attempt to downcast a `Value<'value, 'dwarf, P>` into a `Array<'value, 'dwarf, P>`.
impl<'value, 'dwarf, P> TryFrom<super::Value<'value, 'dwarf, P>> for Array<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: super::Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let super::Value::Array(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<Array<'value, 'dwarf, P>, Self>())
        }
    }
}
