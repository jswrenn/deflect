use deflect::Reflect;
use std::error::Error;

fn foo(val: u8) -> u32 {
    val as _
}

struct Foo<F>(F);

fn run() -> Result<(), Box<dyn Error>> {
    let erased: &dyn Reflect = &Foo(foo);
    let context = deflect::default_debuginfo();
    let value = erased.reflect(&context)?;
    println!("{value:#}");
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err.to_string());
    }
}
