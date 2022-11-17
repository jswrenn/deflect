use std::fmt;

pub(crate) fn inspect_tree<W, R>(
    output: &mut W,
    tree: &mut crate::gimli::EntriesTree<R>,
    dwarf: &crate::gimli::Dwarf<R>,
    unit: &crate::gimli::Unit<R>,
) -> Result<(), anyhow::Error>
where
    W: fmt::Write,
    R: crate::gimli::Reader<Offset = usize>,
{
    inspect_tree_node(output, tree.root()?, dwarf, unit, 0)
}

fn inspect_tree_node<W, R>(
    output: &mut W,
    node: crate::gimli::EntriesTreeNode<R>,
    dwarf: &crate::gimli::Dwarf<R>,
    unit: &crate::gimli::Unit<R>,
    depth: isize,
) -> Result<(), anyhow::Error>
where
    W: fmt::Write,
    R: crate::gimli::Reader<Offset = usize>,
{
    inspect_entry(output, node.entry(), dwarf, unit, depth)?;
    let mut children = node.children();
    while let Some(child) = children.next()? {
        inspect_tree_node(output, child, dwarf, unit, depth + 1)?;
    }
    Ok(())
}

pub(crate) fn inspect_entry<W, R>(
    output: &mut W,
    entry: &crate::gimli::DebuggingInformationEntry<R, usize>,
    dwarf: &crate::gimli::Dwarf<R>,
    unit: &crate::gimli::Unit<R>,
    depth: isize,
) -> Result<(), anyhow::Error>
where
    W: fmt::Write,
    R: crate::gimli::Reader<Offset = usize>,
{
    let indent = (depth * 4).try_into().unwrap_or(0);
    eprintln!(
        "{:indent$} <0x{offset:x}> {tag:?}",
        "",
        offset = entry.offset().0,
        tag = entry.tag().static_string().expect("Unknown tag kind."),
    );
    let mut attrs = entry.attrs();
    while let Some(attr) = attrs.next()? {
        match dwarf.attr_string(unit, attr.value()) {
            Ok(r) => {
                let val = r.to_string_lossy()?;
                match &*attr.name().to_string() {
                    "DW_AT_MIPS_linkage_name" => {
                        let val = rustc_demangle::demangle(&val);
                        eprintln!("{:indent$}    {}: {:?}", "", attr.name(), val)
                    }
                    _ => eprintln!("{:indent$}    {}: {:?}", "", attr.name(), val),
                }
            }
            _ if attr.exprloc_value().is_some() => {
                eprint!("{:indent$}    {} [", "", attr.name());
                let mut ops = attr.exprloc_value().unwrap().operations(unit.encoding());
                while let Some(op) = ops.next()? {
                    eprint!("{op:?}, ");
                }
                eprintln!("]");
            }
            _ => {
                if let (
                    crate::gimli::DW_AT_decl_file,
                    crate::gimli::AttributeValue::FileIndex(file_index),
                ) = (attr.name(), attr.value())
                {
                    let path = crate::fi_to_string(file_index, unit)?;
                    eprintln!("{:indent$}    {}: {:?}", "", attr.name(), path);
                } else {
                    eprintln!("{:indent$}    {}: {:?}", "", attr.name(), attr.value())
                }
            }
        }
    }
    Ok(())
}
