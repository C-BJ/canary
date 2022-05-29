#!/bin/sh

echo "Building Jaye"
cd jaye
cargo build
cp target/debug/jaye ../jaye_build
cd ../
