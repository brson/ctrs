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
import subprocess
import shutil

# The command to invoke the Rust compiler being tested
rustc = os.getenv('RUSTC', 'rustc')
# The location of the tests, by default in ./test
test_dir = os.getenv('CTRS_TEST_DIR', 'test')
# The directory to use for temporary files, default ./tmp-ctrs
tmp_dir = os.getenv('CTRS_TMP_DIR', 'tmp-ctrs')

# Sanity checks

print "# Testing rustc version"
print

retcode = subprocess.call([rustc, "--version"])
if retcode != 0:
    print "rustc isn't working. Maybe set RUSTC env var."
    sys.exit(1)
else:
    print

# Remove old temporary files

if os.path.isdir(tmp_dir):
    shutil.rmtree(tmp_dir)
retcode = subprocess.call(["mkdir", "-p", tmp_dir])
if retcode != 0:
    print "unable to create tmp dir `" + tmp_dir + "`"
    sys.exit(1)

def run_test(version, group, test_name):
    work_dir = tmp_dir + "/" + version + "/" + group + "/" + test_name
    retcode = subprocess.call(["mkdir", "-p", work_dir])
    if retcode != 0:
        print "failed to create tmp dir " + work_dir
        sys.exit(1)

    src_path = test_dir + "/" + version + "/" + group + "/" + test_name

    exe_path = work_dir + "/" + "out.exe"

    # First compile
    retcode = subprocess.call([rustc, src_path, "-o", exe_path])
    if retcode != 0:
        print src_path + ": fail"
        return False

    # Now run
    retcode = subprocess.call([exe_path])
    if retcode != 0:
        print src_path + ": fail"
        return False

    print src_path + ": pass"
    return True


def run_test_group(version, group):
    group_dir = test_dir + "/" + version + "/" + group
    passes = 0
    total = 0
    for test_name in os.listdir(group_dir):
        test_passed = run_test(version, group, test_name)
        if test_passed:
            passes += 1
        total += 1
    return (passes, total)

passes = 0
total = 0

# Each directory under the test directory contains tests for a specific
# version of the language
for version in os.listdir(test_dir):
    version_dir = test_dir + "/" + version
    if not os.path.isdir(version_dir): continue

    print "# Version " + version
    print

    for group in os.listdir(version_dir):
        group_dir = version_dir + "/" + group

        print "## " + version + "/" + group
        print

        (new_passes, new_total) = run_test_group(version, group)

        passes += new_passes
        total += new_total

        print

print "# Summary"
print
print str(passes) + "/" + str(total)

if total == passes:
    sys.exit(0)
else:
    sys.exit(1)
