// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-rustfix
// compile-flags: --edition 2018

#![allow(unused)]
#![deny(elided_lifetimes_in_paths)]
//~^ NOTE lint level defined here

use std::cell::{RefCell, Ref};


struct Foo<'a> { x: &'a u32 }

fn foo(x: &Foo) {
    //~^ ERROR implicit lifetime parameters in types are deprecated
    //~| HELP indicate the anonymous lifetime
}

fn bar(x: &Foo<'_>) {}


struct Wrapped<'a>(&'a str);

fn wrap_gift(gift: &str) -> Wrapped {
    //~^ ERROR implicit lifetime parameters in types are deprecated
    //~| HELP indicate the anonymous lifetime
    Wrapped(gift)
}


fn main() {
    let honesty = RefCell::new((4, 'e'));
    let loyalty: Ref<(u32, char)> = honesty.borrow();
    //~^ ERROR implicit lifetime parameters in types are deprecated
    //~| HELP indicate the anonymous lifetime
}
