//! Reflections of Rust types.

mod array;
mod r#box;
mod boxed_dyn;
mod boxed_slice;
mod data;
mod r#enum;
mod field;
mod fields;
mod function;
mod name;
mod offset;
mod pointer;
mod slice;
mod str_impl;
mod r#struct;
mod variant;
mod variants;

pub use array::Array;
pub use boxed_dyn::BoxedDyn;
pub use boxed_slice::BoxedSlice;
pub use data::Data;
pub use fields::{Fields, FieldsIter};
pub use function::Function;
pub use name::Name;
pub use offset::Offset;
pub use pointer::{Const, Mut, Pointer, Reference, Shared, Unique};
pub use r#box::Box;
pub use r#enum::Enum;
pub use r#field::Field;
pub use r#struct::Struct;
pub use r#variant::Variant;
pub use slice::Slice;
pub use str_impl::str;
pub use variants::{Variants, VariantsIter};

/// A reflected shared reference type.
pub type SharedRef<'dwarf, R> = crate::schema::Pointer<'dwarf, crate::schema::Shared, R>;

/// A reflected unique reference type.
pub type UniqueRef<'dwarf, R> = crate::schema::Pointer<'dwarf, crate::schema::Unique, R>;

/// A reflected `const` pointer type.
pub type ConstPtr<'dwarf, R> = crate::schema::Pointer<'dwarf, crate::schema::Const, R>;

/// A reflected `mut` pointer type.
pub type MutPtr<'dwarf, R> = crate::schema::Pointer<'dwarf, crate::schema::Mut, R>;

