use std::fmt;

pub struct Ref<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
}

impl<'dwarf, R> Ref<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Construct a new `Ref` from a [`DW_TAG_pointer_type`][crate::gimli::DW_TAG_pointer_type].
    pub(crate) fn from_dw_pointer_type(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Self {
        assert_eq!(entry.tag(), crate::gimli::DW_TAG_pointer_type);
        Self { dwarf, unit, entry }
    }

    /// The [DWARF](crate::gimli::Dwarf) sections that this `Ref`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn dwarf(&self) -> &'dwarf crate::gimli::Dwarf<R> {
        self.dwarf
    }

    /// The DWARF [unit][crate::gimli::Unit] that this `Ref`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn unit(&self) -> &crate::gimli::Unit<R, usize> {
        self.unit
    }

    /// The [debugging information entry][crate::gimli::DebuggingInformationEntry] this `Ref` abstracts over.
    #[allow(dead_code)]
    pub(crate) fn entry(&self) -> &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R> {
        &self.entry
    }

    /// The name of this primitive type.
    pub fn name(&self) -> Result<Option<super::Name<R>>, crate::Error> {
        Ok(super::Name::from_die(
            self.dwarf(),
            self.unit(),
            self.entry(),
        )?)
    }

    /// The size of this type, in bytes.
    pub fn size(&self) -> Result<Option<u64>, crate::Error> {
        Ok(crate::get_size(self.entry())?)
    }

    /// The type of the referent.
    pub fn r#type(&'dwarf self) -> Result<Option<super::Type<'dwarf, R>>, crate::Error> {
        let maybe_type = crate::get_type_opt(self.unit(), self.entry())?;
        Ok(if let Some(r#type) = maybe_type {
            Some(super::Type::from_die(self.dwarf, self.unit, r#type)?)
        } else {
            None
        })
    }
}

impl<'dwarf, R> fmt::Debug for Ref<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple = f.debug_tuple("deflect::schema::Ref");
        debug_tuple.field(&crate::debug::DebugEntry::new(
            self.dwarf,
            self.unit,
            &self.entry,
        ));
        debug_tuple.finish()
    }
}

impl<'value, 'dwarf, R> fmt::Display for Ref<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Ok(Some(type_name)) => type_name.fmt(f),
            Ok(None) => panic!("type does not have a name"),
            Err(err) => panic!("reader error: {:?}", err),
        }
    }
}
