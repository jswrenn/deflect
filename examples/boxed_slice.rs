use deflect::Reflect;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = vec![1, 2, 3].into_boxed_slice();
    let erased: &dyn Reflect = &data;
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    println!("{value:#}");
    Ok(())
}
