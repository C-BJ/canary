#!/bin/sh

echo "Building Jaye"
cd jaye_compiler
cargo build
cp -r target/debug ../jaye_compiler_debug_build
cd ../
