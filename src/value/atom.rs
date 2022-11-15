use std::{fmt, ops};

pub struct Atom<'dwarf, 'value, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Atom<'dwarf, R>,
    value: crate::Bytes<'value>,
}

pub enum RustAtom<'value> {
    Bool(&'value bool),
    Char(&'value char),
    F32(&'value f32),
    F64(&'value f64),
    I8(&'value i8),
    I16(&'value i16),
    I32(&'value i32),
    I64(&'value i64),
    I128(&'value i128),
    ISize(&'value isize),
    U8(&'value u8),
    U16(&'value u16),
    U32(&'value u32),
    U64(&'value u64),
    U128(&'value u128),
    USize(&'value usize),
    Unit(&'value ()),
}

impl<'dwarf, 'value, R> Atom<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Atom<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        let size = schema.size().unwrap().unwrap() as usize;
        let value = &value[..size];
        Self { schema, value }
    }

    pub fn to_rust(&self) -> Option<RustAtom> {
        let Some(rust_atom) = self.schema.to_rust() else { return None };
        Some(match rust_atom {
            crate::schema::RustAtom::Bool => RustAtom::Bool(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::Char => RustAtom::Char(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::F32 => RustAtom::F32(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::F64 => RustAtom::F64(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::I8 => RustAtom::I8(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::I16 => RustAtom::I16(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::I32 => RustAtom::I32(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::I64 => RustAtom::I64(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::I128 => RustAtom::I128(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::ISize => RustAtom::ISize(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::U8 => RustAtom::U8(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::U16 => RustAtom::U16(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::U32 => RustAtom::U32(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::U64 => RustAtom::U64(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::U128 => RustAtom::U128(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::USize => RustAtom::USize(unsafe {
                let (&[], &[ref value], &[]) = self.value.align_to() else { panic!() };
                value
            }),
            crate::schema::RustAtom::Unit => RustAtom::Unit(&{
                let []: [crate::Byte; 0] = self.value.try_into().unwrap();
            }),
            crate::schema::RustAtom::Never => unreachable!(),
        })
    }
}

impl<'dwarf, 'value, R> fmt::Debug for Atom<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_rust().fmt(f)
    }
}

impl<'dwarf, 'value, R> ops::Deref for Atom<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Atom<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}

impl<'value> fmt::Debug for RustAtom<'value> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustAtom::Bool(value) => value.fmt(f),
            RustAtom::Char(value) => value.fmt(f),
            RustAtom::F32(value) => value.fmt(f),
            RustAtom::F64(value) => value.fmt(f),
            RustAtom::I8(value) => value.fmt(f),
            RustAtom::I16(value) => value.fmt(f),
            RustAtom::I32(value) => value.fmt(f),
            RustAtom::I64(value) => value.fmt(f),
            RustAtom::I128(value) => value.fmt(f),
            RustAtom::ISize(value) => value.fmt(f),
            RustAtom::U8(value) => value.fmt(f),
            RustAtom::U16(value) => value.fmt(f),
            RustAtom::U32(value) => value.fmt(f),
            RustAtom::U64(value) => value.fmt(f),
            RustAtom::U128(value) => value.fmt(f),
            RustAtom::USize(value) => value.fmt(f),
            RustAtom::Unit(value) => value.fmt(f),
        }
    }
}
