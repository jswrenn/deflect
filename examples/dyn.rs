use deflect::Reflect;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    struct Struct;

    trait Trait {
        fn foo(&self) {}
        fn bar(&self) {}
    }

    impl Trait for Struct {}

    let raw = &Struct as &dyn Trait;
    let erased: &dyn Reflect = &raw;
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    let value: deflect::value::Struct<_> = value.try_into()?;

    println!("{value:#}");
    Ok(())
}
