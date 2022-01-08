#!bin/sh

numberOfFeatures=4
featuresToBuild=("" '--features mysql_addons' '--features sqlite_addons' '--all-features')
packagesNames=('doteur_light' 'doteur_mysql' 'doteur_sqlite' 'doteur')
target_names=('x86_64-unknown-linux-gnu' 'x86_64-pc-windows-gnu' 'x86_64-apple-darwin')

if ! command -v rustup &> /dev/null
then
    echo "rustup could not be found, please ensure it is installed"
    echo "Exiting..."
    exit
fi

rm -rf release
mkdir release

for i in "${target_names[@]}"
do
	echo ""
	echo "Building for target : $i"
	echo "-----------------------"
	for ((j=0;j<=numberOfFeatures;j++)); do
		echo "Running cargo build --release ${featuresToBuild[j]} --target $i";
		cargo build --release ${featuresToBuild[j]} --target $i;
		echo 'Cargo build done';
		echo "Starting to zip target/$i/release in ./release/${packagesNames[j]}_$i";
		zip ./release/${packagesNames[j]}_$i -r target/$i/release -r -qq;
		echo "Zipping done";
		md5sum ./release/${packagesNames[j]}_$i.zip > ./release/${packagesNames[j]}_$i.md5
	done
done

cargo doc;
zip ./release/doteur_doc ./target/doc -r --qq;

echo "Done with success"
