---
title: How to use
nav_order: 3
permalink: usage
---

**Since the version 0.5.0, doteur is shipped in several packages, for more information check the [features](features) section. Soome CLI options might not be avalaible regarding which version you installed on your system**

You will find in this section the instructions on how to use this tool on your computer. Following this guide, you should be able to use this tool at your will.

As a reminder, `doteur` is a CLI tool meaning that you need to run it through a shell or cmd session. There is no GUI available at the moment.

---

## Print help

```
doteur 0.5.1
LABEYE Loïc <loic.labeye@pm.me>
Parse a SQL configuration and convert it into a .dot file, render the output if Graphviz is
installed

USAGE:
    doteur [OPTIONS] [INPUT]...

ARGS:
    <INPUT>...    Name of the sql file or database location if an URL arg is passed, can also be
                  a directory or several files

OPTIONS:
        --dark_mode            Wheter to render in dark mode or not
    -h, --help                 Print help information
    -i, --include <INCLUDE>    Filter to include only the given tables, accept simple regexs
        --legend               Includes hint about the relations type at the bottom of the output
                               file
    -o, --output <OUTPUT>      Name of the output file [default: output.dot]
    -V, --version              Print version information
    -x, --exclude <EXCLUDE>    Filter to exclude the given tables, accept simple regexs

Some functionnalities might not appear as they depend on which version this tool has been downloaded
or built for.

```
---

## Example usage

### Export a .sql file to a .dot

```bash
usr@pop-os:~$ doteur sample.sql
```

The output will be in the output.dot

### Export a .sql file to a .png

```bash
usr@pop-os:~$ doteur sample.sql -o output.png
```

For the format supported, please refer to the graphviz [documentation](https://graphviz.org/doc/info/output.html)

### [mysql_addons] Connect to a remote database without a dialog and export the file as .png

```bash
usr@ubuntu:~/doteur$ doteur mysql://newuser:password@localhost:3306/foo --url -o output.png
```

### [mysql_addons] Connect to a remote database with dialog and export the file as .png

```bash
usr@ubuntu:~/doteur$ doteur --it -o output.png
Database url or ip: localhost
Database port: 3306
Database name: foo
Database user: username
Database password: [hidden]
```

The output will be in a png file.

### [sqlite_addons] Connect to a sqlite3 database without a dialog and export the file as .png

```bash
doteur db.sqlite3 --sqlite -o output.png
```
### Export a .sql file to a .png, render in dark mode

```bash
usr@pop-os:~$ doteur sample.sql -o output.png --dark-mode
```

### Export a .sql file to a .png, include only tables who have either the name hello or world

```bash
usr@pop-os:~$ doteur sample.sql -o output.png -i hello world
```

### Export a .sql file to a .png, include only tables that start with the name hello

```bash
usr@pop-os:~$ doteur sample.sql -o output.png -i hello*
```

### Export a .sql file to a .png, exclude all tables that start with the name hello

```bash
usr@pop-os:~$ doteur sample.sql -o output.png -x hello*
```

### See logs of a output

```bash
usr@pop-os:~$ RUST_LOG=ERROR doteur sample.sql
```

With available levels being `DEBUG`, `INFO`, `WARN`, `ERROR`

