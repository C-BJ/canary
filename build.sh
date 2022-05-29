#!/bin/sh

echo "Building Jaye"
cd jaye_compiler
cargo build
cp target/debug/jaye_compiler ../jaye_compiler_build
cd ../
