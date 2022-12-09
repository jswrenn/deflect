use deflect::Reflect;
use std::error::Error;

struct Foo;

trait Trait {}

impl Trait for Foo {}

fn main() -> Result<(), Box<dyn Error>> {
    let data: Box<dyn Trait> = Box::new(Foo);
    let erased: &dyn Reflect = &data;
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    let value: deflect::value::BoxedDyn<_> = value.try_into()?;
    println!("{value:#}");
    Ok(())
}
