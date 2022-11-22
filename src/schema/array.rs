use std::fmt;

/// An array type.
#[derive(Clone)]
pub struct Array<'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
}

impl<'value, 'dwarf, R> Array<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Construct a new `Function` from a [`DW_TAG_subroutine_type`][crate::gimli::DW_TAG_subroutine_type].
    pub(crate) fn from_dw_tag_subroutine_type(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::err::Error> {
        crate::check_tag(&entry, crate::gimli::DW_TAG_array_type)?;
        Ok(Self { dwarf, unit, entry })
    }

    /// The [DWARF](crate::gimli::Dwarf) sections that this `Function`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn dwarf(&self) -> &'dwarf crate::gimli::Dwarf<R> {
        self.dwarf
    }

    /// The DWARF [unit][crate::gimli::Unit] that this `Function`'s debuginfo belongs to.
    #[allow(dead_code)]
    pub(crate) fn unit(&self) -> &crate::gimli::Unit<R, usize> {
        self.unit
    }

    /// The [debugging information entry][crate::gimli::DebuggingInformationEntry] this `Function` abstracts over.
    #[allow(dead_code)]
    pub(crate) fn entry(&self) -> &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R> {
        &self.entry
    }

    /// The element type of this array.
    pub fn elt(&self) -> Result<super::Type<'dwarf, R>, crate::err::Error> {
        super::Type::from_die(
            self.dwarf,
            self.unit,
            self.unit.entry(crate::get_type(&self.entry)?)?,
        )
    }

    /// The length of this array.
    pub fn len(&self) -> Result<u64, crate::err::Error> {
        let mut tree = self.unit.entries_tree(Some(self.entry.offset()))?;
        let root = tree.root()?;
        let mut children = root.children();
        let dw_tag_subrange_type = children
            .next()?
            .ok_or(crate::err::ErrorKind::missing_child(
                crate::gimli::DW_TAG_subrange_type,
            ))?;
        let dw_tag_subrange_type = dw_tag_subrange_type.entry();
        crate::check_tag(dw_tag_subrange_type, crate::gimli::DW_TAG_subrange_type)?;
        let dw_at_count = crate::get(dw_tag_subrange_type, crate::gimli::DW_AT_count)?;
        let count = dw_at_count
            .udata_value()
            .ok_or(crate::err::ErrorKind::invalid_attr(
                crate::gimli::DW_AT_count,
            ))?;
        Ok(count)
    }

    /// The size of this array, in bytes.
    pub fn size(&self) -> Result<u64, crate::err::Error> {
        let len = self.len()?;
        let elt_size = self.elt()?.size()?;
        Ok(len.checked_mul(elt_size).expect("Computing the size (in bytes) of this slice overflowed when multiplying the length ({len}) by the element size ({elt_size})."))
    }
}

impl<'dwarf, R> fmt::Debug for Array<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple = f.debug_tuple("deflect::schema::Array");
        debug_tuple.field(&crate::debug::DebugEntry::new(
            self.dwarf,
            self.unit,
            &self.entry,
        ));
        debug_tuple.finish()
    }
}

impl<'value, 'dwarf, R> fmt::Display for Array<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*// TODO: Can we get back function names?
        if let Some(name) = self.name().expect("gimli err") {
            let name = name.to_string_lossy().expect("gimli err");
            write!(f, "fn {name}<todo>(todo) -> (todo)")
        } else {
            write!(f, "fn ()<todo>(todo) -> (todo)")
        }*/
        Ok(())
    }
}
