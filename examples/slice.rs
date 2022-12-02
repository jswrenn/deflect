use deflect::Reflect;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let erased: &dyn Reflect = &[1, 2, 3, 4].as_slice();
    let context = deflect::default_debuginfo();
    let value = erased.reflect(&context)?;
    println!("{value:#}");
    Ok(())
}
