enum Bar {
    A = 0xFF00,
    B = 0x00FF,
}

enum Foo {
    A(Bar),
    B,
}

fn main() {
    let foo = Foo::A(Bar::B);

    deflect::with_context(|ctx| {
        let val = deflect::reflect::<Foo, _>(&ctx, &foo);
        println!("{:#?}", val);
    })
    .unwrap();
}
