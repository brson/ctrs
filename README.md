[![Build Status](https://travis-ci.org/brson/ctrs.svg?branch=master)](https://travis-ci.org/brson/ctrs)

# Conformance Tests for Rust

This is a test suite for validating that a compiler implements the Rust programming language.
The basic strategy will be to establish a criteria for identifying 'stable' Rust code,
and collecting as much of it as possible here.

Execute with `./run.py` to test your Rust compiler.
Set the `RUSTC` environment variable if you must.

## Criteria for inclusion

Criteria will expand over time.
Currently:

* Must pass 'rustc -F unstable -F deprecated -F unstable_features'
* No 'extern crate' (i.e. only std)

Run `./stab.py` on a Rust file to see if it is a candidate.

## Organization

All tests are under the `test` directory. Immediately under `test`
are directories for Rust versions. Under each version are directories
for test 'groups'. Test groups are drawn from many sources.

## Current coverage

Current focus is pulling tests out of the reference implementation that
meet the criteria.

- 1.0.0-rc
  - doc - Extracts from the RI 'doc' directory
  - doc-collections - Extracts from the RI collections API docs
  - doc-core - Extracts from the RI core library API docs
  - doc-std - Extracts from the RI std library API docs
  - pretty - Tests from the RI test suite's 'pretty' directory
  - run-pass - Tests from the RI test suite's 'run-pass' directory
  - rust-by-example - Extracts from www.rustbyexample.com

## Compiler interface requirements

Running the test suite requires a Rust compiler that generally behaves
like the reference compiler. Currently the test suite expects the
compiler to support the `-o` flag. In the future it will expect more
flags to work.

## Licensing

Any OSI-approved license accepted. Rust license prefered. Non-free
licenses also welcome, but in another repo.

## Scripts

This test suite is a bunch of python scripts for wrangling Rust code from various places and into a form that is runnable.

* `dupes.py` - Lists files in the repo with identical hashes.
* `run.py` - The test suite runner. Just needs a `rustc` command available or the `RUSTC` environment variable to be set.
* `slurp.py` - Copies all Rust files that pass `stab.py` from one dir to another.
* `slurp_doc.py` - Recursively extracts doc tests from all .rs/.md files to a dir.
* `stab.py` - Checks whether a crate meets the criteria for inclusion.
* `undoc.py` - Extracts doc tests from a single file.

## Future directions

* Improve accuracy of staby.py
* Improve upstream API stability
* Add support for reference compile-fail tests
* Support aux-build tests
* Support should_fail doc tests
* Add support for Makefile tests
* FFI tests, like rust_test_helpers
* Sources
  * compile-fail, run-fail, run-make tests
  * In-tree std crate docs
  * Popular blog posts
  * rustforrubyists.com
  * Other 'example' sources
  * Dependency-free libs e.g. math, quickcheck
  * coretest
  * automatic extraction of #[test] cases
  * tool to extract build plans from cargo graphs

