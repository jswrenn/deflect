#![allow(dead_code)]

use deflect::Reflect;

enum OptionLike {
    Some(std::num::NonZeroU8),
    None,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let erased: &dyn Reflect = &OptionLike::Some(std::num::NonZeroU8::new(42).unwrap());
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    println!("{value:#}");
    Ok(())
}
