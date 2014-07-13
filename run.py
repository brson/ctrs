#!/usr/bin/python

import os
import sys
import subprocess

rustc = os.getenv('RUSTC', 'rustc')
test_dir = 'test'

# Sanity checks

print "# Using rustc version:"
print

retcode = subprocess.call([rustc, "--version"])
if retcode != 0:
    print "rustc isn't working. Maybe set RUSTC env var."
    sys.exit(1)
else:
    print

def run_test_group(group_dir):
    passes = 0
    total = 0
    for test_name in os.listdir(group_dir):
        test_path = group_dir + "/" + test_name
        # FIXME: Need to actually run tests
        proc = subprocess.Popen([rustc, test_path, "--no-trans"], 0, None, None,
                                subprocess.PIPE, subprocess.PIPE)
        retcode = proc.wait()
        if retcode == 0:
            print test_path + ": pass"
            passes += 1
        else:
            print test_path + ": fail"
        total += 1

    return (passes, total)

passes = 0
total = 0

# Each directory under the test directory contains tests for a specific
# version of the language
for version in os.listdir(test_dir):
    version_dir = test_dir + "/" + version
    if not os.path.isdir(version_dir): continue

    print "# Running tests for " + version
    print

    for group in os.listdir(version_dir):
        group_dir = version_dir + "/" + group

        print "## Running " + version + "/" + group
        print

        (new_passes, new_total) = run_test_group(group_dir)

        passes += new_passes
        total += new_total

print
print # Summary
print
print str(passes) + "/" + str(total)

if total == passes:
    sys.exit(0)
else:
    sys.exit(1)
