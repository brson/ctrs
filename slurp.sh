#!/bin/bash

SRC="$1"
DST="$2"

find "$SRC" -name "*.rs" -exec ./stab.py "{}" \; 2> /dev/null|grep "\[stable\]"|sed 's/\[stable\] //'|xargs cp -t "$DST"
