# Wasm coredump

TODO: docs.

## Rewrite

```
wasm-coredump-rewriter < input.wasm > output.wasm
```

## Debug

[wasmgdb]

## Troubleshooting

### Running into stack overflow

Some Wasm binaries have very recursive flow of control, increase the maximum stack size:
```
$ ulimit -s 160000
```
[wasmgdb]: wasmgdb/README.md
