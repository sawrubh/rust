// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern: unresolved name

use module_of_many_things::*;

mod module_of_many_things {
    pub fn f1() { debug!("f1"); }
    pub fn f2() { debug!("f2"); }
    fn f3() { debug!("f3"); }
    pub fn f4() { debug!("f4"); }
}


fn main() {
    f1();
    f2();
    f999(); // 'export' currently doesn't work?
    f4();
}
