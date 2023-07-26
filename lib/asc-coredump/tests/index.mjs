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

  try {
    const out = execSync("../../target/debug/coredump-dump", { input: image, stdio: 'pipe' })
    return out.toString('utf8')
  } catch (err) {
    console.log('------------------------- stdout -----------------------');
    console.log(err.stdout.toString('utf8'));
    console.log('------------------------- /stdout -----------------------');
    console.log('------------------------- stderr -----------------------');
    console.log(err.stderr.toString('utf8'));
    console.log('------------------------- /stderr -----------------------');

    throw new Error(err.message);
  }
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
  instance.exports.start_frame(123, 2, 0);

  instance.exports.start_frame(456, 3, 5);
  instance.exports.add_i32_local(1);
  instance.exports.add_missing_local();
  instance.exports.add_f64_local(0.33333333333333);
  instance.exports.add_f32_local(0.3333333);
  instance.exports.add_i64_local(0x10000000000000n);

  instance.exports.start_frame(678, 100, 3);
  instance.exports.add_i32_local(1);
  instance.exports.add_i32_local(2);
  instance.exports.add_i32_local(3);

  // Write coredump
  instance.exports.write_coredump()

  const expected = `
(module (coredump)
    (process (name ""))
    (thread (name "main")
        (func 2 (offset 123))
        (func 3
            (offset 456)
            (local i32 1)
            (local  (optimized out))
            (local f64 0.33333333333333)
            (local f32 0.3333333)
            (local i64 4503599627370496)
        )
        (func 100
            (offset 678)
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

async function test_duplicated_coredump() {
  const instance = await get_runtime();

  // Add some memory space for coredump
  instance.exports.memory.grow(1)

  // Write a few frames
  instance.exports.start_frame(1, 0);
  instance.exports.start_frame(2, 0);
  instance.exports.start_frame(3, 0);

  // Write coredump, twice.
  instance.exports.write_coredump()
  assert.throws(instance.exports.write_coredump)
  assert.throws(() => instance.exports.start_frame(0, 0))

  console.log("test_duplicated_coredump", "OK");
}

await test_basic();
await test_duplicated_coredump();
