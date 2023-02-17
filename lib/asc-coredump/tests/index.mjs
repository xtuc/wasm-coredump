import assert from "assert/strict";
import { argv, env } from 'node:process';
import { readFile, writeFile } from 'node:fs/promises';
import { execSync } from 'child_process'

async function get_runtime() {
  const importObject = {};
  const wasm = await WebAssembly.compile(
    await readFile(new URL('../build/runtime.wasm', import.meta.url)),
  );
  return await WebAssembly.instantiate(wasm, importObject);
}

function get_coredump(instance) {
  const image = new Uint8Array(instance.exports.memory.buffer);

  const out = execSync("../../target/debug/coredump-dump", { input: image })
  return out.toString('utf8')
}

function expect_coredump(instance, expected) {
  const coredump = get_coredump(instance);
  assert.equal(coredump.trim(), expected.trim())
}

async function test_basic() {
  const instance = await get_runtime();

  // Add some memory space for coredump
  instance.exports.memory.grow(1)

  // Write a few frames
  instance.exports.set_frame0(3);
  instance.exports.set_frame5(3,
      0x7F, 1,
      0x7F, 2,
      0x7F, 3,
      0x7F, 4,
      0x7F, 5);
  instance.exports.set_frame3(100,
      0x7F, 1,
      0x7F, 2,
      0x7F, 3);

  // Write coredump
  instance.exports.write_coredump()

  const expected = `
(module (coredump)
    (process (name ""))
    (thread (name "main")
        (func 3
        )
        (func 3
            (local i32 1)
            (local i32 2)
            (local i32 3)
            (local i32 4)
            (local i32 5)
        )
        (func 100
            (local i32 1)
            (local i32 2)
            (local i32 3)
        )
    )
    (data (i32.const 0) "...65535 bytes")
    (memory 0 1)
)`;

  expect_coredump(instance, expected)
  console.log("test_basic", "OK");
}

await test_basic();
