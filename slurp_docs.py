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
import shutil
from undoc_dir import undoc_dir
from slurp import slurp

def slurp_docs(srcdir, destdir, tmpdir):

    shutil.rmtree(tmpdir)
    os.mkdir(tmpdir)

    undoc_dir(srcdir, tmpdir)
    slurp(tmpdir, destdir)

if __name__ == '__main__':

    if len(sys.argv) < 4:
        print "usage: slurp_docs.py srcdir destdir tmpdir"
        sys.exit(1)

    srcdir = sys.argv[1]
    destdir = sys.argv[2]
    tmpdir = sys.argv[3]

    slurp_docs(srcdir, destdir, tmpdir)