impl<'dwarf, R> Type<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) fn from_die(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        Ok(match entry.tag() {
            crate::gimli::DW_TAG_base_type => {
                let name = Name::from_die(dwarf, unit, &entry)?;
                let name = name.to_slice()?;
                return match name.as_ref() {
                    b"bool" => bool::from_dw_tag_base_type(dwarf, unit, entry).map(Self::bool),
                    b"char" => char::from_dw_tag_base_type(dwarf, unit, entry).map(Self::char),
                    b"f32" => f32::from_dw_tag_base_type(dwarf, unit, entry).map(Self::f32),
                    b"f64" => f64::from_dw_tag_base_type(dwarf, unit, entry).map(Self::f64),
                    b"i8" => i8::from_dw_tag_base_type(dwarf, unit, entry).map(Self::i8),
                    b"i16" => i16::from_dw_tag_base_type(dwarf, unit, entry).map(Self::i16),
                    b"i32" => i32::from_dw_tag_base_type(dwarf, unit, entry).map(Self::i32),
                    b"i64" => i64::from_dw_tag_base_type(dwarf, unit, entry).map(Self::i64),
                    b"i128" => i128::from_dw_tag_base_type(dwarf, unit, entry).map(Self::i128),
                    b"isize" => isize::from_dw_tag_base_type(dwarf, unit, entry).map(Self::isize),
                    b"u8" => u8::from_dw_tag_base_type(dwarf, unit, entry).map(Self::u8),
                    b"u16" => u16::from_dw_tag_base_type(dwarf, unit, entry).map(Self::u16),
                    b"u32" => u32::from_dw_tag_base_type(dwarf, unit, entry).map(Self::u32),
                    b"u64" => u64::from_dw_tag_base_type(dwarf, unit, entry).map(Self::u64),
                    b"u128" => u128::from_dw_tag_base_type(dwarf, unit, entry).map(Self::u128),
                    b"usize" => usize::from_dw_tag_base_type(dwarf, unit, entry).map(Self::usize),
                    b"()" => unit::from_dw_tag_base_type(dwarf, unit, entry).map(Self::unit),
                    _ => unimplemented!(
                        "unhandled primitive: {:#?}",
                        crate::debug::DebugEntry::new(dwarf, unit, &entry)
                    ),
                };
            }
            crate::gimli::DW_TAG_structure_type => {
                let name = Name::from_die(dwarf, unit, &entry)?;
                let name_slice = name.to_slice()?;
                if name_slice.starts_with(b"&[") {
                    return Ok(Self::Slice(Slice::from_dw_tag_structure_type(
                        dwarf, unit, entry,
                    )?));
                } else if name_slice.starts_with(b"&[") {
                    return Ok(Self::Slice(Slice::from_dw_tag_structure_type(
                        dwarf, unit, entry,
                    )?));
                } else if &*name_slice == b"&str" {
                    return Ok(Self::str(str::from_dw_tag_structure_type(
                        dwarf, unit, entry,
                    )?));
                } else if name_slice.starts_with(b"alloc::boxed::Box<") {
                    // boxedslice: data_ptr + length
                    let schema = Struct::from_dw_tag_structure_type(dwarf, unit, entry)?;
                    let mut fields = schema.fields()?;
                    let mut fields = fields.iter()?;
                    let pointer = fields.try_next()?;
                    let pointer = pointer
                        .ok_or_else(|| crate::error::missing_child(crate::gimli::DW_TAG_member))?;
                    let metadata = fields.try_next()?;
                    let metadata = metadata
                        .ok_or_else(|| crate::error::missing_child(crate::gimli::DW_TAG_member))?;
                    let metadata_name = metadata.name()?;
                    let metadata_name_slice = metadata_name.to_slice()?;
                    return match metadata_name_slice.as_ref() {
                        b"length" => {
                            BoxedSlice::new(schema, pointer, metadata).map(Self::BoxedSlice)
                        }
                        b"vtable" => BoxedDyn::new(schema, pointer, metadata).map(Self::BoxedDyn),
                        _ => Err(crate::error::name_mismatch(
                            "`length` or `vtable`",
                            metadata_name.to_string_lossy()?.into_owned(),
                        ))?,
                    };
                } else {
                    let mut tree = unit.entries_tree(Some(entry.offset()))?;
                    let root = tree.root()?;
                    let mut children = root.children();
                    let mut variants = None;

                    while let Some(child) = children.next()? {
                        if child.entry().tag() == crate::gimli::DW_TAG_variant_part {
                            variants = Some(child.entry().clone());
                            break;
                        }
                    }

                    if let Some(_variants) = variants {
                        Self::Enum(Enum::from_dw_tag_structure_type(dwarf, unit, entry)?)
                    } else {
                        Self::Struct(Struct::from_dw_tag_structure_type(dwarf, unit, entry)?)
                    }
                }
            }
            crate::gimli::DW_TAG_enumeration_type => {
                Self::Enum(Enum::from_dw_tag_enumeration_type(dwarf, unit, entry)?)
            }
            crate::gimli::DW_TAG_pointer_type => {
                let name = Name::from_die_opt(dwarf, unit, &entry)?;
                let target = crate::get_type_ref(&entry)?;
                if let Some(name) = name {
                    let name_as_slice = name.to_slice()?;
                    if name_as_slice.starts_with(b"*mut ") {
                        Self::MutPtr(Pointer::new(
                            dwarf,
                            unit,
                            entry.offset(),
                            Some(name),
                            target,
                        ))
                    } else if name_as_slice.starts_with(b"*const ") {
                        Self::ConstPtr(Pointer::new(
                            dwarf,
                            unit,
                            entry.offset(),
                            Some(name),
                            target,
                        ))
                    } else if name_as_slice.starts_with(b"&mut ") {
                        Self::UniqueRef(Pointer::new(
                            dwarf,
                            unit,
                            entry.offset(),
                            Some(name),
                            target,
                        ))
                    } else if name_as_slice.starts_with(b"&") {
                        Self::SharedRef(Pointer::new(
                            dwarf,
                            unit,
                            entry.offset(),
                            Some(name),
                            target,
                        ))
                    } else if name_as_slice.starts_with(b"fn") {
                        // TODO: This should probably be its own type.
                        Self::SharedRef(Pointer::new(
                            dwarf,
                            unit,
                            entry.offset(),
                            Some(name),
                            target,
                        ))
                    } else if name_as_slice.starts_with(b"alloc::boxed::Box<") {
                        Self::Box(Box::new(dwarf, unit, entry.offset(), Some(name), target))
                    } else {
                        eprintln!(
                            "{:#?}",
                            &crate::debug::DebugEntry::new(dwarf, unit, &entry,)
                        );

                        return Err(crate::error::invalid_attr(crate::gimli::DW_AT_name));
                    }
                } else {
                    // the `data_ptr` field of slices points to a pointer type that doesn't have a name.
                    Self::MutPtr(Pointer::new(dwarf, unit, entry.offset(), None, target))
                }
            }
            crate::gimli::DW_TAG_subroutine_type => {
                Self::Function(Function::from_dw_tag_subroutine_type(dwarf, unit, entry)?)
            }
            crate::gimli::DW_TAG_array_type => {
                Self::Array(Array::from_dw_tag_array_type(dwarf, unit, entry)?)
            }
            _otherwise => {
                eprintln!(
                    "UNHANDLED DEBUG ENTRY:\n{:#?}",
                    &crate::debug::DebugEntry::new(dwarf, unit, &entry,)
                );
                anyhow::bail!(
                    "Unhandled debug info kind:\n{:#?}",
                    crate::debug::DebugEntry::new(dwarf, unit, &entry,)
                )
            }
        })
    }

    /// The size of the type.
    pub fn size(&self) -> Result<std::primitive::u64, crate::Error> {
        match self {
            Self::bool(v) => Ok(v.size()),
            Self::char(v) => Ok(v.size()),
            Self::f32(v) => Ok(v.size()),
            Self::f64(v) => Ok(v.size()),
            Self::i8(v) => Ok(v.size()),
            Self::i16(v) => Ok(v.size()),
            Self::i32(v) => Ok(v.size()),
            Self::i64(v) => Ok(v.size()),
            Self::i128(v) => Ok(v.size()),
            Self::isize(v) => Ok(v.size()),
            Self::u8(v) => Ok(v.size()),
            Self::u16(v) => Ok(v.size()),
            Self::u32(v) => Ok(v.size()),
            Self::u64(v) => Ok(v.size()),
            Self::u128(v) => Ok(v.size()),
            Self::usize(v) => Ok(v.size()),
            Self::unit(v) => Ok(v.size()),
            Self::Box(v) => Ok(v.size()),
            Self::BoxedSlice(v) => v.size(),
            Self::BoxedDyn(v) => v.size(),
            Self::Array(v) => v.bytes(),
            Self::Slice(v) => v.size(),
            Self::str(v) => v.size(),
            Self::Struct(v) => v.size(),
            Self::Enum(v) => v.size(),
            Self::Function(_) => Ok(0),
            Self::SharedRef(_) => Ok(std::mem::size_of::<std::primitive::usize>() as _),
            Self::UniqueRef(_) => Ok(std::mem::size_of::<std::primitive::usize>() as _),
            Self::ConstPtr(_) => Ok(std::mem::size_of::<std::primitive::usize>() as _),
            Self::MutPtr(_) => Ok(std::mem::size_of::<std::primitive::usize>() as _),
        }
    }
}

