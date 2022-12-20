use std::fmt;

/// A reflected slice value.
pub struct Slice<'value, 'dwarf, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    value: crate::Bytes<'value>,
    schema: crate::schema::Slice<'dwarf, P::Reader>,
    provider: &'dwarf P,
}

impl<'dwarf, R> crate::schema::Slice<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) unsafe fn with_bytes<'value, P>(
        self,
        provider: &'dwarf P,
        bytes: crate::Bytes<'value>,
    ) -> Result<Slice<'value, 'dwarf, P>, crate::Error>
    where
        P: crate::DebugInfoProvider<Reader = R>,
    {
        Ok(Slice {
            value: bytes,
            schema: self,
            provider,
        })
    }
}

impl<'value, 'dwarf, P> Slice<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// The schema of this value.
    pub fn schema(&self) -> &crate::schema::Slice<'dwarf, P::Reader> {
        &self.schema
    }

    /// The value of the `data_ptr` field of this slice.
    pub fn data_ptr(&self) -> Result<crate::Bytes<'value>, crate::Error> {
        let field =
            unsafe { super::Field::new(self.schema.data_ptr().clone(), self.value, self.provider) };
        let value = field.value()?;
        let value: super::Pointer<crate::schema::Mut, _> = value.try_into()?;
        let ptr = value.deref_raw()?;
        Ok(ptr)
    }

    /// The value of the `length` field of this slice.
    pub fn length(&self) -> Result<usize, crate::Error> {
        let field =
            unsafe { super::Field::new(self.schema.length().clone(), self.value, self.provider) };
        let value = field.value()?;
        let len: usize = value.try_into()?;
        Ok(len)
    }

    /// An iterator over values of this slice.
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

impl<'value, 'dwarf, P> fmt::Debug for Slice<'value, 'dwarf, P>
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

impl<'value, 'dwarf, P> fmt::Display for Slice<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
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

impl<'value, 'dwarf, P> serde::Serialize for Slice<'value, 'dwarf, P>
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
            ser_seq.serialize_element(&elt)?;
        }
        ser_seq.end()
    }
}
