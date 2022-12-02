use deflect::Reflect;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw = &1 as *const i32;
    let erased: &dyn Reflect = &raw;
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    println!("{value:}");
    Ok(())
}
