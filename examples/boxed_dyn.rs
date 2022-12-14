use deflect::Reflect;
use std::error::Error;

struct Foo;

trait Trait {}

impl Trait for Foo {}

fn main() -> Result<(), Box<dyn Error>> {
    // initialize the debuginfo provider
    let context = deflect::default_provider()?;

    // create some type-erased data
    let data: Box<dyn Trait> = Box::new(Foo);

    // cast it to `&dyn Reflect`
    let erased: &dyn Reflect = &data;

    // reflect it!
    let value: deflect::Value = erased.reflect(&context)?;

    // pretty-print the reflected value
    assert_eq!(value.to_string(), "box Foo");

    // downcast into a `BoxedDyn` value
    let value: deflect::value::BoxedDyn = value.try_into()?;

    // dereference the boxed value
    let value: deflect::Value = value.deref()?;
    // downcast into a `Struct` value
    let value: deflect::value::Struct = value.try_into()?;

    // pretty-print the reflected value
    assert_eq!(value.to_string(), "Foo");

    Ok(())
}
