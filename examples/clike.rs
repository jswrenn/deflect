use deflect::Reflect;

#[repr(u64)]
enum TestCLikeEnum {
    A = 400,
    B,
    C,
}

fn main() -> Result<(), deflect::error::Error> {
    let erased: &dyn Reflect = &TestCLikeEnum::B;
    let context = deflect::current_exe_debuginfo();
    let value = erased.reflect(&context)?;
    println!("{value}");
    Ok(())
}
