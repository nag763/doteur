#!bin/sh

rustup toolchain install stable-x86_64-pc-windows-gnu

rm -rf target/*
cargo build --release #Linux by default
cargo build --release --target x86_64-pc-windows-gnu #Windows x64-86

rm -rf release
mkdir release

cd target/x86_64-pc-windows-gnu/
mv release doteur
zip ../../release/doteur_windows_86_64.zip ./doteur -r

cd ../..

cd target
mv release doteur
zip ../release/doteur_linux_86_64.zip ./doteur -r

cd ../release

md5sum doteur_windows_86_64.zip > doteur_windows_86_64.zip.md5
md5sum doteur_linux_86_64.zip > doteur_linux_86_64.zip.md5

cd ..

echo "Done with success"
