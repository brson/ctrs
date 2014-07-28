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
from undoc import undoc

def slurp_doc(srcdir, destdir):

    junkdirs, basename = os.path.split(srcdir)
    if not basename:
        junkdirs, basename = os.path.split(junkdirs)
    assert basename

    test_root = basename

    for dir_name, subdir_list, file_list in os.walk(srcdir):
        for f in file_list:
            if not f.endswith(".md") and not f.endswith(".rs"):
                continue
            test_name = dir_name.replace(junkdirs, "")
            if not test_name.endswith("/"):
                test_name = test_name + "/"
            test_name = test_name + f
            test_name = test_name.replace("/", "_")
            test_name = test_name.replace(".", "_")
            test_name = test_name.translate(None, "\:")
            test_name = test_name.strip("_")
            undoc(dir_name + "/" + f, destdir, test_name)

if __name__ == '__main__':

    if len(sys.argv) < 3:
        print "usage: slurp_doc.py srcdir destdir"
        sys.exit(1)

    srcdir = sys.argv[1]
    destdir = sys.argv[2]

    slurp_doc(srcdir, destdir)
