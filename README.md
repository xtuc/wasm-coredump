# WebAssembly Coredump 

> Monorepo for generating, parsing, debugging WebAssembly coredumps.


## Debugging

See [demo] for an overview about how to use Wasm coredumps.

```
cargo install wasmgdb

wasmgdb /path/to/coredump /path/to/source.wasm
```

See [wasmgdb] for complete documentation.

## Rewrite

```bash
cargo install wasm-coredump-rewriter

wasm-coredump-rewriter < input.wasm > output.wasm
```

## Coredump format

See [specificiation].

## Troubleshooting

### Running into stack overflow

Some Wasm binaries have very recursive flow of control, increase the maximum stack size:
```
$ ulimit -s 160000
```
[wasmgdb]: wasmgdb/README.md
[demo]: wasmgdb/demo.md
[specificiation]: https://github.com/WebAssembly/tool-conventions/blob/19f5576d4344c9fcdb3855d5793908d051f393f0/Coredump.md
