use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    fmt::{self, Formatter},
};

pub(crate) fn inspect<R>(
    output: &mut Formatter<'_>,
    dwarf: &crate::gimli::Dwarf<R>,
    unit: &crate::gimli::Unit<R>,
    entry: &crate::gimli::DebuggingInformationEntry<R, usize>,
) -> Result<(), anyhow::Error>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    let mut tree = unit.entries_tree(Some(entry.offset())).unwrap();
    inspect_tree(output, &mut tree, dwarf, unit).unwrap();
    Ok(())
}

pub(crate) fn inspect_tree<R>(
    output: &mut Formatter<'_>,
    tree: &mut crate::gimli::EntriesTree<R>,
    dwarf: &crate::gimli::Dwarf<R>,
    unit: &crate::gimli::Unit<R>,
) -> Result<(), anyhow::Error>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    inspect_tree_node(output, tree.root()?, dwarf, unit, 0)
}

fn inspect_tree_node<R>(
    output: &mut Formatter<'_>,
    node: crate::gimli::EntriesTreeNode<R>,
    dwarf: &crate::gimli::Dwarf<R>,
    unit: &crate::gimli::Unit<R>,
    depth: isize,
) -> Result<(), anyhow::Error>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    inspect_entry(output, node.entry(), dwarf, unit, depth)?;
    let mut children = node.children();
    while let Some(child) = children.next()? {
        inspect_tree_node(output, child, dwarf, unit, depth + 1)?;
    }
    Ok(())
}

pub(crate) fn inspect_entry<R>(
    output: &mut Formatter<'_>,
    entry: &crate::gimli::DebuggingInformationEntry<R, usize>,
    dwarf: &crate::gimli::Dwarf<R>,
    unit: &crate::gimli::Unit<R>,
    _depth: isize,
) -> Result<(), anyhow::Error>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    let mut debug_struct = output.debug_struct(&format!(
        "<0x{offset:x}> {tag:?}\n",
        offset = entry.offset().0,
        tag = entry.tag().static_string().expect("Unknown tag kind."),
    ));

    let mut attrs = entry.attrs();
    while let Some(attr) = attrs.next()? {
        match dwarf.attr_string(unit, attr.value()) {
            Ok(r) => {
                let val = r.to_string_lossy()?;
                match &*attr.name().to_string() {
                    name @ "DW_AT_MIPS_linkage_name" => {
                        let val = rustc_demangle::demangle(&val);
                        debug_struct.field(name, &val);
                    }
                    name => {
                        debug_struct.field(name, &val);
                    }
                }
            }
            _ if attr.exprloc_value().is_some() => {
                let expression = attr.exprloc_value().unwrap();
                debug_struct.field(
                    attr.name().static_string().unwrap(),
                    &DebugExpression::new(unit, expression),
                );
            }
            _ => {
                if let (
                    crate::gimli::DW_AT_decl_file,
                    crate::gimli::AttributeValue::FileIndex(file_index),
                ) = (attr.name(), attr.value())
                {
                    let path = crate::fi_to_string(file_index, unit)?;
                    debug_struct.field(attr.name().static_string().unwrap(), &path);
                } else {
                    debug_struct.field(attr.name().static_string().unwrap(), &attr.value());
                }
            }
        }
    }
    Ok(())
}

// -----

struct DebugExpression<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    expression: Cell<Option<crate::gimli::Expression<R>>>,
}

impl<'dwarf, R> DebugExpression<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn new(
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        expression: crate::gimli::Expression<R>,
    ) -> Self {
        let expression = Cell::new(Some(expression));
        Self { unit, expression }
    }
}

impl<'dwarf, R> fmt::Debug for DebugExpression<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_list = f.debug_list();
        let expression = self.expression.take().unwrap();
        let mut ops = expression.operations(self.unit.encoding());
        while let Some(op) = ops.next().unwrap() {
            debug_list.entry(&op);
        }

        Ok(())
    }
}

// -----
pub(super) struct DebugEntry<'entry, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R>,
    entry: &'entry crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R, usize>,
}

