// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(dead_code)]

#[derive(PartialEq, Eq)]
struct Foo {
    x: u32
}

const FOO: Foo = Foo { x: 0 };

fn main() {
    let y = Foo { x: 1 };
    match y {
        FOO => { }
        _ => { }
    }
}
