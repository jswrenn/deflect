use super::Name;
use std::fmt;

/// A Rust-like `struct`.
#[derive(Clone)]
pub struct Slice<'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    r#struct: super::Struct<'dwarf, R>,
    data_ptr: super::Field<'dwarf, R>,
    length: super::Field<'dwarf, R>,
}

impl<'dwarf, R> Slice<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Construct a new `Struct` from a [`DW_TAG_structure_type`][crate::gimli::DW_TAG_structure_type].
    pub(crate) fn from_dw_tag_structure_type(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::err::Error> {
        crate::check_tag(&entry, crate::gimli::DW_TAG_structure_type)?;

        let name = Name::from_die(dwarf, unit, &entry)?;

        let name = name.to_slice()?;
        if !(name.starts_with(b"&[") && name.ends_with(b"]")) {
            let actual = String::from_utf8_lossy(name.as_ref()).to_string();
            Err(crate::err::Kind::name_mismatch("&[T]", actual))?;
        };

        let r#struct = super::Struct::from_dw_tag_structure_type(dwarf, unit, entry)?;
        let mut fields = r#struct.fields()?;
        let mut fields = fields.iter()?;

        let data_ptr = fields.try_next()?.unwrap();
        let length = fields.try_next()?.unwrap();

        Ok(Self {
            r#struct,
            data_ptr,
            length,
        })
    }

    /// The [DWARF](crate::gimli::Dwarf) sections that this `Struct`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn dwarf(&self) -> &'dwarf crate::gimli::Dwarf<R> {
        self.r#struct.dwarf()
    }

    /// The DWARF [unit][crate::gimli::Unit] that this `Struct`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn unit(&self) -> &crate::gimli::Unit<R, usize> {
        self.r#struct.unit()
    }

    /// The [debugging information entry][crate::gimli::DebuggingInformationEntry] this `Struct` abstracts over.
    #[allow(dead_code)]
    pub(crate) fn entry(&self) -> &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R> {
        &self.r#struct.entry()
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
    pub fn elt(&self) -> Result<super::Type<'dwarf, R>, crate::err::Error> {
        if let super::Type::MutPtr(r#ref) = self.data_ptr().r#type()? {
            return r#ref.r#type();
        }
        panic!()
    }

    /// The size of this slice, in bytes.
    pub fn size(&self) -> Result<u64, crate::err::Error> {
        Ok(crate::get_size(self.entry())?)
    }

    /// The alignment of this slice, in bytes.
    pub fn align(&self) -> Result<Option<u64>, crate::err::Error> {
        Ok(crate::get_align(self.entry())?)
    }
}

impl<'dwarf, R> fmt::Debug for Slice<'dwarf, R>
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

impl<'value, 'dwarf, R> fmt::Display for Slice<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