impl<'entry, 'dwarf, R> DebugEntry<'entry, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) fn new(
        dwarf: &'dwarf crate::gimli::Dwarf<R>,
        unit: &'dwarf crate::gimli::Unit<R>,
        entry: &'entry crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R, usize>,
    ) -> Self {
        Self { dwarf, unit, entry }
    }
}

impl<'entry, 'dwarf, R> fmt::Debug for DebugEntry<'entry, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct(&dw_tag_to_string(self.entry.tag()));
        let mut attrs = self.entry.attrs();
        while let Some(attr) = attrs.next().unwrap() {
            let name = attr.name();
            let value = attr.value();
            if let Ok(value_as_string) = self.dwarf.attr_string(self.unit, value) {
                if let Ok(value_as_string) = value_as_string.to_string_lossy() {
                    if name == crate::gimli::DW_AT_MIPS_linkage_name {
                        let value_as_string = rustc_demangle::demangle(&value_as_string);
                        debug_struct.field(&dw_at_to_string(name), &value_as_string);
                    } else {
                        debug_struct.field(&dw_at_to_string(name), &value_as_string);
                    }
                } else {
                    debug_struct.field(&dw_at_to_string(name), &value_as_string);
                }
                continue;
            }
            let value = attr.value();
            if let Some(expression) = attr.exprloc_value() {
                debug_struct.field(
                    &dw_at_to_string(name),
                    &DebugExpression::new(self.unit, expression),
                );
            }
            if let Some(value) = attr.udata_value() {
                debug_struct.field(&dw_at_to_string(name), &value);
            } else if let crate::gimli::AttributeValue::FileIndex(file_index) = value {
                if let Ok(value_as_string) = crate::fi_to_string(file_index, self.unit) {
                    debug_struct.field(&dw_at_to_string(name), &value_as_string);
                } else {
                    debug_struct.field(&dw_at_to_string(name), &file_index);
                }
            } else if let crate::gimli::AttributeValue::Encoding(encoding) = value {
                debug_struct.field(&dw_at_to_string(name), &dw_ate_to_string(encoding));
            } else {
                debug_struct.field(&dw_at_to_string(name), &value);
            }
        }
        if self.entry.has_children() {
            let mut tree = self.unit.entries_tree(Some(self.entry.offset())).unwrap();
            let root = tree.root().unwrap();
            let children = RefCell::new(root.children());
            debug_struct.field(
                "children",
                &DebugEntriesTreeIter {
                    dwarf: self.dwarf,
                    unit: self.unit,
                    iter: children,
                },
            );
        }
        debug_struct.finish()
    }
}

fn dw_tag_to_string(at: crate::gimli::DwTag) -> Cow<'static, str> {
    if let Some(name) = at.static_string() {
        Cow::Borrowed(name)
    } else {
        Cow::Owned(format!("{}", at.0))
    }
}

fn dw_at_to_string(at: crate::gimli::DwAt) -> Cow<'static, str> {
    if let Some(name) = at.static_string() {
        Cow::Borrowed(name)
    } else {
        Cow::Owned(format!("{}", at.0))
    }
}

fn dw_ate_to_string(at: crate::gimli::DwAte) -> Cow<'static, str> {
    if let Some(name) = at.static_string() {
        Cow::Borrowed(name)
    } else {
        Cow::Owned(format!("{}", at.0))
    }
}

struct DebugEntriesTreeIter<'tree, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    dwarf: &'dwarf crate::gimli::Dwarf<R>,
    unit: &'dwarf crate::gimli::Unit<R>,
    iter: RefCell<crate::gimli::EntriesTreeIter<'dwarf, 'dwarf, 'tree, R>>,
}

impl<'tree, 'dwarf, R> fmt::Debug for DebugEntriesTreeIter<'tree, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_list = f.debug_list();
        let mut iter = self.iter.borrow_mut();
        while let Some(child) = iter.next().unwrap() {
            let entry = child.entry();
            debug_list.entry(&DebugEntry {
                dwarf: self.dwarf,
                unit: self.unit,
                entry,
            });
        }
        debug_list.finish()
    }
}
