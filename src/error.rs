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
}

#[derive(thiserror::Error, Debug)]
#[error("tag mismatch; expected {:?}, received {:?}", .expected.static_string(), .actual.static_string())]
pub struct TagMismatch {
    expected: crate::gimli::DwTag,
    actual: crate::gimli::DwTag,
}

#[derive(thiserror::Error, Debug)]
#[error("DIE did not have the attribute {:?}", .attr.static_string())]
pub struct MissingAttr {
    attr: crate::gimli::DwAt,
}

#[derive(thiserror::Error, Debug)]
#[error("The attribute {:?} had an invalid value.", .attr.static_string())]
pub struct InvalidAttr {
    attr: crate::gimli::DwAt,
}

#[derive(thiserror::Error, Debug)]
#[error("die did not have the child {tag}")]
pub struct MissingChild {
    tag: crate::gimli::DwTag,
}

#[derive(thiserror::Error, Debug)]
#[error("unit did not have entry at offset=0x{offset:x}", offset = .offset.0)]
pub struct MissingEntry {
    offset: crate::gimli::UnitOffset,
}

#[derive(thiserror::Error, Debug)]
#[error("size mismatch; expected {:?}, received {:?}", .expected, .actual)]
pub struct SizeMismatch {
    expected: usize,
    actual: usize,
}

#[derive(thiserror::Error, Debug)]
#[error("name mismatch; expected {:?}, received {:?}", .expected, .actual)]
pub struct NameMismatch {
    expected: &'static str,
    actual: String,
}

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

#[derive(thiserror::Error, Debug)]
#[error("could not resolve FileIndex to filename")]
pub struct FileIndexing {
    _field: (),
}

#[derive(thiserror::Error, Debug)]
#[error("`Reflect::symbol_address` failed; could not find symbol address for this type.")]
pub struct MissingSymbolAddress {
    _field: (),
}

#[derive(thiserror::Error, Debug)]
#[error("Could not find debug info for symbol address.")]
pub struct MissingDebugInfo {
    _field: (),
}
