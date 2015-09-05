// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use env_logger;
use std::env;
use std::error::Error as StdError;
use std::io;
use std::fmt;
use std::fs::{self, DirEntry};
use std::path::{PathBuf, Path};
use semver::{self, Version};

pub fn build() {
    env_logger::init().unwrap();

    let current_dir = env::current_dir().unwrap();
    let test_dir = current_dir.join("test");
    let out_dir = env::var("OUT_DIR").unwrap();
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let tests: Vec<TestLevel> = find_levels(&test_dir).ok().unwrap();

    info!("{:?}", tests);
    unimplemented!()
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct TestLevel {
    name: Version,
    groups: Vec<TestGroup>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct TestGroup {
    name: String,
    tests: Vec<Test>
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
enum Test {
    // Tests that work basically like those for the RI's compiletest
    CompileTest {
        path: PathBuf,
        opts: Vec<CompileTestOpts>
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
enum CompileTestOpts {
}

fn find_levels(test_dir: &Path) -> Result<Vec<TestLevel>, Error> {
    let levels = try!(fs::read_dir(test_dir)).filter_map(|e| {
        dir_to_semver(e).ok()
    });

    let levels: Vec<TestLevel> = levels.filter_map(|level| {
        let groups: Option<Vec<TestGroup>> = find_groups(test_dir, &level).ok();
        groups.map(|groups| TestLevel { name: level, groups: groups })
    }).collect();

    Ok(levels)
}

fn find_groups(test_dir: &Path, level: &Version) -> Result<Vec<TestGroup>, Error> {
    let level_dir = test_dir.join(&level.to_string());
    let groups = try!(fs::read_dir(level_dir)).filter_map(|e| {
        dir_to_string(e).ok()
    });

    let groups: Vec<TestGroup> = groups.filter_map(|group| {
        let tests: Option<Vec<Test>> = find_tests(test_dir, level, &group).ok();
        tests.map(|tests| TestGroup { name: group, tests: tests })
    }).collect();

    Ok(groups)
}

fn find_tests(test_dir: &Path, level: &Version, group: &str) -> Result<Vec<Test>, Error> {
    let group_dir = test_dir.join(&level.to_string()).join(group);
    let tests = try!(fs::read_dir(group_dir)).filter_map(|e| {
        dir_to_string(e).ok()
    });

    let tests: Vec<Test> = tests.filter_map(|test| {
        read_test(test_dir, level, group, &test).ok()
    }).collect();

    Ok(tests)
}

fn read_test(test_dir: &Path, level: &Version, group: &str, test: &str) -> Result<Test, Error> {
    let path = test_dir.join(&level.to_string()).join(group).join(test);
    let test = Test::CompileTest {
        path: path,
        opts: Vec::new()
    };

    Ok(test)
}

fn dir_to_string(entry: io::Result<DirEntry>) -> Result<String, Error> {
    let dir = try!(entry).path();
    let dir = try!(dir.file_name().ok_or_else(|| format!("wut?")));
    let dir = try!(dir.to_str().ok_or_else(|| format!("non-utf8 path: {:?}", dir)));

    Ok(dir.to_string())
}
    
fn dir_to_semver(entry: io::Result<DirEntry>) -> Result<Version, Error> {
    let dir = try!(dir_to_string(entry));
    
    Ok(try!(Version::parse(&dir)))
}

#[derive(Debug)]
enum Error {
    IoError(io::Error),
    CustomError(String),
    SemverError(semver::ParseError),
}

macro_rules! convert_error {
    ($from:ty, $to:ident) => (
        impl From<$from> for Error {
            fn from(e: $from) -> Error {
                Error::$to(e)
            }
        }
    )
}

convert_error!(io::Error, IoError);
convert_error!(String, CustomError);
convert_error!(semver::ParseError, SemverError);

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::IoError(ref e) => e.fmt(fmt),
            Error::CustomError(ref s) => s.fmt(fmt),
            Error::SemverError(ref e) => e.fmt(fmt),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref e) => e.description(),
            Error::CustomError(ref s) => s,
            Error::SemverError(ref e) => e.description(),
        }
    }
}
