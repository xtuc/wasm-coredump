set-frame.ts: gen-set-frame.js
	node gen-set-frame.js > $@

build/runtime.wasm: coredump.ts set-frame.ts
	asc --config ./asconfig.json --target runtime
