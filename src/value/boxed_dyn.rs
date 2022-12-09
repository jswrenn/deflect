use std::fmt;

/// A reflected [`Box`]'d `dyn Trait` object value.
pub struct BoxedDyn<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    value: crate::Bytes<'value>,
    schema: crate::schema::BoxedDyn<'dwarf, P::Reader>,
    provider: &'dwarf P,
}

impl<'value, 'dwarf, P> BoxedDyn<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::BoxedDyn<'dwarf, P::Reader>,
        provider: &'dwarf P,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            value,
            schema,
            provider,
        })
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

impl<'value, 'dwarf, P> From<BoxedDyn<'value, 'dwarf, P>> for super::Value<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    fn from(value: BoxedDyn<'value, 'dwarf, P>) -> Self {
        super::Value::BoxedDyn(value)
    }
}

impl<'a, 'value, 'dwarf, P> TryFrom<&'a super::Value<'value, 'dwarf, P>>
    for &'a BoxedDyn<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: &'a super::Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let super::Value::BoxedDyn(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                &'a super::Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}

impl<'value, 'dwarf, P> TryFrom<super::Value<'value, 'dwarf, P>> for BoxedDyn<'value, 'dwarf, P>
where
    P: crate::DebugInfoProvider,
{
    type Error = crate::error::Downcast;

    fn try_from(value: super::Value<'value, 'dwarf, P>) -> Result<Self, Self::Error> {
        if let super::Value::BoxedDyn(value) = value {
            Ok(value)
        } else {
            Err(crate::error::Downcast::new::<
                super::Value<'value, 'dwarf, P>,
                Self,
            >())
        }
    }
}
