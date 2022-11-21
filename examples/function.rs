use deflect::Reflect;
use std::error::Error;

fn foo(val: u8) -> u32 {
    val as _
}

fn main() -> Result<(), Box<dyn Error>> {
    let erased: &dyn Reflect = &foo;
    let context = deflect::current_exe_debuginfo();
    let value = erased.reflect(&context)?;
    println!("{value:#}");
    Ok(())
}
