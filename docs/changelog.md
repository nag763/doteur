---
title: Changelog
permalink: changelog
---

You will find below the different details about the different versions of this tool.


---

## Version 0.5.7 (latest) :

- Fix multiple line comments issue ((https://github.com/nag763/doteur/issues/6)[https://github.com/nag763/doteur/issues/6])

- Dependencies and rust version upgrade

---

## Version 0.5.6 :

- Optimize release package size

- Support for schema name prefix

---

## Version 0.5.5 :

- Upgrade deps

- Optimize binary sizes

---

## Version 0.5.4 :

- This release upgrades the dependencies of the crate.
- It also upgrades the rust version.
- Temporary file for rendering is now created in temporary dir
- Partial postgres support

---

## Version 0.5.3 :

- Upgrade dependencies

---

## Version 0.5.2 :

- Fix dependalerts

---

## Version 0.5.1 :

- Minor fixes

---

## Version 0.5.0 :

- A brand new website
- A better packaging 
- A better code, for a better tool !

---

## Version 0.4.1 :

- Adds a connection to a running database with the usage of `--it` and `--url` args
- Better end of program handling with calls to `exit()` rather than `panic!`

---

## Version 0.4.0 :

- Detect primary keys
- Draw primary keys on table renderer
- Rendering different on the relations depending on the `ON DELETE` types
- Enhanced logging

---

## Version 0.3.3 :

- Tool is now available on crates.io
- You can now download the tool with

```
cargo install doteur
```

---

## Version 0.3.2 :

- Fix double attribute rendering when FK
- PK detecting and rendering delayed to v0.4.0 at best

---

## Version 0.3.1 :

- Fix issue with unicode chars on Windows

---

## Version 0.3.0 :

- Multifile support
- Support folder
- Draw keys on table renderer
- Adds a dark mode
- Change styles for light mode

---

## Version 0.2.0 :
 
- Stabilise the new arguments support
- Change background color of tables, change header color
- More models, better code quality (can still be improved)
- Add docker support




