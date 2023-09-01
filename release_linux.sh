#!/bin/bash

linux=x86_64-unknown-linux-gnu
cargo +stable build --release --target $linux
[[ -d ./release ]] || mkdir ./release
cp target/$linux/release/lcode ./release/lcode-$linux
zip -j ./release/lcode-$linux.zip ./release/lcode-$linux
