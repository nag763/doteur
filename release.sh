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

rm -rf release
mkdir release

for i in "${target_names[@]}"
do
	echo ""
	echo "Building for target : $i"
	echo "-----------------------"
	for ((j=0;j<=numberOfFeatures;j++)); do
		echo "";
		echo "** Time for $i with ${featuresToBuild[j]} **"
		echo "Running cargo build --release ${featuresToBuild[j]} --target $i";
		cargo build --release ${featuresToBuild[j]} --target $i -q ;
		echo 'Cargo build done';
		cd "target/$i" ;
		echo "Currently in ";
		pwd;
		echo 'Using upx';
		if [ $i = "x86_64-pc-windows-gnu" ]; then
			upx release/doteur.exe;
		else
			upx release/doteur;
		fi
		echo 'Rebranding folder';
		mv release doteur ;
		echo 'Folder rebranded into doteur';
		echo "Starting to zip $i/release in ../../release/${packagesNames[j]}_$i";
		zip ../../release/${packagesNames[j]}_$i -r doteur -r -qq;
		echo "Zipping done";
		cd ../..;
		echo "Back to ";
		pwd
		echo "Adding sum";
		md5sum ./release/${packagesNames[j]}_$i.zip > ./release/${packagesNames[j]}_$i.zip.md5
		echo "Cleaning release dir";
		rm -r target/$i/doteur;
		echo "Done with ${featuresToBuild[j]} for $i";
	done
done

cargo doc -q --all-features;
zip ./release/doteur_doc ./target/doc -r --q;
md5sum release/doteur_doc.zip > release/doteur_doc.zip.md5 ;

echo "Done with success"
