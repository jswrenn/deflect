use deflect::Reflect;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let raw = &1 as *const i32;
    let erased: &dyn Reflect = &raw;
    let context = deflect::default_debuginfo();
    let value = erased.reflect(&context)?;
    println!("{value:}");
    Ok(())
}
