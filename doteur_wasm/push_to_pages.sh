#!/bin/sh

wasm-pack build --target web --no-typescript --out-dir example/pkg --release
cp example/* ../../doteur-pages/docs_src/live -r
