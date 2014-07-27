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

# Pulls test cases out of code fences in .md/.rs files

import os
import sys

def undoc(filename, outdir):

    if not os.path.isfile(filename):
        print "'%s' is not a file" % filename
        return False

    if not os.path.isdir(outdir):
        print "'%s' is not a dir" % outdir
        return False

    return True

if __name__ == '__main__':

    if len(sys.argv) < 3:
        print "usage: undoc.py [filename] [outdir]"
        sys.exit(1)

    filename = sys.argv[1]
    outdir = sys.argv[2]

    if undoc(filename, outdir):
        sys.exit(0)
    else:
        sys.exit(1)
    
