# Conformance Tests for Rust

This is a test suite for validating that a
compiler implements the Rust programming language.
The basic strategy will be to
establish a criteria for identifying 'stable' Rust code, and
collecting as much of it as possible here.

Execute with `./run.py`. Set the `RUSTC` environment variable if you must.

# Criteria for inclusion

Criteria will expand over time.
Currently:

* Must pass 'rustc -F experimental -F deprecated'
* No 'extern crate' (i.e. only std)
* No enabling feature gates

Run `./stab.py` on a Rust file to see if it is a candidate.

# Licensing

Any OSI-approved license accepted.
Rust license prefered.
Non-free licenses also welcome, but in another repo.

# Organization

All tests are under the `test` directory.
Immediately under `test` are directories for Rust versions.
Under each version are directories for test 'groups'.
Test groups are drawn from many sources.

# Current coverage

- 0.11.0
  - run-pass - Tests from the reference test suite's 'run-pass' directory
  - pretty - Tests from the reference test suite's 'pretty' directory

# Future directions

* Improve accuracy of staby.py
* Improve upstream API stability
* Add support for reference compile-fail tests
* Support aux-build tests
* Add support for Makefile tests
* Sources
  * Extract from all in-tree docs
  * rustbyexample.com
  * rustforrubyists.com
  * Other 'example' sources
  * Dependency-free libs e.g. math, quickcheck

# Compiler interface requirements

Running the test suite requires a Rust compiler that generally behaves
like the reference compiler.
Currently the test suite expects the compiler to support the `-o` flag.
In the future it will expect more flags to work.
