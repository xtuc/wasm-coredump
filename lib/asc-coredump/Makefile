assembly/set-frame.ts: gen-set-frame.js
	node gen-set-frame.js > $@

build/runtime.wasm: assembly/coredump.ts assembly/set-frame.ts
	asc --config ./asconfig.json --target runtime

.PHONY: build
build: build/runtime.wasm
