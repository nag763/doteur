---
layout: page
parent: Installation guide
title: On Docker
permalink: install/docker
---

## Prerequisite

Ensure that you have docker installed on your computer. Check docker installation guide [here](https://docs.docker.com/get-docker/) if needed.

## First : Pull the image of the project

```bash
docker pull nag763/doteur:latest
```

## Finally, use the tool

Then to use it, simply pass a folder with your sql files, and be careful to write all your outputs in the shared folder to ensure the files are available on your host machine once the image is destroyed.

```
docker run --rm -v ${PATH_TO_YOUR_SQL_FOLDER}:/usr/src/doteur/shared -it nag763/doteur:latest bash
```

Now that you are done, check the [Usage section](../usage).
