use std::fmt;

/// A schema for `Box<[T]>`.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BoxedSlice<'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: super::Struct<'dwarf, R>,
    data_ptr: super::Field<'dwarf, R>,
    length: super::Field<'dwarf, R>,
}

impl<'dwarf, R> BoxedSlice<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Construct a new `BoxedSlice`.
    pub(crate) fn new(
        schema: super::Struct<'dwarf, R>,
        data_ptr: super::Field<'dwarf, R>,
        length: super::Field<'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            schema,
            data_ptr,
            length,
        })
    }

    /// The [DWARF](crate::gimli::Dwarf) sections that this `Struct`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn dwarf(&self) -> &'dwarf crate::gimli::Dwarf<R> {
        self.schema.dwarf()
    }

    /// The DWARF [unit][crate::gimli::Unit] that this `Struct`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn unit(&self) -> &crate::gimli::Unit<R, usize> {
        self.schema.unit()
    }

    /// The [debugging information entry][crate::gimli::DebuggingInformationEntry] this `Struct` abstracts over.
    #[allow(dead_code)]
    pub(crate) fn entry(&self) -> &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R> {
        self.schema.entry()
    }

    /// The `data_ptr` field of this slice.
    pub fn data_ptr(&self) -> super::Field<'dwarf, R> {
        self.data_ptr.clone()
    }

    /// The `length` field of this slice.
    pub fn length(&self) -> &super::Field<'dwarf, R> {
        &self.length
    }

    /// The element type of this slice.
    pub fn elt(&self) -> Result<super::Type<'dwarf, R>, crate::Error> {
        if let super::Type::MutPtr(r#ref) = self.data_ptr().r#type()? {
            return r#ref.r#type();
        } else {
            unreachable!()
        }
    }

    /// The size of this slice, in bytes.
    pub fn size(&self) -> Result<u64, crate::Error> {
        Ok(crate::get_size(self.entry())?)
    }

    /// The alignment of this slice, in bytes.
    pub fn align(&self) -> Result<Option<u64>, crate::Error> {
        Ok(crate::get_align(self.entry())?)
    }
}

impl<'dwarf, R> fmt::Debug for BoxedSlice<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple = f.debug_tuple("deflect::schema::Slice");
        debug_tuple.field(&crate::debug::DebugEntry::new(
            self.dwarf(),
            self.unit(),
            self.entry(),
        ));
        debug_tuple.finish()
    }
}

impl<'dwarf, R> fmt::Display for BoxedSlice<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
