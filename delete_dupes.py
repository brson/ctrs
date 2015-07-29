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

import subprocess

cmd = ["sh", "-c", "git ls-tree -r HEAD | cut -c 13- | sort | uniq -D -w 40"];
foo = subprocess.Popen(cmd, stdout=subprocess.PIPE)
(stdout, stderr) = foo.communicate()

current_sha = ""

for line in stdout.split('\n'):
    line = line.strip()
    if not line: continue
    (sha, path) = line.split('\t')
    if current_sha != sha:
        current_sha = sha
        continue
        
    ret = subprocess.call(["git", "rm", path])
    assert ret == 0
