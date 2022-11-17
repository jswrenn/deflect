use deflect::Reflect;

enum OptionLike {
    Some(std::num::NonZeroU8),
    None,
}

fn main() -> Result<(), deflect::Error> {
    let erased: &dyn Reflect = &OptionLike::Some(std::num::NonZeroU8::new(42).unwrap());
    let context = deflect::current_exe_debuginfo();
    let value = erased.reflect(&context)?;
    println!("{value:#}");
    Ok(())
}
