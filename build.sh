#!bin/sh

if ! command -v rustup &> /dev/null
then
    echo "rustup could not be found, please ensure it is installed"
    echo "Exiting..."
    exit
fi

rm -rf target/*
cargo build --release #Linux by default
cargo build --release --target x86_64-pc-windows-gnu #Windows x64-86
cargo doc

cp install.sh target/release/
cp uninstall.sh target/release/

rm -rf release
mkdir release

cd target/x86_64-pc-windows-gnu/
mv release doteur
zip ../../release/doteur_windows_86_64.zip ./doteur -r

cd ../..

cd target
mv release doteur
zip ../release/doteur_linux_86_64.zip ./doteur -r

zip ../release/doteur_doc.zip ./doc -r

cd ../release

md5sum doteur_windows_86_64.zip > doteur_windows_86_64.zip.md5
md5sum doteur_linux_86_64.zip > doteur_linux_86_64.zip.md5
md5sum doteur_doc.zip > doteur_doc.zip.md5

cd ..

echo "Done with success"
