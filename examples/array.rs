use deflect::Reflect;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let erased: &dyn Reflect = &[1, 2, 3, 4];
    let context = deflect::default_provider()?;
    let value: deflect::Value = erased.reflect(&context)?;
    let value: deflect::value::Array = value.try_into()?;
    println!("{value:#}");
    Ok(())
}
