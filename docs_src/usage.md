---
title: How to use
nav_order: 3
---

You will find in this section the instructions on how to use this tool on your computer. Following this guide, you should be able to use this tool at your will.

As a reminder, `doteur` is a CLI tool meaning that you need to run it through a shell or cmd session. There is no GUI available at the moment.

---

## Print help

```
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

---

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
