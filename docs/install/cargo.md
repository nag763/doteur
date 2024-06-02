---
layout: page
parent: Installation guide
title: (Recommended) With Cargo
permalink: install/cargo
---

The easiest and best way to ensure that you download the tool correctly and that it will work for your platform is to download the tool with `cargo`.

`cargo` is a CLI tool used for Rust programs in order to download and install packages on a system. It is similar to `pip` for Python, it will manage your project and help you to ship it.

Check [this guide](https://doc.rust-lang.org/cargo/getting-started/installation.html) on how to install cargo.

Since the release 0.3.3, doteur is downloadable with cargo as it has been published on crates.io. It is then recommended to install the tool this way :

## Step 1 : Download the tool

**Since the version 0.5.0, doteur is shipped with several packages, for more information check the [features](../features) section**
- `light` :

```
cargo install doteur
```

- `mysql_addons` :

```
cargo install doteur --features mysql_addons
```

- `sqlite_addons` :

```
cargo install doteur --features sqlite_addons
```

- `all-features` :

```
cargo install doteur --all-features
```

**Since the version 0.5.0, doteur is shipped with several packages, some features require additional dependencies being installed on your system, please check this [section](../features#additional-known-requirements-on-linux-regarding-the-features) if you face issues**

## Step 2 : Ensure it has been installed correctly

Once the tool has been downloaded with cargo, the executable will be in your path, to verify :

```
doteur --help
```

Which should print the doteur help.

Now that you are done, check the [Usage section](../usage).
