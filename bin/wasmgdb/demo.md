This document goes through running my broken code in production and using [wasmgdb] to analyze a crash, and hopefully fix it.

My Rust code:
```rust
struct MyThing {
    value: usize,
}

fn main() {
    let thing = MyThing { value: 3 };
    process_thing(&thing)
}

fn process_thing(thing: &MyThing) {
    let result = calculate(thing.value);
    println!("result is {}", result);
}

fn calculate(value: usize) -> usize {
    value - 10
}
```

Compiling to Wasm: `cargo build --target wasm32-wasi`.
Transforming the Wasm binary to support coredump (this step will hopefully be gone soon) using [wasm-coredump-rewriter]: `wasm-coredump-rewriter < ./test-program/target/wasm32-wasi/debug/test-program.wasm > ./test-program/target/wasm32-wasi/debug/test-program-final.wasm`.

Finally I can run my program using a Node & WASI environment:
```
$ node --experimental-wasi-unstable-preview1 runner.mjs

thread 'main' panicked at 'attempt to subtract with overflow', src/main.rs:16:5
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Oh no, what a suprise, the program crashed because of an exception in Rust. Annoyingly, we don't have more information (so far).

Note that Node isn't able to show us a stack trace due to the `wasm-coredump-rewriter` transformation.
A coredump has been generated and written to a file called `coredump.1669887752424`.

From the output we can see that the program crashed at line 16, which is here:
```rust
...
fn calculate(value: usize) -> usize {
    /* line 16: */ value - 10
}
```

Now let's analyze the crash in [wasmgdb] to see where it went wrong (if you didn't spot it from my broken code already).

```
$ wasmgdb ./test-program/target/wasm32-wasi/debug/test-program.wasm coredump.*
wasmgdb>
```
We enter a repl similar to gdb, commands are mostly the same.

Let's examine a backtrace:
```
wasmgdb> bt
#18     000137 as __rust_start_panic () at library/panic_abort/src/lib.rs
#17     000129 as rust_panic () at library/std/src/panicking.rs
#16     000128 as rust_panic_with_hook () at library/std/src/panicking.rs
#15     000117 as {closure#0} () at library/std/src/panicking.rs
#14     000116 as __rust_end_short_backtrace<std::panicking::begin_panic_handler::{closure_env#0}, !> () at library/std/src/sys_common/backtrace.rs
#13     000123 as begin_panic_handler () at library/std/src/panicking.rs
#12     000194 as panic_fmt () at library/core/src/panicking.rs
#11     000198 as panic () at library/core/src/panicking.rs
#10     000012 as calculate (value=0x03000000) at src/main.rs
#9      000011 as process_thing (thing=0x2cff0f00) at src/main.rs
#8      000010 as main () at src/main.rs
#7      000008 as call_once<fn(), ()> (???=0x01000000, ???=0x00000000) at /rustc/b833ad56f46a0bbe0e8729512812a161e7dae28a/library/core/src/ops/function.rs
#6      000020 as __rust_begin_short_backtrace<fn(), ()> (f=0x01000000) at /rustc/b833ad56f46a0bbe0e8729512812a161e7dae28a/library/std/src/sys_common/backtrace.rs
#5      000016 as {closure#0}<()> () at /rustc/b833ad56f46a0bbe0e8729512812a161e7dae28a/library/std/src/rt.rs
#4      000077 as lang_start_internal () at library/std/src/rt.rs
#3      000015 as lang_start<()> (main=0x01000000, argc=0x00000000, argv=0x00000000, sigpipe=0x00620000) at /rustc/b833ad56f46a0bbe0e8729512812a161e7dae28a/library/std/src/rt.rs
#2      000013 as __original_main () at <directory not found>/<file not found>
#1      000005 as _start () at <directory not found>/<file not found>
#0      000264 as _start.command_export at <no location>
```

Each line represents a frame on the program's stack, let's take one frame and explain what we see: 
```
#3      000015 as lang_start<()> (main=0x01000000, argc=0x00000000, argv=0x00000000, sigpipe=0x00620000) at /rustc/b833ad56f46a0bbe0e8729512812a161e7dae28a/library/std/src/rt.rs
 ^         ^         ^            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 |   Wasm funcidx    |                                    Func arguments                                                          Source location
 |               Func name
 |
frame number
```

Up the stack from frame #10 is Rust internals, which we can ignore for our example.

We can select a particular frame to further inspect it, let's see #9:
```
wasmgdb> f 9
000011 as process_thing (thing=0x2cff0f00) at src/main.rs
```

Inspect locals (includes Wasm func arguments):
```
wasmgdb> info locals
thing: *MyThing = 0xfff1c
```

We can see `thing`'s content:
```
wasmgdb> p (*thing)
thing (0xfff2c): MyThing = {
        value (0xfff2c):        usize = 0x00000003
}
```

We can already see the `value` content in memory but we can also access it, which is useful for nested structs:
```
wasmgdb> p (*thing)->value
value (0xfff2c): usize = 0x00000003
```

The address `0xfff2c` is the start of the `MyThing` struct, we can also inspect it using its address:
```
wasmgdb> p (MyThing) 0xfff2c
0xfff2c (0xfff2c): MyThing = {
        value (0xfff2c):        usize = 0x00000003
}
```

To confirm what we see we can also inspect frame #10:
```
wasmgdb> f 10
000012 as calculate (value=0x03000000) at src/main.rs

wasmgdb> info locals
value: usize = 0x00000003
```

As you can see, all that evidence points to the decimal value 3 being passed to the `calculate` function, which it subtracted to 10, and caused an interger overflow. We figured it out without using a single `console.log`!

Feel free to reproduce the crash and play with it yourself: https://github.com/xtuc/demo-node-wasm-coredump.

[wasmgdb]: https://github.com/xtuc/wasm-coredump/tree/main/bin/wasmgdb
[wasm-coredump-rewriter]: https://github.com/xtuc/wasm-coredump/tree/main/bin/rewriter
