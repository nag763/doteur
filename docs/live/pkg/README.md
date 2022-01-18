Dev in progress

## How to build 

```
wasm-pack build --target web --no-typescript
```

## How to serve

```
# In example dir
python3 -m http.server
```

## Build and add to GH page

```
wasm-pack build --target web --no-typescript --out-dir example/pkg --release
cp example ../docs_src/live -r
```
