// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[doc(keyword = "as")]
//
/// The keyword for casting a value to a type.
///
/// `as` is most commonly used to turn primitive types into other primitive types, but it has other
/// uses that include turning pointers into addresses, addresses into pointers, and pointers into
/// other pointers.
///
/// ```rust
/// let thing1: u8 = 89.0 as u8;
/// assert_eq!('B' as u32, 66);
/// assert_eq!(thing1 as char, 'Y');
/// let thing2: f32 = thing1 as f32 + 10.5;
/// assert_eq!(true as u8 + thing2 as u8, 100);
/// ```
///
/// In general, any cast that can be performed via ascribing the type can also be done using `as`,
/// so instead of writing `let x: u32 = 123`, you can write `let x = 123 as u32` (Note: `let x: u32
/// = 123` would be best in that situation). The same is not true in the other direction, however,
/// explicitly using `as` allows a few more coercions that aren't allowed implicitly, such as
/// changing the type of a raw pointer or turning closures into raw pointers.
///
/// Other places `as` is used include as extra syntax for [`crate`] and `use`, to change the name
/// something is imported as.
///
/// For more information on what `as` is capable of, see the [Reference]
///
/// [Reference]:
/// https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
/// [`crate`]: keyword.crate.html
mod as_keyword { }

#[doc(keyword = "const")]
//
/// The keyword for defining constants.
///
/// Sometimes a certain value is used many times throughout a program, and it can become
/// inconvenient to copy it over and over. What's more, it's not always possible or desirable to
/// make it a variable that gets carried around to each function that needs it. In these cases, the
/// `const` keyword provides a convenient alternative to code duplication.
///
/// ```rust
/// const THING: u32 = 0xABAD1DEA;
///
/// let foo = 123 + THING;
/// ```
///
/// Constants must be explicitly typed, unlike with `let` you can't ignore its type and let the
/// compiler figure it out. Any constant value can be defined in a const, which in practice happens
/// to be most things that would be reasonable to have a constant (barring `const fn`s, coming
/// soon). For example, you can't have a File as a `const`.
///
/// The only lifetime allowed in a constant is `'static`, which is the lifetime that encompasses
/// all others in a Rust program. For example, if you wanted to define a constant string, it would
/// look like this:
///
/// ```rust
/// const WORDS: &str = "hello rust!";
/// ```
///
/// Thanks to static lifetime elision, you usually don't have to explicitly use 'static:
///
/// ```rust
/// const WORDS: &str = "hello convenience!";
/// ```
///
/// `const` items looks remarkably similar to `static` items, which introduces some confusion as
/// to which one should be used at which times. To put it simply, constants are inlined wherever
/// they're used, making using them identical to simply replacing the name of the const with its
/// value. Static variables on the other hand point to a single location in memory, which all
/// accesses share. This means that, unlike with constants, they can't have destructors, and act as
/// a single value across the entire codebase.
///
/// Constants, as with statics, should always be in SCREAMING_SNAKE_CASE.
///
/// The `const` keyword is also used in raw pointers in combination with `mut`, as seen in `*const
/// T` and `*mut T`. More about that can be read at the [pointer] primitive part of the Rust docs.
///
/// For more detail on `const`, see the [Rust Book] or the [Reference]
///
/// [pointer]: primitive.pointer.html
/// [Rust Book]:
/// https://doc.rust-lang.org/stable/book/2018-edition/ch03-01-variables-and-mutability.html#differences-between-variables-and-constants
/// [Reference]: https://doc.rust-lang.org/reference/items/constant-items.html
mod const_keyword { }

#[doc(keyword = "crate")]
//
/// The `crate` keyword.
///
/// The primary use of the `crate` keyword is as a part of `extern crate` declarations, which are
/// used to specify a dependency on a crate external to the one it's declared in. Crates are the
/// fundamental compilation unit of Rust code, and can be seen as libraries or projects. More can
/// be read about crates in the [Reference].
///
/// ```rust ignore
/// extern crate rand;
/// extern crate my_crate as thing;
/// extern crate std; // implicitly added to the root of every Rust project
/// ```
///
/// The `as` keyword can be used to change what the crate is referred to as in your project. If a
/// crate name includes a dash, it is implicitly imported with the dashes replaced by underscores.
///
/// `crate` is also used as in conjunction with `pub` to signify that the item it's attached to
/// is public only to other members of the same crate it's in.
///
/// ```rust
/// # #[allow(unused_imports)]
/// pub(crate) use std::io::Error as IoError;
/// pub(crate) enum CoolMarkerType { }
/// pub struct PublicThing {
///     pub(crate) semi_secret_thing: bool,
/// }
/// ```
///
/// [Reference]: https://doc.rust-lang.org/reference/items/extern-crates.html
mod crate_keyword { }

