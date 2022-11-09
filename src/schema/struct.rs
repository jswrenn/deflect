use std::{borrow::Cow, fmt};

pub struct Struct<'dwarf, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf gimli::Dwarf<R>,
    unit: &'dwarf gimli::Unit<R, usize>,
    entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,

    name: R,
    size: u64,
    align: u64,
}

impl<'dwarf, 'value, R> fmt::Debug for Struct<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct(&self.name().unwrap());
        self.fields(|field| {
            ds.field(field.name().unwrap().as_ref(), &field.r#type());
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
    ) -> Result<Self, crate::Error> {
        println!("{:?}", entry.tag().static_string());
        crate::check_tag(&entry, gimli::DW_TAG_structure_type)?;
        let name = crate::get_name(&entry, dwarf, unit)?;
        let size = crate::get_size(&entry)?;
        let align = crate::get_align(&entry)?;
        Ok(Self { dwarf, unit, name, size, align, entry })
    }

    pub fn name(&self) -> Result<Cow<str>, gimli::Error> {
        self.name.to_string_lossy()
    }

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
                )
                .unwrap()),
                _ => continue,
            }
        }
    }

    pub fn size(&self) -> usize {
        self.size as _
    }

    pub fn align(&self) -> usize {
        self.align as _
    }
}
