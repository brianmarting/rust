// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait Trait<'a> {}

fn main() {
    let _: &for<'a> Trait<'a> + 'static;
    //~^ ERROR expected a path on the left-hand side of `+`, not `& for<'a>Trait<'a>`
    //~| HELP try adding parentheses
    //~| SUGGESTION &( for<'a>Trait<'a> + 'static)
}
