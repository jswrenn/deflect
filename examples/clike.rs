use deflect::Reflect;

enum TestCLikeEnum {
    A,
    B,
    C,
}

fn main() -> Result<(), deflect::Error> {
    let erased: &dyn Reflect = &TestCLikeEnum::B;

    deflect::with_context(|ctx| {
        let value = erased.reflect(&ctx);
        println!("{:#?}", value);
    })?;

    Ok(())
}

