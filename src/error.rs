//! Error kinds.

pub(crate) fn tag_mismatch(
    expected: crate::gimli::DwTag,
    actual: crate::gimli::DwTag,
) -> crate::Error {
    anyhow!(
        "tag mismatch; expected {:?}, received {:?}",
        expected.static_string(),
        actual.static_string()
    )
}

pub(crate) fn missing_attr(attr: crate::gimli::DwAt) -> crate::Error {
    anyhow!("DIE did not have attribute attr {:?}", attr.static_string())
}

pub(crate) fn invalid_attr(attr: crate::gimli::DwAt) -> crate::Error {
    anyhow!(
        "attribute {:?} had an unexpected form",
        attr.static_string()
    )
}

pub(crate) fn missing_child(tag: crate::gimli::DwTag) -> crate::Error {
    anyhow!(
        "DIE did not have expected child of tag {:?}",
        tag.static_string()
    )
}

pub(crate) fn size_mismatch(expected: usize, actual: usize) -> crate::Error {
    anyhow!("size mismatch; expected {expected} bytes, found {actual}.")
}

pub(crate) fn name_mismatch(expected: &'static str, actual: String) -> crate::Error {
    anyhow!("name mismatch; expected {expected} bytes, found {actual}.")
}

pub(crate) fn file_indexing() -> crate::Error {
    anyhow!("could not map file index to a file name")
}

pub(crate) fn arithmetic_overflow() -> crate::Error {
    anyhow!("arithmetic operation overflowed")
}

pub(crate) fn enum_destructure() -> crate::Error {
    anyhow!("could not destructure enum into variant")
}

/// Could not downcast the value into the given type.
#[derive(thiserror::Error, Debug)]
#[error("Could not downcast into {src}, received {dst}")]
pub struct DowncastErr {
    src: &'static str,
    dst: &'static str,
}

impl DowncastErr {
    pub(crate) fn new<Src, Dst>() -> Self {
        let src = std::any::type_name::<Src>();
        let dst = std::any::type_name::<Dst>();
        Self { src, dst }
    }
}
