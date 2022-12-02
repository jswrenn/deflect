use std::fmt;
struct DisplayDebug<T>(T);

impl<T> fmt::Display for DisplayDebug<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[test]
fn phantom_data() -> Result<(), Box<dyn std::error::Error>> {
    use std::marker::PhantomData;
    let erased: &dyn deflect::Reflect = &PhantomData::<usize>;
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    let value: deflect::value::Struct<_> = value.try_into()?;
    assert_eq!(value.to_string(), "PhantomData<usize>");
    Ok(())
}

#[test]
fn unit_struct() -> Result<(), Box<dyn std::error::Error>> {
    struct UnitStruct;
    let erased: &dyn deflect::Reflect = &UnitStruct;
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    assert_eq!(value.to_string(), "UnitStruct");
    Ok(())
}

#[test]
fn tuple_struct() -> Result<(), Box<dyn std::error::Error>> {
    struct TupleStruct(u8);
    let erased: &dyn deflect::Reflect = &TupleStruct(42);
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    assert_eq!(value.to_string(), "TupleStruct { __0: 42 }");
    Ok(())
}

#[test]
fn braced_struct() -> Result<(), Box<dyn std::error::Error>> {
    struct BracedStruct {
        #[allow(dead_code)]
        foo: u8,
    }
    let erased: &dyn deflect::Reflect = &BracedStruct { foo: 42 };
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    assert_eq!(value.to_string(), "BracedStruct { foo: 42 }");
    println!("{:#}", DisplayDebug(&value));
    Ok(())
}

mod r#ref {
    #[test]
    fn unit_struct() -> Result<(), Box<dyn std::error::Error>> {
        struct UnitStruct;
        let erased: &dyn deflect::Reflect = &UnitStruct;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(value.to_string(), "UnitStruct");
        Ok(())
    }
}

mod slice {
    use itertools::Itertools;

    #[quickcheck_macros::quickcheck]
    fn of_units(data: Vec<()>) -> Result<(), Box<dyn std::error::Error>> {
        let slice = data.as_slice();
        let erased: &dyn deflect::Reflect = &slice;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        let value: deflect::value::Slice<_> = value.try_into()?;

        assert_eq!(data.len(), value.len()?);
        assert_eq!(data.len(), value.iter()?.count());

        let collected: Vec<_> = value
            .iter()?
            .map_ok(|value| <()>::try_from(value))
            .flatten_ok()
            .try_collect()?;

        assert_eq!(data, collected);

        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn of_chars(data: Vec<char>) -> Result<(), Box<dyn std::error::Error>> {
        let slice = data.as_slice();
        let erased: &dyn deflect::Reflect = &slice;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        let value: deflect::value::Slice<_> = value.try_into()?;

        assert_eq!(data.len(), value.len()?);
        assert_eq!(data.len(), value.iter()?.count());

        let collected: Vec<_> = value
            .iter()?
            .map_ok(|value| char::try_from(value))
            .flatten_ok()
            .try_collect()?;

        assert_eq!(data, collected);

        Ok(())
    }
}

#[quickcheck_macros::quickcheck]
fn str(data: String) -> Result<(), Box<dyn std::error::Error>> {
    let slice = data.as_str();
    let erased: &dyn deflect::Reflect = &slice;
    let context = deflect::default_provider()?;
    let value = erased.reflect(&context)?;
    let value: &str = value.try_into()?;
    assert_eq!(slice, value);
    Ok(())
}

mod primitive {

    use std::ptr;

    #[quickcheck_macros::quickcheck]
    fn unit(n: ()) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!("()", value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn bool(n: bool) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn char(n: char) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn f32(n: f32) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn f64(n: f64) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn i8(n: i8) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn i16(n: i16) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn i32(n: i32) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn i64(n: i64) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn i128(n: i128) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn isize(n: isize) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn u8(n: u8) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn u16(n: u16) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn u32(n: u32) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn u64(n: u64) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn u128(n: u128) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }

    #[quickcheck_macros::quickcheck]
    fn usize(n: usize) -> Result<(), Box<dyn std::error::Error>> {
        let erased: &dyn deflect::Reflect = &n;
        let context = deflect::default_provider()?;
        let value = erased.reflect(&context)?;
        assert_eq!(n.to_string(), value.to_string());
        assert!(ptr::eq(
            &n,
            <&_>::try_from(value).expect("failed to downcast")
        ));
        Ok(())
    }
}
