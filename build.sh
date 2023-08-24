export PATH=$PATH:/usr/local/osx-ndk-x86/bin

./make_color.sh cargo +stable zigbuild --release --target aarch64-apple-darwin
./make_color.sh cargo +stable zigbuild --release --target x86_64-apple-darwin
./make_color.sh cargo +stable build --release --target x86_64-unknown-linux-gnu
./make_color.sh cargo +stable build --release --target x86_64-pc-windows-gnu

mv target/aarch64-apple-darwin/release/lcode ./release/lcode-aarch64-apple-darwin
mv target/x86_64-apple-darwin/release/lcode ./release/lcode-x86_64-apple-darwin
mv target/x86_64-unknown-linux-gnu/release/lcode ./release/lcode-x86_64-unknown-linux-gnu
mv target/x86_64-pc-windows-gnu/release/lcode.exe ./release/lcode-x86_64-pc-windows-gnu.exe

zip -j ./release/code-aarch64-apple-darwin.zip ./release/code-aarch64-apple-darwin
zip -j ./release/code-x86_64-apple-darwin.zip ./release/code-x86_64-apple-darwin
zip -j ./release/code-x86_64-unknown-linux-gnu.zip ./release/code-x86_64-unknown-linux-gnu
zip -j ./release/code-x86_64-pc-windows-gnu.zip ./release/code-x86_64-pc-windows-gnu.exe
