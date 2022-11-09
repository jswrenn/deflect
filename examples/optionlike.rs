use deflect::Reflect;

enum OptionLike {
    Some(std::num::NonZeroU8),
    None,
}

fn main() -> Result<(), deflect::Error> {
    let erased: &dyn Reflect = &OptionLike::Some(std::num::NonZeroU8::new(42).unwrap());

    deflect::with_context(|ctx| {
        let value = erased.reflect(&ctx);
        println!("{:#?}", value);
    })?;

    Ok(())
}
