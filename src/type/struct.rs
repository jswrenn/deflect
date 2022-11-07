use std::fmt;

pub struct Struct<'dwarf, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf gimli::Dwarf<R>,
    unit: &'dwarf gimli::Unit<R, usize>,
    entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
}

impl<'dwarf, 'value, R> fmt::Debug for Struct<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct(&self.name());
        self.fields(|field| {
            ds.field(field.name().as_str(), &field.r#type());
        });
        ds.finish()
    }
}

impl<'dwarf, 'value, R> Struct<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    pub(crate) fn from_dw_tag_structure_type(
        dwarf: &'dwarf gimli::Dwarf<R>,
        unit: &'dwarf gimli::Unit<R, usize>,
        entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Self {
        Self { dwarf, unit, entry }
    }

    pub fn name(&self) -> String {
        crate::get_name(&self.entry, self.dwarf, self.unit)
            .unwrap()
            .unwrap()
            .to_string_lossy()
            .unwrap()
            .to_owned()
            .to_string()
    }

    /*pub fn fields_2(&self) -> super::Fields<'dwarf, R> {
        super::Fields::new
    }*/

    pub fn fields<F>(&self, mut f: F)
    where
        F: FnMut(super::field::Field<'dwarf, R>),
    {
        let mut tree = self.unit.entries_tree(Some(self.entry.offset())).unwrap();
        let root = tree.root().unwrap();
        let mut children = root.children();
        while let Some(child) = children.next().unwrap() {
            match child.entry().tag() {
                gimli::DW_TAG_member => f(super::field::Field::from_dw_tag_member(
                    self.dwarf,
                    self.unit,
                    child.entry().clone(),
                )),
                _ => continue,
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
