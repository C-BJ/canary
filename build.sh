#!/bin/sh

echo "Building Jaye"
cd jaye_compiler
cargo build
cp ./target/debug ../jaye_compiler_debug_build
cd ../
