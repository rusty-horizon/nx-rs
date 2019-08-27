// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:structs.rs
extern crate structs;

use structs::{NormalStruct, UnitStruct, TupleStruct, FunctionalRecord};

fn main() {
    let fr = FunctionalRecord {
    //~^ ERROR cannot create non-exhaustive struct
        first_field: 1920,
        second_field: 1080,
        ..FunctionalRecord::default()
    };

    let ns = NormalStruct { first_field: 640, second_field: 480 };
    //~^ ERROR cannot create non-exhaustive struct

    let NormalStruct { first_field, second_field } = ns;
    //~^ ERROR `..` required with struct marked as non-exhaustive

    let ts = TupleStruct(640, 480);
    //~^ ERROR expected function, found struct `TupleStruct` [E0423]

    let ts_explicit = structs::TupleStruct(640, 480);
    //~^ ERROR tuple struct `TupleStruct` is private [E0603]

    let TupleStruct { 0: first_field, 1: second_field } = ts;
    //~^ ERROR `..` required with struct marked as non-exhaustive

    let us = UnitStruct;
    //~^ ERROR expected value, found struct `UnitStruct` [E0423]

    let us_explicit = structs::UnitStruct;
    //~^ ERROR unit struct `UnitStruct` is private [E0603]

    let UnitStruct { } = us;
    //~^ ERROR `..` required with struct marked as non-exhaustive
}
