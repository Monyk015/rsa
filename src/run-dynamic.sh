#!/bin/bash
gcc static.c -L../target/debug -lrsa -o a.out
# export LD_LIBRARY_PATH=/mnt/d/Rust/rsa/target/debug:$LD_LIBRARY_PATH