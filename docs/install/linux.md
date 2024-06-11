---
layout: page
parent: Installation guide
title: On Linux
permalink: install/linux
---

**Since the version 0.5.0, doteur is shipped with several packages, for more information check the [features](../features) section**

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

**Since the version 0.5.0, doteur is shipped with several packages, some features require additional dependencies being installed on your system, please check this [section](../features#additional-known-requirements-on-linux-regarding-the-features) if you face issues**

## First step : download the tool

See the download list [here](../downloads)

And unzip it :

```
unzip $ZIP_FOLDER_NAME
```

## Second : Add it to your path

### By linking the binary into your /usr/local/bin

```
sudo ln -s /${unzipped_folder}/doteur /usr/local/bin/doteur
```

### By moving it into your /usr/local/bin


```
sudo mv /${unzipped_folder}/doteur /usr/local/bin/doteur
```


You should now be done ! Check with your terminal that the tool is in your path :

```
doteur --help
```

It should print the help guide of the tool.

Now that you are done, check the [Usage section](../usage).
