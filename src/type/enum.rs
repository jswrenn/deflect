use std::fmt;

pub struct Enum<'dwarf, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf gimli::Dwarf<R>,
    unit: &'dwarf gimli::Unit<R, usize>,
    entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
}

impl<'dwarf, R> fmt::Debug for Enum<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_tuple(&self.name());
        self.variants(|variant| {
            ds.field(&variant);
        });
        ds.finish()
    }
}

impl<'dwarf, R> Enum<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    pub(crate) fn new(
        dwarf: &'dwarf gimli::Dwarf<R>,
        unit: &'dwarf gimli::Unit<R, usize>,
        entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Self {
        let _tree = unit.entries_tree(Some(entry.offset())).unwrap();
        //crate::inspect_tree(&mut tree, dwarf, unit);
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

    pub fn variants_2(&self) {
        let mut tree = self.unit.entries_tree(Some(self.entry.offset())).unwrap();
        let root = tree.root().unwrap();
        match self.entry.tag() {
            gimli::DW_TAG_enumeration_type => {
                let mut children = root.children();
                while let Some(child) = children.next().unwrap() {
                    let child = child.entry();
                    assert_eq!(child.tag(), gimli::DW_TAG_enumerator);
                    let discriminant = match child.attr_value(gimli::DW_AT_const_value).unwrap() {
                        Some(gimli::AttributeValue::Data1(value)) => {
                            Some(super::discriminant::Discriminant::U8(value))
                        }
                        Some(gimli::AttributeValue::Data2(value)) => {
                            Some(super::discriminant::Discriminant::U16(value))
                        }
                        Some(gimli::AttributeValue::Data4(value)) => {
                            Some(super::discriminant::Discriminant::U32(value))
                        }
                        Some(gimli::AttributeValue::Data8(value)) => {
                            Some(super::discriminant::Discriminant::U64(value))
                        }
                        None => None,
                        _ => panic!(),
                    };
                    let _variant = super::variant::Variant::new(
                        self.dwarf,
                        self.unit,
                        child.clone(),
                        discriminant,
                    );
                }
            }
            gimli::DW_TAG_structure_type => {
                todo!();
            }
            _ => panic!(),
        }
    }

    pub fn variants<F>(&self, mut f: F)
    where
        F: FnMut(super::variant::Variant<'dwarf, R>),
    {
        let mut tree = self.unit.entries_tree(Some(self.entry.offset())).unwrap();
        let root = tree.root().unwrap();
        match self.entry.tag() {
            gimli::DW_TAG_enumeration_type => {
                let discriminant_type = crate::get_type(&self.entry).unwrap().unwrap();
                let discriminant_type = self.unit.entry(discriminant_type).unwrap();
                let discriminant_type =
                    super::Type::from_die(self.dwarf, self.unit, discriminant_type);

                let mut children = root.children();
                while let Some(child) = children.next().unwrap() {
                    let child = child.entry();
                    assert_eq!(child.tag(), gimli::DW_TAG_enumerator);

                    let gimli::AttributeValue::Udata(value) = child.attr_value(gimli::DW_AT_const_value).unwrap().unwrap() else {
                        unimplemented!()
                    };

                    let _discriminant = Some(match discriminant_type {
                        crate::r#type::Type::U8 => super::Discriminant::U8(value as _),
                        crate::r#type::Type::U16 => super::Discriminant::U16(value as _),
                        crate::r#type::Type::U32 => super::Discriminant::U32(value as _),
                        crate::r#type::Type::U64 => super::Discriminant::U64(value as _),
                        _ => panic!(),
                    });

                    let discriminant: Option<super::Discriminant> = child
                        .attr_value(gimli::DW_AT_const_value)
                        .unwrap()
                        .map(|value| value.into());
                    f(super::variant::Variant::new(
                        self.dwarf,
                        self.unit,
                        child.clone(),
                        discriminant,
                    ));
                }
            }
            gimli::DW_TAG_structure_type => {
                let mut variant_part = None;
                {
                    let mut children = root.children();
                    while let Some(child) = children.next().unwrap() {
                        if child.entry().tag() == gimli::DW_TAG_variant_part {
                            variant_part = Some(child.entry().offset());
                            break;
                        }
                    }
                }

                let mut tree = self.unit.entries_tree(variant_part).unwrap();
                let root = tree.root().unwrap();
                let mut variants = root.children();

                while let Some(child) = variants.next().unwrap() {
                    let entry = child.entry();
                    if child.entry().tag() == gimli::DW_TAG_variant {
                        let discriminant: Option<super::Discriminant> = entry
                            .attr_value(gimli::DW_AT_discr_value)
                            .unwrap()
                            .map(|value| value.into());

                        let mut variant_children = child.children();
                        let variant_entry = variant_children.next().unwrap().unwrap();
                        let variant_entry = variant_entry.entry();
                        let variant_ty = crate::get_type(variant_entry).unwrap().unwrap();
                        let entry = self.unit.entry(variant_ty).unwrap();

                        f(super::variant::Variant::new(
                            self.dwarf,
                            self.unit,
                            entry,
                            discriminant,
                        ));
                    }
                }
            }
            _ => panic!(),
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
