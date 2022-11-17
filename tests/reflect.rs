#[test]
fn phantom_data() -> Result<(), deflect::Error> {
    use std::marker::PhantomData;
    let erased: &dyn deflect::Reflect = &PhantomData::<usize>;
    let context = deflect::current_exe_debuginfo();
    let value = erased.reflect(&context)?;
    assert_eq!(value.to_string(), "PhantomData<usize>");
    Ok(())
}

#[test]
fn unit_struct() -> Result<(), deflect::Error> {
    struct UnitStruct;
    let erased: &dyn deflect::Reflect = &UnitStruct;
    let context = deflect::current_exe_debuginfo();
    let value = erased.reflect(&context)?;
    assert_eq!(value.to_string(), "UnitStruct");
    Ok(())
}

#[test]
fn tuple_struct() -> Result<(), deflect::Error> {
    struct TupleStruct(u8);
    let erased: &dyn deflect::Reflect = &TupleStruct(42);
    let context = deflect::current_exe_debuginfo();
    let value = erased.reflect(&context)?;
    assert_eq!(value.to_string(), "TupleStruct { __0: 42 }");
    Ok(())
}

#[test]
fn braced_struct() -> Result<(), deflect::Error> {
    struct BracedStruct {
        #[allow(dead_code)]
        foo: u8,
    }
    let erased: &dyn deflect::Reflect = &BracedStruct { foo: 42 };
    let context = deflect::current_exe_debuginfo();
    let value = erased.reflect(&context)?;
    assert_eq!(value.to_string(), "BracedStruct { foo: 42 }");
    Ok(())
}

mod primitive {
    use std::{error::Error, ptr};

    #[quickcheck_macros::quickcheck]
    fn bool(n: bool) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn char(n: char) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn f32(n: f32) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn f64(n: f64) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn i8(n: i8) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn i16(n: i16) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn i32(n: i32) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn i64(n: i64) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn i128(n: i128) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn isize(n: isize) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn u8(n: u8) -> Result<(), Box<dyn Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn u16(n: u16) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn u32(n: u32) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn u64(n: u64) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn u128(n: u128) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn usize(n: usize) -> Result<(), deflect::Error> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::current_exe_debuginfo();
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }
}
