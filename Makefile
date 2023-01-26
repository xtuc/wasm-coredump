target/debug/wasm-coredump-rewriter: rewriter/runtime/build/runtime.wasm
	cargo build

rewriter/runtime/build/runtime.wasm:
	make -C ./rewriter/runtime build/runtime.wasm

.PHONY: test
test:
	cd lib/asc-coredump && node tests/index.mjs
