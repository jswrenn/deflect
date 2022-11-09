use deflect::Reflect;

fn main() -> Result<(), deflect::Error> {
    let x = 42;
    let foo = async {
        drop(x);
    };

    let erased: &dyn Reflect = &foo;

    deflect::with_context(|ctx| {
        let value = erased.reflect(&ctx);
        println!("{:#?}", value);
    })?;

    Ok(())
}
