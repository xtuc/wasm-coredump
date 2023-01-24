import { WASI } from 'wasi';
import { readFileSync, writeFileSync } from 'fs'
import { argv, env } from 'node:process';

const wasi = new WASI({
  args: argv,
  env,
  preopens: {},
});

const wasmBuffer = readFileSync('./build/runtime.wasm');
const importObject = { wasi_snapshot_preview1: wasi.wasiImport };

WebAssembly.instantiate(wasmBuffer, importObject).then(({ instance }) => {
    instance.exports.write_coredump()
    const mem = new Uint8Array(instance.exports.memory.buffer)
    writeFileSync("/tmp/coredump", mem)
});

