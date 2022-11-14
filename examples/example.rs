use deflect::Reflect;

enum CLike {
    A = 1,
    B = 2,
    C = 3,
}

enum OptionLike {
    Some(CLike),
    None,
}

fn main() -> Result<(), deflect::Error> {
    let pandapandapanda = OptionLike::Some(CLike::B);
    let foo = async {
        drop(pandapandapanda);
    };

    let erased: &dyn Reflect = &foo;

    deflect::with_context(|ctx| {
        let value = erased.reflect(&ctx);
        println!("{:#?}", value);
    })?;

    Ok(())
}
