enum OptionLike {
    Some(std::num::NonZeroU8),
    None,
}

fn main() {
    deflect::with_context(|ctx| {
        let _ = deflect::reflect_type::<OptionLike, _>(&ctx);
    })
    .unwrap();
}
