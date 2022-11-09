use std::fmt;

#[derive(Clone)]
pub struct Variant<'dwarf, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf gimli::Dwarf<R>,
    unit: &'dwarf gimli::Unit<R, usize>,
    entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    discriminant: super::Discriminant<R>,
    discriminant_value: Option<super::discriminant::DiscriminantValue>,
}

impl<'dwarf, 'value, R> fmt::Debug for Variant<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = format!("{} = {:?}", self.name(), &self.discriminant_value);
        let mut ds = f.debug_struct(name.as_str());
        self.fields(|field| {
            ds.field(&field.name().unwrap().to_string(), &field.r#type());
        });
        ds.finish()
    }
}

impl<'dwarf, 'value, R> Variant<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    pub(crate) fn new(
        dwarf: &'dwarf gimli::Dwarf<R>,
        unit: &'dwarf gimli::Unit<R, usize>,
        entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
        discriminant: super::Discriminant<R>,
        discriminant_value: Option<super::discriminant::DiscriminantValue>,
    ) -> Self {
        Self {
            dwarf,
            unit,
            entry,
            discriminant,
            discriminant_value,
        }
    }

    pub fn name(&self) -> String {
        crate::get_name(&self.entry, self.dwarf, self.unit)
            .unwrap()
            .to_string_lossy()
            .unwrap()
            .to_string()
    }

    pub fn discriminant(&self) -> &super::discriminant::Discriminant<R> {
        &self.discriminant
    }

    pub fn discriminant_value(&self) -> Option<super::discriminant::DiscriminantValue> {
        self.discriminant_value
    }

    pub fn fields<F>(&self, mut f: F)
    where
        F: FnMut(super::field::Field<'dwarf, R>),
    {
        if self.entry.has_children() {
            let mut tree = self.unit.entries_tree(Some(self.entry.offset())).unwrap();
            let root = tree.root().unwrap();
            let mut children = root.children();
            while let Some(child) = children.next().unwrap() {
                f(super::field::Field::from_dw_tag_member(
                    self.dwarf,
                    self.unit,
                    child.entry().clone(),
                )
                .unwrap());
            }
        }
    }

    pub fn size(&self) -> usize {
        self.entry
            .attr_value(gimli::DW_AT_byte_size)
            .unwrap()
            .and_then(|r| r.udata_value())
            .unwrap()
            .try_into()
            .unwrap()
    }

    pub fn align(&self) -> usize {
        self.entry
            .attr_value(gimli::DW_AT_alignment)
            .unwrap()
            .and_then(|r| r.udata_value())
            .unwrap()
            .try_into()
            .unwrap()
    }
}
