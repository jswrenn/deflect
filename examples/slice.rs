use deflect::Reflect;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let erased: &dyn Reflect = &[1, 2, 3, 4].as_slice();
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    println!("{value:#}");
    Ok(())
}
