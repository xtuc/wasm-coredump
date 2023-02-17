target/debug/wasm-coredump-rewriter: lib/asc-coredump/build/runtime.wasm
	cargo build

lib/asc-coredump/build/runtime.wasm:
	make -C ./lib/asc-coredump build/runtime.wasm

.PHONY: test-runtime
test-runtime: lib/asc-coredump/build/runtime.wasm
	cd lib/asc-coredump && node tests/index.mjs

.PHONY: test-rewriter
test-rewriter: target/debug/wasm-coredump-rewriter
	cd ./bin/rewriter && \
		bash test/test.bash

.PHONY: test
test: test-runtime test-rewriter
