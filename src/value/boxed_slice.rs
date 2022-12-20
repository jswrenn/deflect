use std::fmt;

/// A reflected [`Box`]'d slice.
pub struct BoxedSlice<'value, 'dwarf, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    value: crate::Bytes<'value>,
    schema: crate::schema::BoxedSlice<'dwarf, P::Reader>,
    provider: &'dwarf P,
}

impl<'dwarf, R> crate::schema::BoxedSlice<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) unsafe fn with_bytes<'value, P>(
        self,
        provider: &'dwarf P,
        value: crate::Bytes<'value>,
    ) -> Result<BoxedSlice<'value, 'dwarf, P>, crate::Error>
    where
        P: crate::DebugInfoProvider<Reader = R>,
    {
        Ok(BoxedSlice {
            schema: self,
            value,
            provider,
        })
    }
}

impl<'value, 'dwarf, P> BoxedSlice<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// The schema of this value.
    pub fn schema(&self) -> &crate::schema::BoxedSlice<'dwarf, P::Reader> {
        &self.schema
    }

    /// The `data_ptr` field of this boxed slice.
    pub fn data_ptr(&self) -> Result<crate::Bytes<'value>, crate::Error> {
        let field =
            unsafe { super::Field::new(self.schema.data_ptr().clone(), self.value, self.provider) };
        let value = field.value()?;
        let value: super::Pointer<crate::schema::Mut, _> = value.try_into()?;
        let ptr = value.deref_raw()?;
        Ok(ptr)
    }

    /// The `length` field of this boxed slice.
    pub fn length(&self) -> Result<usize, crate::Error> {
        let field =
            unsafe { super::Field::new(self.schema.length().clone(), self.value, self.provider) };
        let value = field.value()?;
        let len: usize = value.try_into()?;
        Ok(len)
    }

    /// An iterator over the values in this slice.
    pub fn iter(&self) -> Result<super::Iter<'value, 'dwarf, P>, crate::Error> {
        let elt_type = self.schema.elt()?;
        let elt_size = elt_type.size()?;
        let elt_size = usize::try_from(elt_size)?;

        let length = self.length()?;
        let bytes = elt_size * length;

        let value = self.data_ptr()?.as_ptr();
        let value = std::ptr::slice_from_raw_parts(value, bytes);
        let value = unsafe { &*value };

        Ok(unsafe { super::Iter::new(value, elt_size, elt_type, length, self.provider) })
    }
}

impl<'value, 'dwarf, P> fmt::Debug for BoxedSlice<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::BoxedSlice");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, P> fmt::Display for BoxedSlice<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("box ")?;
        let mut debug_list = f.debug_list();
        for maybe_elt in self.iter().map_err(crate::fmt_err)? {
            let elt = maybe_elt.map_err(crate::fmt_err)?;
            debug_list.entry(&crate::DebugDisplay(elt));
        }
        debug_list.finish()?;
        f.write_str("[..]")
    }
}

impl<'value, 'dwarf, P> serde::Serialize for BoxedSlice<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let len = Some(self.length().map_err(crate::ser_err)?);
        let mut ser_seq = serializer.serialize_seq(len).map_err(crate::ser_err)?;
        for maybe_elt in self.iter().map_err(crate::ser_err)? {
            let elt = maybe_elt.map_err(crate::ser_err)?;
            ser_seq.serialize_element(&elt)?
        }
        ser_seq.end()
    }
}
