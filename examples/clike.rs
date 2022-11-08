enum TestCLikeEnum {
    A,
    B,
    C,
}

fn main() {
    deflect::with_context(|ctx| {
        let _ = deflect::reflect_type::<TestCLikeEnum, _>(&ctx);
    })
    .unwrap();
}
