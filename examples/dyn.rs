use deflect::Reflect;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    struct Struct;

    trait Trait {
        fn foo(&self) {}
        fn bar(&self) {}
    }

    impl Trait for Struct {}

    let raw = &Struct as &dyn Trait;
    let erased: &dyn Reflect = &raw;
    let context = deflect::current_exe_debuginfo();
    let value = erased.reflect(&context)?;
    let value: deflect::value::Struct<_> = value.try_into()?;

    println!("{value:#}");
    Ok(())
}
