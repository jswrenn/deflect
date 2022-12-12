use std::fmt;

/// A schema for `Box<dyn Trait>`.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BoxedDyn<'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: super::Struct<'dwarf, R>,
    pointer: super::Field<'dwarf, R>,
    vtable: super::Field<'dwarf, R>,
}

impl<'dwarf, R> BoxedDyn<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Construct a new `BoxedSlice`.
    pub(crate) fn new(
        schema: super::Struct<'dwarf, R>,
        pointer: super::Field<'dwarf, R>,
        vtable: super::Field<'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            schema,
            pointer,
            vtable,
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

    /// This fat pointer, interpreted as a struct.
    pub fn as_struct(&self) -> &super::Struct<'dwarf, R> {
        &self.schema
    }

    /// The `pointer` field of this slice.
    pub fn pointer(&self) -> &super::Field<'dwarf, R> {
        &self.pointer
    }

    /// The `vtable` field of this slice.
    pub fn vtable(&self) -> &super::Field<'dwarf, R> {
        &self.vtable
    }

    /// The size of this fat pointer, in bytes.
    pub fn size(&self) -> Result<u64, crate::Error> {
        crate::get_size(self.entry())
    }

    /// The alignment of fat pointer, in bytes.
    pub fn align(&self) -> Result<Option<u64>, crate::Error> {
        crate::get_align(self.entry())
    }
}

impl<'dwarf, R> fmt::Debug for BoxedDyn<'dwarf, R>
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

impl<'dwarf, R> fmt::Display for BoxedDyn<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
