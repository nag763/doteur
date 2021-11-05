#!bin/sh


if [ -x "$(command -v doteur)" ];
then
	echo "It looks like doteur is already installed in your path"
	echo "Exiting ..."
	exit
fi


if [ -x "$(command -v cargo)" ];
then
    cargo install --path .
elif [[ -f "./doteur" ]];
then
    if [ "$EUID" -ne 0 ];
    	then 
		echo "Cargo isn't in your path, either install cargo or run this as sudo"
		echo "Exiting ..."
		exit
    else
    	   cp ./doteur /usr/bin/doteur
    fi
else
	echo "Couldn't install doteur, please ensure you didn't modify the zip or download it again"
	echo "Exiting ..."
	exit
fi

echo "Doteur installed correctly ! Enjoy !"
exit
