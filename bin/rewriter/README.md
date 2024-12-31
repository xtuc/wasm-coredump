# wasm-coredump-rewriter

`wasm-coredump-rewriter` is used to add Coredump generation to a compiled Wasm
module.

## Installation

```bash
cargo install wasm-coredump-rewriter
```

## Usage

```
Rewrite Wasm binaries to add coredump support

Usage: wasm-coredump-rewriter [OPTIONS]

Options:
      --check-memory-operations    Wraps each memory operation. This will likely reduce significantly your program's performance
      --debug                      Enable debugging, mostly useful for developing this tooling
      --instance-id <INSTANCE_ID>  Specify the instance index to use in stack frames. MUST match the order in which instances are instantiated at runtime [default: 0]
  -h, --help                       Print help
  -V, --version                    Print version
```

### Step 1: rewrite the Wasm module

Rewrite the source Wasm module to inject the Coredump runtime code. The runtime
will catch traps (excluding traps in host functions or memory violations) and
generate a coredump.

Use the following command:
```bash
wasm-coredump-rewriter < source.wasm > output.wasm
```

The Coredump runtime has a performance cost.

### Step 2: Wasm execution trapped

Your program entered a trap and a Coredump was generated.

To extract the Coredump write the Wasm instance memory to a file, for instance in JavaScript:
```js
const instance = await WebAssembly.instantiate(...);

try {
    wasi.start(instance);
} catch(err) {
    const image = new Uint8Array(instance.exports.memory.buffer);
    writeFile("coredump." + Date.now(), image);
}
```

### Step 3: analyzing / debugging the coredump

See [wasmgdb] for analyzing / debugging the coredump.

[wasmgdb]: https://github.com/xtuc/wasm-coredump/tree/main/bin/wasmgdb
