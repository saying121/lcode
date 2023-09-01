export PATH=$PATH:/usr/local/osx-ndk-x86/bin

apple=aarch64-apple-darwin
apple86=x86_64-apple-darwin
linux=x86_64-unknown-linux-gnu
win=x86_64-pc-windows-gnu

./make_color.sh cargo +stable build --release --target $apple
./make_color.sh cargo +stable build --release --target $apple86
./make_color.sh cargo +stable build --release --target $linux
./make_color.sh cargo +stable build --release --target $win

[[ -d ./release ]] || mkdir ./release

cp target/$apple/release/lcode ./release/lcode-$apple
cp target/$apple86/release/lcode ./release/lcode-$apple86
cp target/$linux/release/lcode ./release/lcode-$linux
cp target/$win/release/lcode.exe ./release/lcode-$win.exe

zip -j ./release/lcode-$apple.zip ./release/lcode-$apple
zip -j ./release/lcode-$apple86.zip ./release/lcode-$apple86
zip -j ./release/lcode-$linux.zip ./release/lcode-$linux
zip -j ./release/lcode-$win.zip ./release/lcode-$win.exe