pub use super::Type;

macro_rules! generate_primitive {
    ($($t:ident,)*) => {
        $(
            generate_primitive!(@
                $t,
                concat!(
                    "A schema for [`",
                    stringify!($t),
                    "`][std::primitive::",
                    stringify!($t),
                    "]."
                )
            );
        )*
    };
    (@ $t:ident, $doc:expr) => {
        #[doc = $doc]
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct $t<'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            dwarf: &'dwarf crate::gimli::Dwarf<R>,
            unit: &'dwarf crate::gimli::Unit<R, std::primitive::usize>,
            entry: crate::gimli::UnitOffset,
        }

        impl<'dwarf, R> $t<'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            pub(crate) fn from_dw_tag_base_type(
                dwarf: &'dwarf crate::gimli::Dwarf<R>,
                unit: &'dwarf crate::gimli::Unit<R, std::primitive::usize>,
                entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
            ) -> Result<Self, crate::Error> {
                crate::check_tag(&entry, crate::gimli::DW_TAG_base_type)?;

                let name = Name::from_die(dwarf, unit, &entry)?;
                let expected = std::any::type_name::<std::primitive::$t>();
                if name.to_slice()? != expected.as_bytes() {
                    let actual = name.to_string_lossy()?.to_string();
                    Err(crate::error::name_mismatch(expected, actual))?;
                }

                let size: std::primitive::usize = crate::get_size(&entry)?
                    .try_into()?;
                let expected = core::mem::size_of::<std::primitive::$t>();
                if size != expected {
                    Err(crate::error::size_mismatch(expected, size))?;
                }

                Ok(Self {
                    dwarf,
                    unit,
                    entry: entry.offset()
                })
            }


            /// The size of this type.
            pub fn name(&self) -> &'static std::primitive::str {
                std::any::type_name::<std::primitive::$t>()
            }

            /// The size of this type.
            pub fn size(&self) -> std::primitive::u64 {
                std::mem::size_of::<std::primitive::$t>() as _
            }

            /// The minimum alignment of this type.
            pub fn align(&self) -> std::primitive::u64 {
                std::mem::align_of::<std::primitive::$t>() as _
            }
        }

        impl<'dwarf, R> std::fmt::Debug for $t<'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let entry = self.unit.entry(self.entry).map_err(crate::fmt_err)?;
                let mut debug_tuple = f.debug_tuple(stringify!($t));
                debug_tuple.field(&crate::debug::DebugEntry::new(
                    self.dwarf,
                    self.unit,
                    &entry,
                ));
                debug_tuple.finish()
            }
        }

        impl<'dwarf, R> std::fmt::Display for $t<'dwarf, R>
        where
            R: crate::gimli::Reader<Offset = std::primitive::usize>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.name().fmt(f)
            }
        }
    };
}

