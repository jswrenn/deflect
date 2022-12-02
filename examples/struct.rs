#![allow(dead_code)]

use deflect::Reflect;

struct Example {
    foo: bool,
    bar: &'static str,
    baz: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value = Example {
        foo: true,
        bar: "Hello World!",
        baz: 42.0,
    };

    let erased: &dyn Reflect = &value;

    let dbginfo = deflect::default_provider()?;
    let reflection = erased.reflect(&dbginfo)?;

    assert_eq!(
        reflection.to_string(),
        r#"Example { foo: true, bar: "Hello World!", baz: 42 }"#
    );

    Ok(())
}
