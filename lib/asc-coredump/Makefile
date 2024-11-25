build/runtime.wasm: assembly/coredump.ts
	asc --config ./asconfig.json --target runtime

build/runtime-wasi.wasm: assembly/coredump-wasi.ts
	asc --config ./asconfig-wasi.json --target runtime

.PHONY: build
build: build/runtime.wasm build/runtime-wasi.wasm
