use std::{fmt, ops};

/// A field of a [struct][super::Struct] or [variant][super::Variant].
pub struct Field<'value, 'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Field<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'value, 'dwarf, R> Field<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Field<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { schema, value }
    }

    /// The value of this field.
    pub fn value(&'dwarf self) -> Result<super::Value<'value, 'dwarf, R>, crate::err::Error> {
        let r#type = self.r#type()?;
        let offset = self.offset()?.address(0)? as usize;
        let value = &self.value[offset..];
        unsafe { super::Value::with_type(r#type, value) }
    }
}

impl<'value, 'dwarf, R> fmt::Display for Field<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name().map_err(crate::fmt_err)?.fmt(f)?;
        f.write_str(" : ")?;
        self.value().map_err(crate::fmt_err)?.fmt(f)
    }
}

impl<'value, 'dwarf, R> ops::Deref for Field<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Field<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
