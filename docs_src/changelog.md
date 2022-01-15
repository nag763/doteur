---
title: Changelog
permalink: changelog
---

You will find below the different details about the different versions of this tool.

---

## Version 0.5.0 (in development)

---

## Version 0.4.1 (latest) :

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




