#!/bin/sh

wasm-pack build --target web --no-typescript --out-dir example/pkg --release
cp example ../docs_src/live -r
