<!-- Do not edit README.md manually. Instead, edit the module comment of `src/lib.rs`. -->

# deflect

**[EXPERIMENTAL]**
Deflect brings reflection to Rust using [DWARF] debug info.

Deflect can be used to recover the concrete types of trait objects, inspect
the internal state of `async` generators, and pretty-print arbitrary data.

[DWARF]: https://en.wikipedia.org/wiki/DWARF

## Example
Use the [`Reflect`] trait to debug or recursively destructure any value.

```rust
pub struct Foo {
    a: u8
}

// initialize the debuginfo provider
let context = deflect::default_provider()?;

// create some type-erased data
let erased: Box<dyn Any> = Box::new(Foo { a: 42 });

// cast it to `&dyn Reflect`
let reflectable: &dyn deflect::Reflect = &erased;

// reflect it!
let value: deflect::Value = reflectable.reflect(&context)?;

// pretty-print the reflected value
assert_eq!(value.to_string(), "box Foo { a: 42 }");

// downcast into a `BoxedDyn` value
let value: deflect::value::BoxedDyn = value.try_into()?;

// dereference the boxed value
let value: deflect::Value = value.deref()?;

// downcast into a `Struct` value
let value: deflect::value::Struct = value.try_into()?;

// get the field `a` by name
let Some(field) = value.field("a")? else {
    panic!("no field named `a`!")
};

// get the value of the field
let value = field.value()?;

// downcast into a `u8`
let value: u8 = value.try_into()?;

// check that it's equal to `42`!
assert_eq!(value, 42);
```
See the `examples` directory of this crate's source for additional examples.

## Limitations
The current implementation of [`default_provider`] only works when DWARF
debuginfo is stored in the program's binary. It will not work if DWARF
debug info is split into other files. Pull requests are welcome.

This crate is highly experimental. It is not suitable as a critical
component of any system. The initial releases of this crate require
significant polish. Pull requests are welcome. Its known soundness holes
include ignorance of `UnsafeCell`. Don't reflect into types containing
`UnsafeCell`.

Additionally, the particulars of how Rust encodes DWARF debug info my change
over time. This crate will do its best to keep up with those changes. Again,
pull requests are welcome.

## License

This project is licensed under the Apache License, Version 2.0, or the MIT
license, at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in deflect by you, shall be licensed as MIT and Apache 2.0,
without any additional terms or conditions.
