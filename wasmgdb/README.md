# wasmgdb

> gdb for WebAssembly

## Install

```
cargo install wasmgdb
```

## Usage

Use [wasm-edit] to transform your module and, once WebAssembly traps, collect the
WebAssembly memory and analyze the coredump.

## Analyze a coredump

```
wasmgdb <coredump> <source.wasm>
```

### Commands

#### `bt`

Display the stack trace.

#### `f <#>`

Selects a stack frame and display informations.

#### `p <expr>`

Inspect the content of a variable.
Requires to select the frame with `f` first.

#### `p *<expr>`

Inspect the content of a variable after dereferencing it.
Requires to select the frame with `f` first.

#### `p/s <expr>`

Print the variable as string.
Requires to select the frame with `f` first.

#### `p (<type>) <expr>`

Print the content of <expr> as <type>.

#### `x/<number> <hex-addr>`

Examine the memory address at <hex-addr> with <number> length.

#### `x/<number>s <hex-addr>`

Examine the memory address at <hex-addr> and prints as string of <number> length.

#### `find <expr>`

Find the <expr> as bytes in memory.

Usage:
`find <start-addr>, <end-addr>, <expr>`
`find <expr>`

#### `info types`

List all defined types.

#### `info locals`

List local values (includes Wasm func arguments).

#### `info symbol <funcidx>`

Get informations about the function at the given index.

#### `info imports`

List WebAssembly module's imported functions from the host.

### Expr

- Member access: `<object>-><member>`
- Cast: `(<type>) <hex-addr>`
- String: `"<string>"`

[wasm-edit]: https://github.com/xtuc/wasm-edit#coredump-generation
