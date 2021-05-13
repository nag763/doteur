#!bin/sh

rustup toolchain install stable-x86_64-pc-windows-gnu

rm -rf target/*
cargo build --release #Linux by default
cargo build --release --target x86_64-pc-windows-gnu #Windows x64-86

rm -rf release
mkdir release

zip release/windows_x86.zip target/x86_64-pc-windows-gnu/release/ -R
zip release/linux_x86.zip target/release/ -R

md5sum release/windows_x86.zip > release/windows_x86.zip.md5
md5sum release/linux_x86.zip > release/linux_x86.zip.md5

