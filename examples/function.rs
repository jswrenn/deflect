use deflect::Reflect;

fn foo(val: u8) -> u32 {
    val as _
}

struct Wrapper<F>(F);

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let erased: &dyn Reflect = &Wrapper(foo);
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    println!("{value:#}");
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}
