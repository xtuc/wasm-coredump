.PHONY: setup
setup:
	cd ./lib/asc-coredump && yarn

target/debug/coredump-dump:
	cargo build

target/debug/wasm-coredump-rewriter: lib/asc-coredump/build/runtime.wasm
	cargo build

lib/asc-coredump/build/runtime.wasm: lib/asc-coredump/assembly/coredump.ts
	make -C ./lib/asc-coredump build/runtime.wasm

.PHONY: test-runtime
test-runtime: lib/asc-coredump/build/runtime.wasm target/debug/coredump-dump
	cd lib/asc-coredump && node tests/index.mjs

.PHONY: test-rewriter
test-rewriter: target/debug/wasm-coredump-rewriter
	cd ./bin/rewriter && \
		bash test/test.bash

.PHONY: test
test: test-runtime test-rewriter
	cargo test

.PHONY: publish
publish:
	cd ./lib/coredump-types && cargo publish
	cd ./lib/coredump-encoder && cargo publish
	cd ./lib/printer && cargo publish
	cd ./lib/coredump-builder && cargo publish
	cd ./lib/ast && cargo publish
	cd ./lib/parser && cargo publish
	cd ./bin/rewriter && cargo publish --allow-dirty
	cd ./bin/wasmgdb && cargo publish
	cd ./lib/coredump-to-stack && cargo publish
	cd ./bin/debuginfo-split && cargo publish
