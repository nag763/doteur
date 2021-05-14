#!bin/sh

rustup toolchain install stable-x86_64-pc-windows-gnu

rm -rf target/*
cargo build --release #Linux by default
cargo build --release --target x86_64-pc-windows-gnu #Windows x64-86

rm -rf release
mkdir release

zip release/doteur_windows_86_64.zip target/x86_64-pc-windows-gnu/release/* -r
zip release/doteur_linux_86_64.zip target/release/* -r

md5sum release/doteur_windows_86_64.zip > release/doteur_windows_86_64.zip.md5
md5sum release/doteur_linux_86_64.zip > release/doteur_linux_86_64.zip.md5

