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

fn main() -> Result<(), deflect::error::Error> {
    let x = 42;
    let pandapandapanda = OptionLike::Some(CLike::B);
    let foo = async move {
        drop(x);
        drop(pandapandapanda);
    };

    let erased: &dyn Reflect = &foo;
    let context = deflect::current_exe_debuginfo();
    let value = erased.reflect(&context)?;
    println!("{value}");

    Ok(())
}
