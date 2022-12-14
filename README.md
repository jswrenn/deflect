<!-- Do not edit README.md manually. Instead, edit the module comment of `src/lib.rs`. -->

# deflect

**Debug anything.** Deflect brings reflection to Rust using [DWARF] debug
info.

Deflect can be used to recover the concrete types of trait objects, inspect
the internal state of `async` generators, and pretty-print arbitrary data.

[DWARF]: https://en.wikipedia.org/wiki/DWARF

## Example
Use the [`Reflect`] trait to debug or recursively destructure any value.

```rust
struct Foo;
trait Trait {}
impl Trait for Foo {}

// initialize the debuginfo provider
let context = deflect::default_provider()?;

// create some type-erased data
let data: Box<dyn Trait> = Box::new(Foo);

// cast it to `&dyn Reflect`
let erased: &dyn Reflect = &data;

// reflect it!
let value: deflect::Value = erased.reflect(&context)?;

// pretty-print the reflected value
assert_eq!(value.to_string(), "box Foo");

// downcast into a `BoxedDyn` value
let value: deflect::value::BoxedDyn = value.try_into()?;

// dereference the boxed value
let value: deflect::Value = value.deref()?;
// downcast into a `Struct` value
let value: deflect::value::Struct = value.try_into()?;

// pretty-print the reflected value
assert_eq!(value.to_string(), "Foo");
#Ok(())
```

## License

This project is licensed under the Apache License, Version 2.0, or the MIT
license, at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in deflect by you, shall be licensed as MIT and Apache 2.0,
without any additional terms or conditions.
