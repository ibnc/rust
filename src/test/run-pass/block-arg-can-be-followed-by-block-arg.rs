// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub fn main() {
    fn f(i: &fn() -> uint) -> uint { i() }
    let v = ~[-1.0, 0.0, 1.0, 2.0, 3.0];
    let z = do do v.iter().fold(f) |x, _y| { x } { 22u };
    assert_eq!(z, 22u);
}
