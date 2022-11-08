#![feature(once_cell)]

use addr2line::Context;
use gimli::{AttributeValue, EndianReader, Reader, RunTimeEndian, UnitOffset};

use anyhow::anyhow;
use std::{
    ffi::c_void,
    mem::{self, MaybeUninit},
    ptr::slice_from_raw_parts,
    rc::Rc,
};

mod schema;
mod r#value;

pub use r#schema::Type;
pub use r#value::Value;

type Byte = MaybeUninit<u8>;
type Bytes<'value> = &'value [Byte];

pub type Addr2LineContext = Context<EndianReader<RunTimeEndian, Rc<[u8]>>>;

pub fn with_context<F>(f: F) -> anyhow::Result<()>
where
    F: FnOnce(Context<EndianReader<RunTimeEndian, Rc<[u8]>>>),
{
    let file = current_binary().ok_or(anyhow!("Could not open current binary"))?;
    let mmap = unsafe { memmap2::Mmap::map(&file).unwrap() };
    let object = object::File::parse(&*mmap)?;
    let ctx = addr2line::Context::new(&object)?;
    f(ctx);
    Ok(())
}

/// Produces the DWARF unit and entry offset for the DIE of `T`.
fn dw_unit_and_die_of<'ctx, T: ?Sized, R>(
    ctx: &'ctx Context<R>,
) -> anyhow::Result<(&'ctx gimli::Unit<R>, gimli::UnitOffset)>
where
    R: gimli::Reader<Offset = usize>,
{
    /// Produces the symbol address of itself.
    #[inline(never)]
    fn symbol_addr<T: ?Sized>() -> Option<*mut c_void> {
        let ip = (symbol_addr::<T> as usize + 1) as *mut c_void;
        let mut symbol_addr = None;
        backtrace::resolve(ip, |symbol| {
            symbol_addr = symbol.addr();
        });
        symbol_addr
    }

    let Some(symbol_addr) = symbol_addr::<T>() else {
        return Err(anyhow!("Could not find symbol address for `symbol_addr::<T>`."))
    };

    let dw_die_offset = ctx
        .find_frames(symbol_addr as u64)?
        .next()?
        .and_then(|f| f.dw_die_offset)
        .ok_or(anyhow!("Could not find debug info for `symbol_addr::<T>`."))?;

    let unit = ctx.find_dwarf_unit(symbol_addr as u64).unwrap();

    let mut ty = None;
    let mut tree = unit.entries_tree(Some(dw_die_offset))?;
    let mut children = tree.root()?.children();

    while let Some(child) = children.next()? {
        if ty.is_none() && child.entry().tag() == gimli::DW_TAG_template_type_parameter {
            ty = get_type(child.entry())?;
            break;
        }
    }

    let ty = ty.ok_or(anyhow!("Could not find parameter type entry"))?;

    Ok((unit, ty))
}

pub fn reflect<'ctx, 'value, T: ?Sized, R>(
    ctx: &'ctx Context<R>,
    value: &'value T,
) -> anyhow::Result<value::Value<'ctx, 'value, R>>
where
    R: gimli::Reader<Offset = usize>,
{
    let r#type = reflect_type::<T, _>(ctx)?;
    let value = slice_from_raw_parts(value as *const T as *const Byte, mem::size_of_val(value));
    let value = unsafe { &*value };
    Ok(unsafe { value::Value::with_type(r#type, value) })
}

pub fn reflect_type<'ctx, T: ?Sized, R>(
    ctx: &'ctx Context<R>,
) -> anyhow::Result<Type<'ctx, R>>
where
    R: gimli::Reader<Offset = usize>,
{
    let (unit, offset) = dw_unit_and_die_of::<T, _>(ctx)?;

    let mut tree = unit.entries_tree(Some(offset))?;
    inspect_tree(&mut tree, ctx.dwarf(), unit);

    let die = unit.entry(offset).unwrap();
    Ok(Type::from_die(ctx.dwarf(), unit, die))
}

fn current_binary() -> Option<std::fs::File> {
    let file = std::fs::File::open("/proc/self/exe").ok()?;
    Some(file)
}

fn get_name<R: gimli::Reader<Offset = usize>>(
    entry: &gimli::DebuggingInformationEntry<R>,
    dwarf: &gimli::Dwarf<R>,
    unit: &gimli::Unit<R, usize>,
) -> anyhow::Result<Option<R>> {
    let Some(name) = entry.attr_value(gimli::DW_AT_name)? else { return Ok(None) };
    let name = dwarf.attr_string(unit, name)?;
    Ok(Some(name))
}

pub(crate) fn get_type<R: gimli::Reader<Offset = usize>>(
    entry: &gimli::DebuggingInformationEntry<R>,
) -> Result<Option<UnitOffset>, anyhow::Error> {
    get_attr_ref(entry, gimli::DW_AT_type)
}

fn get_attr_ref<R: gimli::Reader<Offset = usize>>(
    entry: &gimli::DebuggingInformationEntry<R>,
    name: gimli::DwAt,
) -> Result<Option<UnitOffset>, anyhow::Error> {
    if let Some(attr) = entry.attr(name)? {
        if let AttributeValue::UnitRef(offset) = attr.value() {
            return Ok(Some(offset));
        }
    }
    Ok(None)
}

fn eval_addr<R>(
    unit: &gimli::Unit<R>,
    attr: AttributeValue<R>,
    start: u64,
) -> Result<Option<u64>, anyhow::Error>
where
    R: gimli::Reader<Offset = usize>,
{
    if let Some(loc) = attr.exprloc_value() {
        // TODO: We probably don't need full evaluation here and can
        // just support PlusConstant.
        let mut eval = loc.evaluation(unit.encoding());
        eval.set_initial_value(start);
        if let gimli::EvaluationResult::Complete = eval.evaluate()? {
            let result = eval.result();
            match result[..] {
                [gimli::Piece {
                    size_in_bits: None,
                    bit_offset: None,
                    location: gimli::Location::Address { address },
                }] => return Ok(Some(address)),
                _ => eprintln!("Warning: Unsupported evaluation result {:?}", result,),
            }
        }
    } else if let AttributeValue::Udata(offset) = attr {
        return Ok(Some(start + offset));
    }
    Ok(None)
}

fn fi_to_string<'a, R: gimli::Reader<Offset = usize> + 'a>(
    file_index: u64,
    unit: &'a gimli::Unit<R>,
) -> Result<String, anyhow::Error> {
    let line_program = unit
        .line_program
        .as_ref()
        .ok_or(anyhow!("no lineprogram"))?;
    let file = line_program
        .header()
        .file(file_index)
        .ok_or(anyhow!("no such file"))?;
    let AttributeValue::String(ref bytes) = file.path_name() else {
        return Err(anyhow!("path name was not a string"));
    };
    let path = bytes.to_string_lossy().unwrap().into_owned();
    Ok(path)
}

fn inspect_tree<R: gimli::Reader<Offset = usize>>(
    tree: &mut gimli::EntriesTree<R>,
    dwarf: &gimli::Dwarf<R>,
    unit: &gimli::Unit<R>,
) -> Result<(), anyhow::Error> {
    inspect_tree_node(tree.root()?, dwarf, unit, 0)
}

fn inspect_tree_node<R: gimli::Reader<Offset = usize>>(
    node: gimli::EntriesTreeNode<R>,
    dwarf: &gimli::Dwarf<R>,
    unit: &gimli::Unit<R>,
    depth: isize,
) -> Result<(), anyhow::Error> {
    inspect_entry(node.entry(), dwarf, unit, depth)?;
    let mut children = node.children();
    while let Some(child) = children.next()? {
        inspect_tree_node(child, dwarf, unit, depth + 1)?;
    }
    Ok(())
}

fn inspect_entry<R: gimli::Reader<Offset = usize>>(
    entry: &gimli::DebuggingInformationEntry<R, usize>,
    dwarf: &gimli::Dwarf<R>,
    unit: &gimli::Unit<R>,
    depth: isize,
) -> Result<(), anyhow::Error> {
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
                if let (gimli::DW_AT_decl_file, AttributeValue::FileIndex(file_index)) =
                    (attr.name(), attr.value())
                {
                    let path = fi_to_string(file_index, unit)?;
                    eprintln!("{:indent$}    {}: {:?}", "", attr.name(), path);
                } else {
                    eprintln!("{:indent$}    {}: {:?}", "", attr.name(), attr.value())
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
