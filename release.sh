#!bin/sh

numberOfFeatures=3
featuresToBuild=('--no-default-features' '--features mysql_addons' '--features sqlite_addons' '--all-features')
packagesNames=('doteur_light' 'doteur_mysql' 'doteur_sqlite' 'doteur')
target_names=('x86_64-unknown-linux-gnu' 'x86_64-pc-windows-gnu')

if ! command -v rustup &> /dev/null
then
    echo "rustup could not be found, please ensure it is installed"
    echo "Exiting..."
    exit
fi

target_args=""
for target in "${target_names[@]}"; do
	target_args+="--target ${target} "
done

rm -rf release
mkdir release

rm -rf target

echo ""
echo "Target args ${target_args}"
echo "-----------------------"
for ((j=0;j<=numberOfFeatures;j++)); do
	echo "";
	echo "** Time for $i with ${featuresToBuild[j]} **"
	echo "Running cargo build --bin doteur --release ${featuresToBuild[j]} ${target_args} -q ";
	cargo build --release ${featuresToBuild[j]} ${target_args} -q ;
	echo 'Cargo build done';
	for target in "${target_names[@]}"; do
		echo 'Using upx';
		if [ $target = "x86_64-pc-windows-gnu" ]; then
			upx target/$target/release/doteur.exe;
		else
			upx target/$target/release/doteur;
		fi
		echo "Moving to release dir"
		cd target/$target/
		echo "Updating dir name"
		mv release doteur
		echo "Include copy of licenese and README"
		cp ../../LICENCE.MD doteur
		cp ../../README.md doteur
		echo "Zipping in";
		zip ../../release/${packagesNames[j]}_$target doteur/doteur* doteur/LICENCE.md doteur/README.md -qq -r;
		echo "Back to former dir"
		cd ../..
		echo "Adding sum";
		md5sum ./release/${packagesNames[j]}_$target.zip > ./release/${packagesNames[j]}_$target.zip.md5
	done
	echo "Cleaning release dir";
	rm -r target/$target;
	echo "Done with ${featuresToBuild[j]}";
done

cargo doc -q --all-features;
zip ./release/doteur_doc ./target/doc -r --q;
md5sum release/doteur_doc.zip > release/doteur_doc.zip.md5 ;

echo "Done with success"
