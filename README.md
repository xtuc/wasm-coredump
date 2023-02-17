# WebAssembly Coredump 

> Monorepo for generating, parsing, debugging WebAssembly coredumps.

See [demo] for an overview about how to use Wasm coredumps.

## Debugging

> Think gdb for WebAssembly

See [wasmgdb] for documentation.

## Use Coredump today

Since no Wasm engine support exists today, you an use [wasm-coredump-rewriter] to inject Coredump
support.

## Coredump format

See [specificiation].

## Troubleshooting

### Running into stack overflow

Some Wasm binaries have very recursive flow of control, increase the maximum stack size:
```
$ ulimit -s 160000
```
[wasmgdb]: bin/wasmgdb/README.md
[demo]: bin/wasmgdb/demo.md
[specificiation]: https://github.com/WebAssembly/tool-conventions/blob/main/Coredump.md
[wasm-coredump-rewriter]: bin/rewriter/README.md
