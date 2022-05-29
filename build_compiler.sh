#!/bin/sh

echo "Building Jaye's compiler"
cd jaye_compiler
cargo build
mv target/debug ../jaye_compiler_debug_build
cd ../
