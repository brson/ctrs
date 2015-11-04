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
# Space-separated list of test versions
test_versions_filter = os.getenv('CTRS_VERSIONS', None)
# Space-separated list of test groups to run
test_groups_filter = os.getenv('CTRS_GROUPS', None)
# Space-separated list of test names
test_names_filter = os.getenv('CTRS_NAMES', None)

if test_versions_filter: test_versions_filter = test_versions_filter.split(" ")
if test_groups_filter: test_groups_filter = test_groups_filter.split(" ")
if test_names_filter: test_names_filter = test_names_filter.split(" ")

verbose = False

for arg in sys.argv:
    if arg == "--verbose":
        verbose = True

# Sanity checks

print "Testing Rust compiler version:"
print

retcode = subprocess.call([rustc, "--version"])
if retcode != 0:
    print "Rust compiler isn't working. Maybe set RUSTC env var."
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

def get_config(filename):
    test_only = False

    with open(filename) as f:
        for line in f:
            if "// ctrs-test-only" in line:
                test_only = True

    return {
        'test_only': test_only
    }

def run_test(version, group, test_name):
    work_dir = tmp_dir + "/" + version + "/" + group + "/" + test_name
    retcode = subprocess.call(["mkdir", "-p", work_dir])
    if retcode != 0:
        print "failed to create tmp dir " + work_dir
        sys.exit(1)

    src_path = test_dir + "/" + version + "/" + group + "/" + test_name

    config = get_config(src_path)

    exe_path = work_dir + "/" + "out.exe"

    # First compile
    if verbose: print "building " + src_path
    rustc_args = [rustc, src_path, "--crate-type=bin", "-o", exe_path]
    if config['test_only']:
        rustc_args += ["--test"]
    p = subprocess.Popen(rustc_args,
                         stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    output, errors = p.communicate()
    retcode = p.wait()

    if verbose or retcode != 0:
        print
        print "=== stdout ===\n"
        print output
        print "=== stderr ===\n"
        print errors

    if retcode != 0:
        print src_path + ": fail"
        return False

    # Now run
    if verbose: print "running " + src_path
    p = subprocess.Popen([exe_path],
                         stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    output, errors = p.communicate()
    retcode = p.wait()

    if verbose or retcode != 0:
        print
        print "=== stdout ===\n"
        print output
        print "=== stderr ===\n"
        print errors

    if retcode != 0:
        print src_path + ": fail"
        return False

    return True


def run_test_group(version, group):
    return run_basic_test_group(version, group)

def is_broken(path):
    with open(path, "r") as f:
        lines = f.readlines()
        for line in lines:
            if "// ctrs-broken" in line:
                return True
    return False

def run_basic_test_group(version, group):
    group_dir = test_dir + "/" + version + "/" + group

    test_names = []
    for test_name in os.listdir(group_dir):
        if is_broken(group_dir + "/" + test_name):
            print "skipping broken test: " + group + " / " + test_name
            continue
        keep = True
        if test_names_filter:
            keep = False
            for filter in test_names_filter:
                if filter in test_name:
                    keep = True
        if keep:
            test_names += [test_name]

    passes = 0
    total = 0
    for test_name in test_names:
        if not test_name.endswith(".rs"): continue

        test_passed = run_test(version, group, test_name)
        if test_passed:
            passes += 1
        total += 1
        if total != 1:
            # Overwriting progress in place
            sys.stdout.write("\r")
        sys.stdout.write(version + "/" + group + ": " + str(passes) + "/" + str(total - passes) + "/" + str(len(test_names)))
        sys.stdout.flush()

    passfail = " [pass]"
    if passes != total:
        passfail = " [fail]"
    print "\r" + version + "/" + group + ": " + str(passes) + "/" + str(total - passes) + "/" + str(len(test_names)) + passfail
    return (passes, total)

total = 0

for version in os.listdir(test_dir):
    version_dir = test_dir + "/" + version
    if not os.path.isdir(version_dir): continue
    if test_versions_filter and not version in test_versions_filter: continue

    for group in os.listdir(version_dir):
        group_dir = version_dir + "/" + group

        if test_groups_filter and not group in test_groups_filter: continue

        for test_name in os.listdir(group_dir):
            if is_broken(group_dir + "/" + test_name):
                print "skipping broken test: " + group + " / " + test_name
                continue
            keep = True
            if test_names_filter:
                for filter in test_names_filter:
                    if filter in test_name:
                        keep = True
            if keep:
                total += 1

print "Running " + str(total) + " tests"
print

total = 0
passes = 0

# Each directory under the test directory contains tests for a specific
# version of the language
versions = []
for version in os.listdir(test_dir):
    version_dir = test_dir + "/" + version
    if not os.path.isdir(version_dir): continue
    if test_versions_filter and not version in test_versions_filter: continue
    versions += [version]

versions = sorted(versions)

for version in versions:

    final_groups = []

    version_dir = test_dir + "/" + version
    for group in os.listdir(version_dir):
        group_dir = version_dir + "/" + group

        if test_groups_filter and not group in test_groups_filter: continue
        final_groups += [group]

    final_groups = sorted(final_groups)

    for group in final_groups:
        (new_passes, new_total) = run_test_group(version, group)
        passes += new_passes
        total += new_total



print
print "# Summary"
print
print str(passes) + "/" + str(total - passes) + "/" + str(total)

if total == passes:
    sys.exit(0)
else:
    sys.exit(1)
