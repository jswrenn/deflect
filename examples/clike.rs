#![allow(dead_code)]

use deflect::Reflect;

#[repr(u64)]
enum TestCLikeEnum {
    A = 400,
    B,
    C,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let erased: &dyn Reflect = &TestCLikeEnum::B;
    let context = deflect::default_provider()?;
    let value: deflect::Value = erased.reflect(&context)?;
    let value: deflect::value::Enum = value.try_into()?;
    println!("{value}");
    Ok(())
}
