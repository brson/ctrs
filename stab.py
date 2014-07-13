#!/usr/bin/python

import os
import sys
from subprocess import call

rustc = os.getenv('RUSTC', 'rustc')

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


# Sanity checks

retcode = call([rustc, "--version"])
if retcode != 0:
    print "rustc isn't working. Maybe set RUSTC env var."
    sys.exit(1)


if is_stable(filename):
    print "[stable] " + filename
    sys.exit(0)
else:
    print "[unstable] " + filename
    sys.exit(1)