generate_primitive! {
    bool,
    char,
    f32,
    f64,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
}

/// A schema for [`()`][prim@unit].
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct unit<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R, std::primitive::usize>,
    entry: crate::gimli::UnitOffset,
}

impl<'dwarf, R> unit<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    pub(crate) fn from_dw_tag_base_type(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R, std::primitive::usize>,
        entry: crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Self, crate::Error> {
        crate::check_tag(&entry, crate::gimli::DW_TAG_base_type)?;

        let name = Name::from_die(dwarf, unit, &entry)?;
        let expected = std::any::type_name::<()>();
        if name.to_slice()? != expected.as_bytes() {
            let actual = name.to_string_lossy()?.to_string();
            Err(crate::error::name_mismatch(expected, actual))?;
        }

        let size: std::primitive::usize = crate::get_size(&entry)?.try_into()?;
        let expected = core::mem::size_of::<()>();
        if size != expected {
            Err(crate::error::size_mismatch(expected, size))?;
        }

        Ok(Self {
            dwarf,
            unit,
            entry: entry.offset(),
        })
    }

    /// The size of this type.
    pub fn name(&self) -> &'static std::primitive::str {
        std::any::type_name::<()>()
    }

    /// The size of this type.
    pub fn size(&self) -> std::primitive::u64 {
        std::mem::size_of::<()>() as _
    }

    /// The minimum alignment of this type.
    pub fn align(&self) -> std::primitive::u64 {
        std::mem::align_of::<()>() as _
    }
}

impl<'dwarf, R> std::fmt::Debug for unit<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entry = self.unit.entry(self.entry).map_err(crate::fmt_err)?;
        let mut debug_tuple = f.debug_tuple(stringify!($t));
        debug_tuple.field(&crate::debug::DebugEntry::new(
            self.dwarf, self.unit, &entry,
        ));
        debug_tuple.finish()
    }
}

impl<'dwarf, R> std::fmt::Display for unit<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = std::primitive::usize>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
    }
}
