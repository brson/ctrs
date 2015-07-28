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

def translate_test_case(out_file_name, markdown_tags, lines):

    is_definitely_rust = True

    useless_tags = [
        'ignore', 'console', 'text', 'notation',
        'gram', 'ebnf', 'sh', 'precedence',
        'field', 'tuple', 'literals',
        'linked-failure', 'c', 'md', 'keyword',
        'cpp', 'notrust', 'bash', 'ruby',
        'toml', 'shell', 'txt', 'should-fail',
        'js', 'javascript', 'html', 'make',
        'json', 'py'
    ]

    # TODO: Do stuff with these
    usefull_tags = [ 'no_run', 'test_harness', 'should_panic' ]

    # Configure translation from tags
    for tag in markdown_tags:
        if tag == 'rust':
            pass
        elif tag in useless_tags:
            is_definitely_rust = False
        elif tag in usefull_tags:
            is_definitely_rust = False
        else:
            # idk!
            print "NEW TAG DISCOVERED! [" + tag + "]"
            is_definitely_rust = False

    # Configure translation from source

    has_main = False

    for line in lines:
        if "fn main" in line:
            has_main = True

    if not is_definitely_rust:
        return

    if os.path.exists(out_file_name):
        print "%s exists!" % out_file_name
        return

    with open(out_file_name, 'w') as f:
        if not has_main:
            f.write("fn main() {\n")

        for line in lines:
            if line.startswith("# "):
                line = line[2:len(line)]
            if not has_main:
                # Indent to make pretty
                f.write("    ")
            f.write(line + "\n")

        if not has_main:
            f.write("}\n")
        

def undoc(filename, outdir, test_name):

    if not os.path.isfile(filename):
        print "'%s' is not a file" % filename
        return False

    if not os.path.isdir(outdir):
        print "'%s' is not a dir" % outdir
        return False

    test_num = 0

    with open(filename) as f:
        in_code_block = False
        markdown_tags = []
        lines = []
        for line in f:

            if line.find("```") >=0 or line.find("~~~") >= 0:
                line = line.strip()
            if line.strip().startswith("/// ") or line.strip().startswith("//! "):
                line = line.strip()[4 : len(line)]
            if line.strip().startswith("///") or line.strip().startswith("//!"):
                line = line.strip()[3 : len(line)]

            if line.startswith("```") or line.startswith("~~~"):
                in_code_block = not in_code_block

                if in_code_block:
                    assert not markdown_tags
                    assert not lines
                    tags = line
                    tags = tags.replace("`", "")
                    tags = tags.replace("~", "")
                    tags = tags.strip()
                    tags = tags.translate(None, "{}.")
                    if len(tags) > 0:
                        tags = tags.split(",")
                        if len(tags) == 1:
                            tags = tags[0].split(" ")
                    else:
                        tags = []
                    markdown_tags = tags
                else:
                    out_file_name = outdir + "/" + test_name + "_" + str(test_num).zfill(4) + ".rs"
                    translate_test_case(out_file_name, markdown_tags, lines)
                    markdown_tags = []
                    lines = []
                    test_num += 1

                continue

            if in_code_block:
                lines = lines + [line]

            pass

    return True

if __name__ == '__main__':

    if len(sys.argv) < 3:
        print "usage: undoc.py filename outdir"
        sys.exit(1)

    filename = sys.argv[1]
    outdir = sys.argv[2]

    test_name = os.path.basename(filename)
    test_name = test_name.replace(".", "_")

    if undoc(filename, outdir, test_name):
        sys.exit(0)
    else:
        sys.exit(1)
    
