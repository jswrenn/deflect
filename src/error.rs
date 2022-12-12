//! Error kinds.

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Kind {
    #[error(transparent)]
    TagMismatch(#[from] TagMismatch),
    #[error(transparent)]
    MissingAttr(#[from] MissingAttr),
    #[error(transparent)]
    InvalidAttr(#[from] InvalidAttr),
    #[error(transparent)]
    MissingChild(#[from] MissingChild),
    #[error(transparent)]
    MissingEntry(#[from] MissingEntry),
    #[error(transparent)]
    SizeMismatch(#[from] SizeMismatch),
    #[error(transparent)]
    NameMismatch(#[from] NameMismatch),
    #[error(transparent)]
    FileIndexing(#[from] FileIndexing),
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    Downcast(#[from] Downcast),
    #[error(transparent)]
    MissingSymbolAddress(MissingSymbolAddress),
    #[error(transparent)]
    MissingDebugInfo(MissingDebugInfo),
    #[error(transparent)]
    Gimli(#[from] crate::gimli::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Object(#[from] crate::object::read::Error),
    #[error(transparent)]
    Ref(#[from] &'static super::Error),
    #[error(transparent)]
    ArithmeticOverflow(ArithmeticOverflow),
    #[error(transparent)]
    EnumDestructure(EnumDestructure),
    #[error("other")]
    Other,
}

impl Kind {
    pub(crate) fn tag_mismatch(expected: crate::gimli::DwTag, actual: crate::gimli::DwTag) -> Self {
        Self::TagMismatch(TagMismatch { expected, actual })
    }

    pub(crate) fn missing_attr(attr: crate::gimli::DwAt) -> Self {
        Self::MissingAttr(MissingAttr { attr })
    }

    pub(crate) fn invalid_attr(attr: crate::gimli::DwAt) -> Self {
        Self::InvalidAttr(InvalidAttr { attr })
    }

    pub(crate) fn missing_child(tag: crate::gimli::DwTag) -> Self {
        Self::MissingChild(MissingChild { tag })
    }

    pub(crate) fn missing_entry(offset: crate::gimli::UnitOffset) -> Self {
        Self::MissingEntry(MissingEntry { offset })
    }

    pub(crate) fn size_mismatch(expected: usize, actual: usize) -> Self {
        Self::SizeMismatch(SizeMismatch { expected, actual })
    }

    pub(crate) fn name_mismatch(expected: &'static str, actual: String) -> Self {
        Self::NameMismatch(NameMismatch { expected, actual })
    }

    pub(crate) fn file_indexing() -> Self {
        Self::FileIndexing(FileIndexing { _field: () })
    }

    pub(crate) fn missing_symbol_address() -> Self {
        Self::MissingSymbolAddress(MissingSymbolAddress { _field: () })
    }

    pub(crate) fn missing_debug_info() -> Self {
        Self::MissingDebugInfo(MissingDebugInfo { _field: () })
    }

    pub(crate) fn arithmetic_overflow() -> Self {
        Self::ArithmeticOverflow(ArithmeticOverflow { _field: () })
    }

    pub(crate) fn enum_destructure() -> Self {
        Self::EnumDestructure(EnumDestructure { _field: () })
    }
}

/// Expected a DWARF tag of one kind, but received another.
#[derive(thiserror::Error, Debug)]
#[error("tag mismatch; expected {:?}, received {:?}", .expected.static_string(), .actual.static_string())]
pub struct TagMismatch {
    expected: crate::gimli::DwTag,
    actual: crate::gimli::DwTag,
}

/// Expected the DWARF DIE to have an attribute of one kind; it did not.
#[derive(thiserror::Error, Debug)]
#[error("DIE did not have the attribute {:?}", .attr.static_string())]
pub struct MissingAttr {
    attr: crate::gimli::DwAt,
}

/// Expected the DWARF attribute to have a value of some form; it did not.
#[derive(thiserror::Error, Debug)]
#[error("The attribute {:?} had an invalid value.", .attr.static_string())]
pub struct InvalidAttr {
    attr: crate::gimli::DwAt,
}

/// Expected the DWARF DIE to have a child of a given tag; it did not.
#[derive(thiserror::Error, Debug)]
#[error("die did not have the child {tag}")]
pub struct MissingChild {
    tag: crate::gimli::DwTag,
}

/// Expected the DWARF unit to have an entry at a given offset; it did not.
#[derive(thiserror::Error, Debug)]
#[error("unit did not have entry at offset=0x{offset:x}", offset = .offset.0)]
pub struct MissingEntry {
    offset: crate::gimli::UnitOffset,
}

/// Expected the value to have a particular size; it did not.
#[derive(thiserror::Error, Debug)]
#[error("size mismatch; expected {:?}, received {:?}", .expected, .actual)]
pub struct SizeMismatch {
    expected: usize,
    actual: usize,
}

/// Expected the entry to have a given name; it did not.
#[derive(thiserror::Error, Debug)]
#[error("name mismatch; expected {:?}, received {:?}", .expected, .actual)]
pub struct NameMismatch {
    expected: &'static str,
    actual: String,
}

/// Could not downcast the value into the given type.
#[derive(thiserror::Error, Debug)]
#[error("Could not downcast into {src}, received {dst}")]
pub struct Downcast {
    src: &'static str,
    dst: &'static str,
}

impl Downcast {
    pub(crate) fn new<Src, Dst>() -> Self {
        let src = std::any::type_name::<Src>();
        let dst = std::any::type_name::<Dst>();
        Self { src, dst }
    }
}

/// Could not resolve a `FileIndex` into a file name.
#[derive(thiserror::Error, Debug)]
#[error("could not resolve FileIndex to filename")]
pub struct FileIndexing {
    _field: (),
}

/// Could not find the symbol address of a given memory address.
#[derive(thiserror::Error, Debug)]
#[error("`Reflect::symbol_address` failed; could not find symbol address for this type.")]
pub struct MissingSymbolAddress {
    _field: (),
}

/// Could not find debug info for the given function.
#[derive(thiserror::Error, Debug)]
#[error("Could not find debug info for symbol address.")]
pub struct MissingDebugInfo {
    _field: (),
}

/// Arithmetic operation overflowed.
#[derive(thiserror::Error, Debug)]
#[error("An arithmetic operation unexpectedly overflowed.")]
pub struct ArithmeticOverflow {
    _field: (),
}

/// A reflected `enum` could not be destructured into its variant.
#[derive(thiserror::Error, Debug)]
#[error("A reflected `enum` could not be destructured into its variant. This is a bug.")]
pub struct EnumDestructure {
    _field: (),
}
