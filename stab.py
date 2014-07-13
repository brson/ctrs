#!/usr/bin/python
# Copyright 2014 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

import os
import sys
from subprocess import call

rustc = os.getenv('RUSTC', 'rustc')

# Sanity checks

retcode = call([rustc, "--version"])
if retcode != 0:
    print "rustc isn't working. Maybe set RUSTC env var."
    sys.exit(1)


# Some naive checks from just looking at the file,
# including examining compiletest test directives
def passes_smell_test(filename):

    with open(filename) as f:
        for line in f:
            # Look for things that might be risky

            # 'extern crate' might bring in experimental API's
            if "extern crate" in line: return False
            # No turning on feature gates
            if "feature" in line: return False
            # No ignored test cases
            if "// ignore" in line: return False
            # No test cases requiring aux
            if "aux-build" in line: return False
            # No compile-flags
            if "compile-flags" in line: return False

    return True

def uses_stable_apis(filename):
    retcode = call([rustc, filename, "--no-trans", "-F", "experimental", "-F", "deprecated"]);
    if retcode == 0: return True
    else: return False

def is_stable(filename):
    if not passes_smell_test(filename): return False
    if not uses_stable_apis(filename): return False
    return True


# Parse arguments

if len(sys.argv) < 2:
    print "usage: stab.py [filename]"
    sys.exit(1)

filename = sys.argv[1]

if is_stable(filename):
    print "[stable] " + filename
    sys.exit(0)
else:
    print "[unstable] " + filename
    sys.exit(1)

