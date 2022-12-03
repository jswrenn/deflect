use std::fmt;

/// A schema for a shared reference (i.e., `&T`).
#[derive(Clone)]
pub struct Box<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: crate::gimli::UnitOffset,
    name: Option<super::Name<R>>,
    target: crate::gimli::UnitOffset,
}

impl<'dwarf, R> Box<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    /// Construct a new `Shared`.
    pub(super) fn new(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::UnitOffset,
        name: Option<super::Name<R>>,
        target: crate::gimli::UnitOffset,
    ) -> Self {
        Self {
            dwarf,
            unit,
            entry,
            name,
            target,
        }
    }

    /// The name of this reference type.
    pub fn name(&self) -> Option<&super::Name<R>> {
        self.name.as_ref()
    }

    /// The size of this type, in bytes.
    pub fn size(&self) -> u64 {
        core::mem::size_of::<usize>() as _
    }

    /// The type of the referent.
    pub fn r#type(&self) -> Result<super::Type<'dwarf, R>, crate::Error> {
        let entry = self.unit.entry(self.target)?;
        super::Type::from_die(self.dwarf, self.unit, entry)
    }
}

impl<'dwarf, R> fmt::Debug for Box<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entry = self.unit.entry(self.entry).map_err(crate::fmt_err)?;
        let mut debug_tuple = f.debug_tuple("deflect::schema::Shared");
        debug_tuple.field(&crate::debug::DebugEntry::new(
            self.dwarf, self.unit, &entry,
        ));
        debug_tuple.finish()
    }
}

impl<'dwarf, R> fmt::Display for Box<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = self.name() {
            name.fmt(f)
        } else {
            f.write_str("*? ")?;
            let target = self.r#type().map_err(crate::fmt_err)?;
            target.fmt(f)
        }
    }
}
