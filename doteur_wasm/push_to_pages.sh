#!/bin/sh

wasm-pack build --target web --no-typescript --out-dir example/pkg --release
rm -rf ../docs_src/live
cp example ../docs_src/live -rf
