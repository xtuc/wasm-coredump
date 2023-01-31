target/debug/wasm-coredump-rewriter: rewriter/runtime/build/runtime.wasm
	cargo build

lib/asc-coredump/build/runtime.wasm:
	make -C ./lib/asc-coredump build/runtime.wasm

.PHONY: test
test: lib/asc-coredump/build/runtime.wasm
	cd lib/asc-coredump && node tests/index.mjs
