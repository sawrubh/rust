// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-win32 leaks
// Issue #787
// Don't try to clean up uninitialized locals

extern mod std;

fn test_break() { loop { let x: @int = break; } }

fn test_cont() { let mut i = 0; while i < 1 { i += 1; let x: @int = loop; } }

fn test_ret() { let x: @int = return; }

fn test_fail() {
    fn f() { let x: @int = fail!(); }
    task::try(|| f() );
}

fn test_fail_indirect() {
    fn f() -> ! { fail!(); }
    fn g() { let x: @int = f(); }
    task::try(|| g() );
}

pub fn main() {
    test_break();
    test_cont();
    test_ret();
    test_fail();
    test_fail_indirect();
}