#[doc(keyword = "enum")]
//
/// For defining enumerations.
///
/// Enums in Rust are similar to those of other compiled languages like C, but have important
/// differences that make them considerably more powerful. What Rust calls enums are more commonly
/// known as [Algebraic Data Types] if you're coming from a functional programming background. The
/// important detail is that each enum variant can have data to go along with it.
///
/// ```rust
/// # struct Coord;
/// enum SimpleEnum {
///     FirstVariant,
///     SecondVariant,
///     ThirdVariant,
/// }
///
/// enum Location {
///     Unknown,
///     Anonymous,
///     Known(Coord),
/// }
///
/// enum ComplexEnum {
///     Nothing,
///     Something(u32),
///     LotsOfThings {
///         usual_struct_stuff: bool,
///         blah: String,
///     }
/// }
///
/// enum EmptyEnum { }
/// ```
///
/// The first enum shown is the usual kind of enum you'd find in a C-style language. The second
/// shows off a hypothetical example of something storing location data, with `Coord` being any
/// other type that's needed, for example a struct. The third example demonstrates the kind of
/// data a variant can store, ranging from nothing, to a tuple, to an anonymous struct.
///
/// Instantiating enum variants involves explicitly using the enum's name as its namespace,
/// followed by one of its variants. `SimpleEnum::SecondVariant` would be an example from above.
/// When data follows along with a variant, such as with rust's built-in [`Option`] type, the data
/// is added as the type describes, for example `Option::Some(123)`. The same follows with
/// struct-like variants, with things looking like `ComplexEnum::LotsOfThings { usual_struct_stuff:
/// true, blah: "hello!".to_string(), }`. Empty Enums are similar to () in that they cannot be
/// instantiated at all, and are used mainly to mess with the type system in interesting ways.
///
/// For more information, take a look at the [Rust Book] or the [Reference]
///
/// [Algebraic Data Types]: https://en.wikipedia.org/wiki/Algebraic_data_type
/// [`Option`]: option/enum.Option.html
/// [Rust Book]: https://doc.rust-lang.org/book/second-edition/ch06-01-defining-an-enum.html
/// [Reference]: https://doc.rust-lang.org/reference/items/enumerations.html
mod enum_keyword { }

#[doc(keyword = "extern")]
//
/// For external connections in Rust code.
///
/// The `extern` keyword is used in two places in Rust. One is in conjunction with the [`crate`]
/// keyword to make your Rust code aware of other Rust crates in your project, i.e., `extern crate
/// lazy_static;`. The other use is in foreign function interfaces (FFI).
///
/// `extern` is used in two different contexts within FFI. The first is in the form of external
/// blocks, for declaring function interfaces that Rust code can call foreign code by.
///
/// ```rust ignore
/// #[link(name = "my_c_library")]
/// extern "C" {
///     fn my_c_function(x: i32) -> bool;
/// }
/// ```
///
/// This code would attempt to link with `libmy_c_library.so` on unix-like systems and
/// `my_c_library.dll` on Windows at runtime, and panic if it can't find something to link to. Rust
/// code could then use `my_c_function` as if it were any other unsafe Rust function. Working with
/// non-Rust languages and FFI is inherently unsafe, so wrappers are usually built around C APIs.
///
/// The mirror use case of FFI is also done via the `extern` keyword:
///
/// ```rust
/// #[no_mangle]
/// pub extern fn callable_from_c(x: i32) -> bool {
///     x % 3 == 0
/// }
/// ```
///
/// If compiled as a dylib, the resulting .so could then be linked to from a C library, and the
/// function could be used as if it was from any other library.
///
/// For more information on FFI, check the [Rust book] or the [Reference].
///
/// [Rust book]:
/// https://doc.rust-lang.org/book/second-edition/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code
/// [Reference]: https://doc.rust-lang.org/reference/items/external-blocks.html
mod extern_keyword { }

#[doc(keyword = "fn")]
//
/// The `fn` keyword.
///
/// The `fn` keyword is used to declare a function.
///
/// Example:
///
/// ```rust
/// fn some_function() {
///     // code goes in here
/// }
/// ```
///
/// For more information about functions, take a look at the [Rust Book][book].
///
/// [book]: https://doc.rust-lang.org/book/second-edition/ch03-03-how-functions-work.html
mod fn_keyword { }

#[doc(keyword = "let")]
//
/// The `let` keyword.
///
/// The `let` keyword is used to declare a variable.
///
/// Example:
///
/// ```rust
/// # #![allow(unused_assignments)]
/// let x = 3; // We create a variable named `x` with the value `3`.
/// ```
///
/// By default, all variables are **not** mutable. If you want a mutable variable,
/// you'll have to use the `mut` keyword.
///
/// Example:
///
/// ```rust
/// # #![allow(unused_assignments)]
/// let mut x = 3; // We create a mutable variable named `x` with the value `3`.
///
/// x += 4; // `x` is now equal to `7`.
/// ```
///
/// For more information about the `let` keyword, take a look at the [Rust Book][book].
///
/// [book]: https://doc.rust-lang.org/book/second-edition/ch03-01-variables-and-mutability.html
mod let_keyword { }

#[doc(keyword = "struct")]
//
/// The `struct` keyword.
///
/// The `struct` keyword is used to define a struct type.
///
/// Example:
///
/// ```
/// struct Foo {
///     field1: u32,
///     field2: String,
/// }
/// ```
///
/// There are different kinds of structs. For more information, take a look at the
/// [Rust Book][book].
///
/// [book]: https://doc.rust-lang.org/book/second-edition/ch05-01-defining-structs.html
mod struct_keyword { }
