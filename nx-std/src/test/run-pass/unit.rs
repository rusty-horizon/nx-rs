// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused_assignments)]
#![allow(unknown_lints)]
// pretty-expanded FIXME #23616

#![allow(unused_variables)]
#![allow(dead_assignment)]

fn f(u: ()) { return u; }

pub fn main() {
    let u1: () = ();
    let mut u2: () = f(u1);
    u2 = ();
    return ();
}