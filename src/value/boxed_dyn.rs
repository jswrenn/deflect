use std::fmt;

/// A reflected [`Box`]'d `dyn Trait` object value.
pub struct BoxedDyn<'value, 'dwarf, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    value: crate::Bytes<'value>,
    schema: crate::schema::BoxedDyn<'dwarf, P::Reader>,
    provider: &'dwarf P,
}

impl<'dwarf, R> crate::schema::BoxedDyn<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) unsafe fn with_bytes<'value, P>(
        self,
        provider: &'dwarf P,
        value: crate::Bytes<'value>,
    ) -> Result<BoxedDyn<'value, 'dwarf, P>, crate::Error>
    where
        P: crate::DebugInfoProvider<Reader = R>,
    {
        Ok(BoxedDyn {
            schema: self,
            value,
            provider,
        })
    }
}

impl<'value, 'dwarf, P> BoxedDyn<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    /// The schema of this value.
    pub fn schema(&self) -> &crate::schema::BoxedDyn<'dwarf, P::Reader> {
        &self.schema
    }

    fn data(&self, size: usize) -> Result<crate::Bytes<'value>, crate::Error> {
        let field =
            unsafe { super::Field::new(self.schema.pointer().clone(), self.value, self.provider) };
        let value = field.value()?;
        let value: super::Pointer<crate::schema::Mut, _> = value.try_into()?;
        let ptr = value.deref_raw_dyn(size)?;
        Ok(ptr)
    }

    fn vtable(&self) -> Result<super::Value<'value, 'dwarf, P>, crate::Error> {
        let vtable = self.schema.vtable().clone();
        let field = unsafe { super::Field::new(vtable, self.value, self.provider) };
        let value = field.value()?;
        Ok(value)
    }

    /// [`Box`]'d `dyn Trait` object value.
    pub fn deref(&self) -> Result<super::Value<'value, 'dwarf, P>, crate::Error> {
        let vtable = self.vtable()?;
        let vtable: super::Pointer<crate::schema::Shared, _> = vtable.try_into()?;
        let vtable = vtable.deref()?;
        let vtable: super::Array<_> = vtable.try_into()?;
        let mut vtable = vtable.iter()?;
        let drop_glue = vtable.next();
        let drop_glue = drop_glue.unwrap();
        let drop_glue = drop_glue?;
        let drop_glue: usize = drop_glue.try_into()?;

        let size = vtable.next();
        let size = size.unwrap();
        let size = size?;
        let size: usize = size.try_into()?;

        let data = self.data(size)?;

        let crate::DebugInfo {
            context,
            unit,
            entry,
        } = self.provider.info_for(drop_glue as _)?;
        let entry = unit.entry(entry)?;
        let schema = crate::schema::Type::from_die(context.dwarf(), unit, entry)?;

        unsafe { crate::Value::with_type(schema, data, self.provider) }
    }
}

impl<'value, 'dwarf, P> fmt::Debug for BoxedDyn<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::BoxedDyn");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, P> fmt::Display for BoxedDyn<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.deref().map_err(crate::fmt_err)?;
        f.write_str("box ")?;
        value.fmt(f)
    }
}

impl<'value, 'dwarf, P> serde::Serialize for BoxedDyn<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = self.deref().map_err(crate::ser_err)?;
        value.serialize(serializer)
    }
}
