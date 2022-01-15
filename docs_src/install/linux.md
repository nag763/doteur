---
layout: page
parent: Installation guide
title: On Linux
permalink: install/linux
---

## Prerequisites

It is required for you to have gcc at least in the version 9, and highly recommended to install graphviz in order to render :

### On Debian/ Ubuntu

```
sudo apt-get install gcc graphviz
```

### On Fedora

```
sudo rpm -i gcc graphviz
```

### On ArchLinux

```
sudo pacman -S gcc graphviz
```

## First step : download the tool

See the download list [here](../downloads)

And unzip it :

```
unzip $ZIP_FOLDER_NAME
```

## Second step : Move the tool in your library

```
sudo mv $UNZIPPED_FOLDER/doteur /usr/local/lib
```

### Finally create a symlink so that the tool remains in your path

```
sudo ln -s /usr/local/lib/doteur/doteur /usr/local/bin/doteur
```

You should now be done ! Check with your terminal that the tool is in your path :

```
doteur --help
```

It should print the help guide of the tool.

Now that you are done, check the [Usage section](../usage).