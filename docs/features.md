---
title: About features
nav_order: 2
permalink: features
---

Since 0.5.0, doteur is shipped in different packages. Read this section if you feel like you are lost on which feature to download for your computer.

## What are features ?

Features are enhancements to the original experience of the tool usage. They are meant to provide additional functionnalities and fit more specific usages.

## Why should I care about features ?

As you can see, this tool already covers a large panel of functionnalities and is starting to become bigger and bigger. It can be annoying as most users won't ever be using most of what is available while having a larger executable than it should be.

The purpose is then to download the necessary features instead of everything that is available. However, you can still download the whole package if you feel like you will be using all the features of the tool.


## What are the available features 

- `default` : No additionnal features to the original experience, it means that you will only be able to parse SQL exports.
- `mysql_addons` : This feature will allow you to connect to a remote MySQL database instance besides of the `light` features
- `sqlite_addons` : This feature will allow you to connect to a local SQLite database instace besides of the `light` features
- `all-features` : This feature contains all the previous features

## How can I chose which feature to download ?

### Downloading with cargo

Check this section regarding the downloads with cargo and the [features](install/cargo#step-1--download-the-tool).

### Docker usage

With docker, you download by default the version with all the features

### Linux and Windows binary downloads

Simply download the binary that matches the feature name

## Additional known requirements on Linux regarding the features

### mysql_addons

#### On Debian/ Ubuntu

```
sudo apt-get libssl-dev
```

#### On Fedora

```
sudo rpm -i libssl-dev
```

#### On ArchLinux

```
sudo pacman -S libssl-dev
```

### sqlite_addons

#### On Debian/ Ubuntu

```
sudo apt-get libsqlite3-dev
```

#### On Fedora

```
sudo rpm -i libsqlite3-dev
```

#### On ArchLinux

```
sudo pacman -S libsqlite3-dev
```

### all-features

Match the requirements for `mysql_addons` and `sqlite_addons`
