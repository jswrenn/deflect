use super::Name;
use std::fmt;

pub struct Atom<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
}

/// A primitive, non-compound (i.e., "atomic") type, like `u8` or `bool`.
impl<'dwarf, R> Atom<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) fn from_dw_tag_base_type(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        crate::check_tag(&entry, crate::gimli::DW_TAG_base_type)?;
        Ok(Self { dwarf, unit, entry })
    }

    /// The [DWARF](crate::gimli::Dwarf) sections that this debuginfo entry belongs to.
    pub fn dwarf(&self) -> &'dwarf crate::gimli::Dwarf<R> {
        self.dwarf
    }

    /// The DWARF [unit][gimli::Unit] that this debuginfo entry belongs to.
    pub fn unit(&self) -> &crate::gimli::Unit<R, usize> {
        self.unit
    }

    /// The [debugging information entry][crate::gimli::DebuggingInformationEntry] this type abstracts over.
    pub fn entry(&self) -> &crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R> {
        &self.entry
    }

    /// The name of this primitive type.
    pub fn name(&self) -> Result<Option<Name<R>>, crate::Error> {
        Ok(Name::from_die(self.dwarf(), self.unit(), self.entry())?)
    }

    /// The size of this type, in bytes.
    pub fn size(&self) -> Result<Option<u64>, crate::Error> {
        Ok(crate::get_size(self.entry())?)
    }

    /// The alignment of this type, in bytes.
    pub fn align(&self) -> Result<Option<u64>, crate::Error> {
        Ok(crate::get_align(self.entry())?)
    }

    pub fn to_rust(&self) -> Option<RustAtom> {
        let Some(name) = (match self.name() {
            Ok(name) => name,
            Err(err) => panic!("{:?}", err),
        }) else {
            panic!("type does not have a name");
        };
        let name = match name.to_slice() {
            Ok(name) => name,
            Err(err) => panic!("{:?}", err),
        };
        Some(match name.as_ref() {
            b"bool" => RustAtom::Bool,
            b"char" => RustAtom::Char,

            b"i8" => RustAtom::I8,
            b"i16" => RustAtom::I16,
            b"i32" => RustAtom::I32,
            b"i64" => RustAtom::I64,
            b"i128" => RustAtom::I128,
            b"isize" => RustAtom::ISize,

            b"u8" => RustAtom::U8,
            b"u16" => RustAtom::U16,
            b"u32" => RustAtom::U32,
            b"u128" => RustAtom::U128,
            b"usize" => RustAtom::USize,

            b"()" => RustAtom::Unit,
            b"!" => RustAtom::Never,

            _ => return None,
        })
    }
}

impl<'dwarf, R> fmt::Debug for Atom<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.name().unwrap().as_ref(), f)
    }
}

pub enum RustAtom {
    Bool = 1,
    Char,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    Unit,
    Never,
}
