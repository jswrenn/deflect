use std::backtrace::Backtrace;

#[derive(thiserror::Error, Debug)]
#[error("{}\n{}", self.kind, self.backtrace)]
pub struct Error {
    pub kind: Kind,
    pub backtrace: Backtrace,
}

impl<E> From<E> for Error
where
    Kind: From<E>,
{
    fn from(error: E) -> Self {
        Self {
            kind: error.into(),
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
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
    TryFromInt(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    Downcast(#[from] Downcast),
    #[error(transparent)]
    Gimli(#[from] crate::gimli::Error),
}

impl Kind {
    pub fn downcast(src: &'static str, dst: &'static str) -> Self {
        Self::Downcast(Downcast { src, dst })
    }

    pub fn tag_mismatch(expected: crate::gimli::DwTag, actual: crate::gimli::DwTag) -> Self {
        Self::TagMismatch(TagMismatch { expected, actual })
    }

    pub fn missing_attr(attr: crate::gimli::DwAt) -> Self {
        Self::MissingAttr(MissingAttr { attr })
    }

    pub fn invalid_attr(attr: crate::gimli::DwAt) -> Self {
        Self::InvalidAttr(InvalidAttr { attr })
    }

    pub fn missing_child(tag: crate::gimli::DwTag) -> Self {
        Self::MissingChild(MissingChild { tag })
    }

    pub fn missing_entry(offset: crate::gimli::UnitOffset) -> Self {
        Self::MissingEntry(MissingEntry { offset })
    }

    pub fn size_mismatch(expected: usize, actual: usize) -> Self {
        Self::SizeMismatch(SizeMismatch { expected, actual })
    }

    pub fn name_mismatch(expected: &'static str, actual: String) -> Self {
        Self::NameMismatch(NameMismatch { expected, actual })
    }

    pub fn try_from_int(err: std::num::TryFromIntError) -> Self {
        Self::TryFromInt(err)
    }

    pub fn gimli(err: crate::gimli::Error) -> Self {
        Self::Gimli(err)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("tag mismatch; expected {:?}, received {:?}", .expected.static_string(), .actual.static_string())]
pub struct TagMismatch {
    expected: crate::gimli::DwTag,
    actual: crate::gimli::DwTag,
}

impl TagMismatch {
    pub fn new(expected: crate::gimli::DwTag, actual: crate::gimli::DwTag) -> Self {
        Self { expected, actual }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("DIE did not have the attribute {:?}", .attr.static_string())]
pub struct MissingAttr {
    attr: crate::gimli::DwAt,
}

impl MissingAttr {
    pub fn new(attr: crate::gimli::DwAt) -> Self {
        Self { attr }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("The attribute {:?} had an invalid value.", .attr.static_string())]
pub struct InvalidAttr {
    attr: crate::gimli::DwAt,
}

impl InvalidAttr {
    pub fn new(attr: crate::gimli::DwAt) -> Self {
        Self { attr }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("die did not have the child {tag}")]
pub struct MissingChild {
    tag: crate::gimli::DwTag,
}

impl MissingChild {
    pub fn new(tag: crate::gimli::DwTag) -> Self {
        Self { tag }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("unit did not have entry at offset=0x{offset:x}", offset = .offset.0)]
pub struct MissingEntry {
    offset: crate::gimli::UnitOffset,
}

impl MissingEntry {
    pub fn new(offset: crate::gimli::UnitOffset) -> Self {
        Self { offset }
    }
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
    pub fn new<Src, Dst>() -> Self {
        let src = std::any::type_name::<Src>();
        let dst = std::any::type_name::<Dst>();
        Self { src, dst }
    }
}
