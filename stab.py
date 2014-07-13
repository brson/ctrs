#!/usr/bin/python

import sys

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

    return True

# FIXME This is a very naive check. rustc should be doing it
def is_stable(filename):
    if not passes_smell_test(filename): return False
    return True

if len(sys.argv) < 2:
    sys.exit(1)

filename = sys.argv[1]

if is_stable(filename):
    print "[stable] " + filename
    sys.exit(0)
else:
    print "[unstable] " + filename
    sys.exit(1)

