fn main() -> Result<(), deflect::Error> {
    let x = 42;
    let foo = async {
        drop(x);
    };

    deflect::with_context(|ctx| {
        let val = deflect::reflect::<_, _>(&ctx, &foo);
        println!("{:#?}", val);
    })
    .unwrap();
    Ok(())
}
