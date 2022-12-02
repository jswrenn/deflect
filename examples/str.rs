use deflect::Reflect;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let erased: &dyn Reflect = &"Hello World";
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    println!("{value:#}");
    Ok(())
}
