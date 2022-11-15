use deflect::Reflect;

enum TestCLikeEnum {
    A,
    B,
    C,
}

fn main() -> Result<(), deflect::Error> {
    let erased: &dyn Reflect = &TestCLikeEnum::B;
    let context = deflect::current_exe_debuginfo();
    let value = erased.reflect(&context);
    println!("{:#?}", value);
    Ok(())
}
