[![crates-dl](https://img.shields.io/crates/v/doteur)](https://crates.io/crates/doteur)
[![doteur-stars](https://img.shields.io/github/stars/nag763/doteur?style=social)](https://github.com/nag763/doteur/stargazers)
[![doteur-license](https://img.shields.io/crates/l/doteur)](https://github.com/nag763/doteur/blob/main/LICENCE.MD)
[![github-issues](https://img.shields.io/github/issues/nag763/doteur)](https://github.com/nag763/doteur/issues)
[![docker-build](https://img.shields.io/docker/cloud/build/nag763/doteur)](https://hub.docker.com/r/nag763/doteur)

<p align="center"><img src="https://raw.githubusercontent.com/nag763/doteur/main/.github/assets/logo.png"></img></p>

<h2 align="center">Doteur</h2>
<h4 align="center">A simple tool to draw your mysql relations from exports.</h4>

<p align="center"><img height ="480" width="640" src="https://raw.githubusercontent.com/nag763/doteur/main/.github/assets/sample.jpeg"></img></p>


#### Help us

If you use the tool, and like it, don't forget to add a star to the project on GitHub :star:, it helps us to keep motivation to improve it.

If you have any suggestion, or issue, please report it, it will help us to fix them.

#### General information

- <u>Author :</u> LABEYE Loïc
- <u>Licence :</u> MIT
- <u>Official website :</u> https://doteur.net
- <u>Platforms available :</u>
	- Linux
		- [X] Tested
	- Windows
		- [X] Tested
- <u>Github :</u> https://github.com/nag763/doteur
- <u>crates.io :</u> https://crates.io/crates/doteur
- <u>Docker image, continuous delivery :</u> https://hub.docker.com/r/nag763/doteur  

#### Known system requirements

##### For Linux

libssl-dev is the required if you installed a version including mysql features (either doteur_mysql or doteur).

libsqlite3-dev is the required if you installed a version including sqlite features (either doteur_sqlite or doteur).


Also, it is required to have gcc installed with at least the version 9.

###### On Debian based systems

```bash
sudo apt-get install gcc
libssl-dev #for mysql features
libsqlite3-dev #for sqlite features
```

###### On Fedora

```bash
sudo rpm -i gcc 
libssl-dev #for mysql features
libsqlite3-dev #for sqlite features
```
###### On arch based systems

```bash
sudo pacman -S gcc
libssl-dev #for mysql features
libsqlite3-dev #for sqlite features
```

#### How to use through Docker

To download the tool

```bash
docker pull nag763/doteur:latest
```

Then to use it, simply pass a folder with your sql files, and be careful to write all your outputs in the shared folder to ensure the files are available on your host machine once the image is destroyed.

```bash
docker run --rm -v ${PATH_TO_YOUR_SQL_FOLDER}:/usr/src/doteur/shared -it nag763/doteur:latest bash
```

#### How to install it natively

0. **Optional but highly recommended**

First install graphviz.

On Linux, use your default package manager.

Example on ubuntu :

```
usr@ubuntu:~$ sudo apt-get install graphviz
```

On Windows, use winget or the default graphviz installer.

You can still refer to the [graphviz download page](https://graphviz.org/download/).

*The tool uses graphviz libraries to render in other formats than .dot, if graphviz is not installed or you do not want to install it, you can still use this [tool](https://dreampuf.github.io/GraphvizOnline/) and pass it your output file*.

1. Download the tool

- Via cargo

```
cargo install doteur
```

- Via the [release page](https://github.com/nag763/doteur/releases/latest)

- Via the [official website](https://doteur.net)

- Via github

```bash
cargo install --git https://github.com/nag763/doteur
```



2. Use the tool

```bash
doteur --help
```

3. Add it to your path

If you need to use it regularly, it is recommended to add the bin folder to your path. For this, look on how to do it on your distro.

On linux :

```bash
ln -s path/to/doteur /usr/sbin
```

#### Cli usage

```bash
doteur 0.4.1
LABEYE Loïc <loic.labeye@pm.me>
Convert .sql files to .dot files, render them if graphviz installed

USAGE:
    doteur [FLAGS] [OPTIONS] [--] [input]...

FLAGS:
        --dark-mode    Render in dark mode
    -h, --help         Prints help information
        --it           Starts an interactive dialog to connect to a remote database
        --legend       Includes hint about the relations type at the bottom of the outpout file
        --sqlite       Specificate that the input is a sqlite3 database
        --url          Specificate that the input is an URL (i.e. mysql://usr:password@localhost:3306/database)
    -V, --version      Prints version information

OPTIONS:
    -x, --exclude <exclude>...    Exclude the given tables
    -i, --include <include>...    Include only the given tables
    -o, --output <output>         The output filename

ARGS:
    <input>...    Name of the sql file or database location if url arg is passed, can also be a directory or several files
```

#### Example usage

##### Export a .sql file to a .dot

```bash
usr@pop-os:~$ doteur sample.sql
```

The output will be in the output.dot

##### Export a .sql file to a .png

```bash
usr@pop-os:~$ doteur sample.sql -o output.png
```

For the format supported, please refer to the graphviz [documentation](https://graphviz.org/doc/info/output.html)

##### Connect to a remote database without a dialog and export the file as .png

```bash
usr@ubuntu:~/doteur$ doteur mysql://newuser:password@localhost:3306/foo --url -o output.png
```

##### Connect to a remote database with dialog and export the file as .png

```bash
usr@ubuntu:~/doteur$ doteur --it -o output.png
Database url or ip: localhost
Database port: 3306
Database name: foo
Database user: username
Database password: [hidden]
```

The output will be in a png file.

##### Connect to a sqlite3 database without a dialog and export the file as .png

```bash
doteur db.sqlite3 --sqlite -o output.png
```
##### Export a .sql file to a .png, render in dark mode

```bash
usr@pop-os:~$ doteur sample.sql -o output.png --dark-mode
```

##### Export a .sql file to a .png, include only tables who have either the name hello or world

```bash
usr@pop-os:~$ doteur sample.sql -o output.png -i hello world
```

##### Export a .sql file to a .png, include only tables that start with the name hello

```bash
usr@pop-os:~$ doteur sample.sql -o output.png -i hello*
```

##### Export a .sql file to a .png, exclude all tables that start with the name hello

```bash
usr@pop-os:~$ doteur sample.sql -o output.png -x hello*
```

##### See logs of a output

```bash
usr@pop-os:~$ RUST_LOG=ERROR doteur sample.sql
```

With available levels being `DEBUG`, `INFO`, `WARN`, `ERROR`
