use deflect::Reflect;
use std::error::Error;



fn main() -> Result<(), Box<dyn Error>> {
    #[allow(dead_code)]
    struct Foo {
        a: u8,
    }

    // initialize the debuginfo provider
    let context = deflect::default_provider()?;

    // create some type-erased data
    let data: Box<dyn std::any::Any> = Box::new(Foo { a: 42 });

    // cast it to `&dyn Reflect`
    let erased: &dyn Reflect = &data;

    // reflect it!
    let value: deflect::Value = erased.reflect(&context)?;

    // pretty-print the reflected value
    assert_eq!(value.to_string(), "box Foo { a: 42 }");

    // downcast into a `BoxedDyn` value
    let value: deflect::value::BoxedDyn = value.try_into()?;

    // dereference the boxed value
    let value: deflect::Value = value.deref()?;
    // downcast into a `Struct` value
    let value: deflect::value::Struct = value.try_into()?;

    // pretty-print the reflected value
    assert_eq!(value.to_string(), "Foo { a: 42 }");

    Ok(())
}
