#!/bin/sh

echo "Building Jaye's compiler"
cd jaye_compiler
cargo build
cp -r target/debug/jaye_compiler ../jaye
cd ../
