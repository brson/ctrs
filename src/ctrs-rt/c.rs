// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::path::PathBuf;
use semver::Version;

pub fn build() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let tests: Vec<TestLevel> = find_tests();
}

struct TestLevel {
    name: Version,
    groups: Vec<TestGroup>,
}

struct TestGroup {
    name: String,
    tests: Vec<Test>
}

enum Test {
    // Tests that work basically like those for the RI's compiletest
    CompileTest {
        path: PathBuf,
        opts: Vec<CompileTestOpts>
    }
}

enum CompileTestOpts {
}

fn find_tests() -> Vec<TestLevel> {
    unimplemented!()
}
