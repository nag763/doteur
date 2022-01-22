
[![crates-dl](https://img.shields.io/crates/v/doteur)](https://crates.io/crates/doteur)
[![doteur-stars](https://img.shields.io/github/stars/nag763/doteur?style=social)](https://github.com/nag763/doteur/stargazers)
[![doteur-license](https://img.shields.io/crates/l/doteur)](https://github.com/nag763/doteur/blob/main/LICENCE.MD)
[![github-issues](https://img.shields.io/github/issues/nag763/doteur)](https://github.com/nag763/doteur/issues)
[![docker-build](https://img.shields.io/docker/cloud/build/nag763/doteur)](https://hub.docker.com/r/nag763/doteur)

<p align="center"><img src="https://raw.githubusercontent.com/nag763/doteur/main/.github/assets/logo.png"></img></p>

<h2 align="center">Doteur</h2>
<h4 align="center">A simple tool to render graphically your SQL schemas.</h4>

<p align="center"><img height ="480" width="640" src="https://raw.githubusercontent.com/nag763/doteur/main/.github/assets/sample.jpeg"></img></p>

## About

Doteur is a CLI (Command Line Interface) tool that has for purpose to render the SQL schemas into good looking graphs. This will help you to easily understand the structure of a large database and understand what happens behind the scenes of your project.

Besides, you will be able to use the large panel of features to either sort the tables you want to visualize or render with a different color scheme for instance.

So far the tool handles both the MySQL and SQLite syntaxes, but it is planned to handle the Postgre one as soon as the formers will be considered as stable. The input of the tool can be either a sql file export, or given the version you downloaded, connect to either a MySQL running instance or an existing SQLite database.

The tool has been developed on Linux, but is also available for Windows 10 and 11 and macOS.

## 🆕 Live test

Since the version 0.5.1, the tool is testable with limited functionnalities on the GitHub pages. **Please note that the output of the live functionnality is experimental and won't always reflect the CLI's output.**

You can test the tool [here](https://nag763.github.io/doteur/live/)

## 🆕 About features

Since the version 0.5.0, the CLI tool is downloadable with several features. If you don't intend to connect to connect to either a running mysql or sqlite database, ignore what follows, the exports in SQL files will be handled no matter what feature you download the tool for.

### 🆕 What's a fetaure ?

A feature is an addition to the original tool that adds several components and functionnalities to the original ones.

### 🆕 How will it impact me

If you download the tool with a certain feature, you might not be able to use the other functionnalities. By default, no additionnal functionnalities are packaged, and you might not be able to either connect to a running mysql instance or a sqlite database.

### 🆕 What are the available features

* `mysql_addons` : Allow you to connect to running MySQL instance
* `sqlite_addons` : Allow you to connect to a local SQLite database

## Help us

If you use the tool, and like it, don't forget to add a star to the project on GitHub ⭐, it helps us to keep motivation to improve it.

If you have any suggestion, or issue, please report it, it will help us to fix them.

## General information

- <u>Author :</u> LABEYE Loïc
- <u>Licence :</u> MIT
- <u>Official website :</u> https://doteur.net
- <u>Platforms available :</u>
	- Linux
		- [X] Tested
	- Windows
		- [ ] Untested
- <u>Github :</u> https://github.com/nag763/doteur
- <u>Github pages :</u> https://nag763.github.io/doteur/
- <u>crates.io :</u> https://crates.io/crates/doteur
- <u>Docker image, continuous delivery :</u> https://hub.docker.com/r/nag763/doteur  

## How to install

🆕 [Check the github page dedicated to this section](https://nag763.github.io/doteur/install)

## How to use

🆕 [Check the github page dedicated to this section](https://nag763.github.io/doteur/usage)
