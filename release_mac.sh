apple=aarch64-apple-darwin

cargo +stable build --release --target $apple
[[ -d ./release ]] || mkdir ./release
cp target/$apple/release/lcode ./release/lcode-$apple
zip -j ./release/lcode-$apple.zip ./release/lcode-$apple
