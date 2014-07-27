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
from stab import is_stable
import shutil

def slurp(srcdir, destdir):

    for src_file in os.listdir(srcdir):
        if not src_file.endswith("rs"):
            continue
        src_path = srcdir + "/" + src_file
        if is_stable(src_path):
            print "[stable] " + src_path
            shutil.copy(src_path, destdir)
        else:
            print "[unstable] " + src_path

if __name__ == '__main__':

    if len(sys.argv) < 3:
        print "usage: slurp.py [srcdir] [destdir]"
        sys.exit(1)

    srcdir = sys.argv[1]
    destdir = sys.argv[2]

    slurp(srcdir, destdir)
