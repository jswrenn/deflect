use std::{borrow::Cow, fmt};

pub struct Enum<'dwarf, R: gimli::Reader<Offset = usize>>
where
    R: gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf gimli::Dwarf<R>,
    pub(crate) unit: &'dwarf gimli::Unit<R, usize>,
    entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    name: R,
    discriminant: super::Discriminant<R>,
}

impl<'dwarf, R> fmt::Debug for Enum<'dwarf, R>
where
    R: gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_tuple(&self.name().unwrap());
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
    pub(crate) fn from_dw_tag_enumeration_type(
        dwarf: &'dwarf gimli::Dwarf<R>,
        unit: &'dwarf gimli::Unit<R, usize>,
        entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        crate::check_tag(&entry, gimli::DW_TAG_enumeration_type)?;
        let name = crate::get_name(&entry, dwarf, unit)?;
        let discriminant = super::Discriminant::from_dw_tag_enumeration_type(dwarf, unit, &entry)?;
        Ok(Self {
            dwarf,
            unit,
            entry,
            name,
            discriminant,
        })
    }

    pub(crate) fn from_dw_tag_structure_type(
        dwarf: &'dwarf gimli::Dwarf<R>,
        unit: &'dwarf gimli::Unit<R, usize>,
        entry: gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        crate::check_tag(&entry, gimli::DW_TAG_structure_type)?;
        let name = crate::get_name(&entry, dwarf, unit)?;
        let discriminant = 'variant: {
            let mut tree = unit.entries_tree(Some(entry.offset()))?;
            let root = tree.root()?;
            let mut children = root.children();
            while let Some(child) = children.next()? {
                let entry = child.entry();
                if child.entry().tag() == gimli::DW_TAG_variant_part {
                    break 'variant super::Discriminant::from_dw_tag_variant_part(dwarf, unit, entry)?;
                }
            }
            return Err(crate::ErrorKind::MissingChild { tag: gimli::DW_TAG_variant_part })?;
        };

        Ok(Self {
            dwarf,
            unit,
            entry,
            name,
            discriminant,
        })
    }

    pub fn name(&self) -> Result<Cow<str>, gimli::Error> {
        self.name.to_string_lossy()
    }

    pub fn variants<F>(&self, mut f: F)
    where
        F: FnMut(super::variant::Variant<'dwarf, R>),
    {
        let mut tree = self.unit.entries_tree(Some(self.entry.offset())).unwrap();
        let root = tree.root().unwrap();
        match self.entry.tag() {
            gimli::DW_TAG_enumeration_type => {
                let discriminant_type = crate::get_type(&self.entry).unwrap();
                let discriminant_type = self.unit.entry(discriminant_type).unwrap();
                let discriminant_type =
                    super::Type::from_die(self.dwarf, self.unit, discriminant_type).unwrap();

                let mut children = root.children();
                while let Some(child) = children.next().unwrap() {
                    let child = child.entry();
                    assert_eq!(child.tag(), gimli::DW_TAG_enumerator);

                    let gimli::AttributeValue::Udata(value) = child.attr_value(gimli::DW_AT_const_value).unwrap().unwrap() else {
                        unimplemented!()
                    };

                    let _discriminant = Some(match discriminant_type {
                        super::Type::U8 => super::DiscriminantValue::U8(value as _),
                        super::Type::U16 => super::DiscriminantValue::U16(value as _),
                        super::Type::U32 => super::DiscriminantValue::U32(value as _),
                        super::Type::U64 => super::DiscriminantValue::U64(value as _),
                        _ => panic!(),
                    });

                    let discriminant_value: Option<super::DiscriminantValue> = child
                        .attr_value(gimli::DW_AT_const_value)
                        .unwrap()
                        .map(|value| value.into());
                    f(super::variant::Variant::new(
                        self.dwarf,
                        self.unit,
                        child.clone(),
                        self.discriminant.clone(),
                        discriminant_value,
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
                        let discriminant_value: Option<super::DiscriminantValue> = entry
                            .attr_value(gimli::DW_AT_discr_value)
                            .unwrap()
                            .map(|value| value.into());

                        let mut variant_children = child.children();
                        let variant_entry = variant_children.next().unwrap().unwrap();
                        let variant_entry = variant_entry.entry();
                        let variant_ty = crate::get_type(variant_entry).unwrap();
                        let entry = self.unit.entry(variant_ty).unwrap();

                        f(super::variant::Variant::new(
                            self.dwarf,
                            self.unit,
                            entry,
                            self.discriminant.clone(),
                            discriminant_value,
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

    pub fn discriminant(&self) -> &super::Discriminant<R> {
        &self.discriminant
    }
}
