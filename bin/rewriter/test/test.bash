set -xe

test() {
    name=$1
    shift
    args=$@
    wat2wasm --debug-names ./test/$name/initial.wast -o /tmp/"$name"_initial.wasm
    ../../target/debug/wasm-coredump-rewriter $args < /tmp/"$name"_initial.wasm > /tmp/"$name"_expected.wasm
    wasm2wat /tmp/"$name"_expected.wasm > ./test/$name/expected.wast
}

test basic
test memory --check-memory-operations
