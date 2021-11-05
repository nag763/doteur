#!bin/sh


if ! [ -x "$(command -v doteur)" ];
then
	echo "It looks like doteur is not installed in your path"
	echo "Exiting ..."
	exit
fi


if [[ -x "$(command -v cargo)" ]] && cargo install --list | grep -q doteur ;
then
	cargo uninstall doteur
    	echo "Doteur uninstalled successfully from your path"
    	echo "Exiting..."
    	exit
else
    if [ "$EUID" -ne 0 ];
    	then 
		echo "Doteur isn't in your cargo path, please run again this tool as sudo to ensure it is neither in your usr path"
		echo "Exiting ..."
		exit
    else
    	   rm /usr/bin/doteur
	   echo "Doteur uninstalled successfully from your path"
    	   echo "Exiting..."
    	   exit
    fi
fi

echo "We couldn't uninstall doteur, please ensure it is installed or remove it manually"
echo "Doteur path :"
which doteur
exit
