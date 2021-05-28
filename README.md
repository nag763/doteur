y<p align="center"><img src="https://github.com/nag763/doteur/blob/main/logo.png"></img></p>

<h2 align="center">Doteur</h2>
<h4 align="center">A simple tool to draw your mysql relations from exports.</h4>


![sample](sample.jpeg)

#### Help us

If you use the tool, and like it, don't forget to add a star to the project on GitHub :star:, it helps us to keep motivation to improve it.

If you have any suggestion, or issue, please report it, it will help us to fix them.

#### General information

- <u>Author :</u> LABEYE Loïc
- <u>Licence :</u> MIT
- <u>Platforms available :</u>
	- Linux
		- [X] Tested
	- Windows
		- [X] Tested
		
- <u>Docker image, continuous delivery :</u> https://hub.docker.com/r/nag763/doteur  

#### How to use through Docker

To download the tool

```bash
docker pull nag763/doteur:latest
```

Then to use it, simply pass a folder with your sql files, and be careful to write all your outputs in the shared folder to ensure the files are available on your host machine once the image is destroyed.

```bash
docker run --rm -v ~/doteur/examples:/usr/src/doteur/shared -it nag763/doteur:latest bash
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

- Via the release page.

- Via github

```bash
git clone https://github.com/nag763/doteur
cd doteur
cargo install --path .
```

- Use the tool

```bash
doteur --help
```

3. Add it to your path

If you need to use it regularly, it is recommended to add the bin folder to your path. For this, look on how to do it on your distro.

#### Cli usage

```bash
usr@pop-os:~$ doteur --help
doteur 0.3.0
LABEYE Loïc
Convert .sql files to .dot files

USAGE:
    doteur [FLAGS] [OPTIONS] <input>...

FLAGS:
        --dark-mode    render in dark mode
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -x, --exclude <exclude>...    exclude the given tables
    -i, --include <include>...    include only the given tables
    -o, --output <output>         The output filename

ARGS:
    <input>...    Name of the sql file, can also be a directory or several files
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

The output will be in a png file.

For the format supported, please refer to the graphviz [documentation](https://graphviz.org/doc/info/output.html)

##### Export a .sql file to a .png, render in dark mode

```bash
usr@pop-os:~$ doteur sample.sql -o output.png -x hello* --dark-mode
```

##### Export a .sql file to a .png, include only tables who have either the name hello or world

```bash
usr@pop-os:~$ doteur sample.sql -o output.png -i hello world
```

##### Export a .sql file to a .png, include only tables who starts with the name hello

```bash
usr@pop-os:~$ doteur sample.sql -o output.png -i hello*
```

##### Export a .sql file to a .png, exclude all tables who starts with the name hello

```bash
usr@pop-os:~$ doteur sample.sql -o output.png -x hello*
```
