use std::{
    fmt::{self, Write},
    ops,
};

pub struct Atom<'dwarf, 'value, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Field<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'dwarf, 'value, R> Atom<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Field<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { schema, value }
    }
}

impl<'dwarf, 'value, R> fmt::Debug for Atom<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some(name) = (match self.name() {
            Ok(name) => name,
            Err(err) => panic!("{:?}", err),
        }) else {
            panic!("field does not have a name");
        };
        let name = match name.to_string_lossy() {
            Ok(name) => name,
            Err(err) => panic!("{:?}", err),
        };
        f.write_str(&name)
    }
}

impl<'dwarf, 'value, R> ops::Deref for Atom<'dwarf, 'value, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Field<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
