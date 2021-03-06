// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-fast
// aux-build:crateresolve5-1.rs
// aux-build:crateresolve5-2.rs

extern mod cr5_1 (name = "crateresolve5", vers = "0.1");
extern mod cr5_2 (name = "crateresolve5", vers = "0.2");


fn main() {
    // Nominal types from two multiple versions of a crate are different types
    assert!(cr5_1::nominal() == cr5_2::nominal()); //~ ERROR mismatched types: expected
}
