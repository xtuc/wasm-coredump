use crate::parse;
use std::sync::Arc;

#[test]
fn test_basic() {
    let mut coredump_builder =
        wasm_coredump_builder::CoredumpBuilder::new().executable_name("foo.exe");

    {
        let mut thread_builder =
            wasm_coredump_builder::ThreadBuilder::new().thread_name("main-thread");

        {
            let coredump_frame = wasm_coredump_builder::FrameBuilder::new()
                .codeoffset(123)
                .funcidx(456)
                .build();
            thread_builder.add_frame(coredump_frame);
        }
        {
            let coredump_frame = wasm_coredump_builder::FrameBuilder::new()
                .codeoffset(789)
                .funcidx(0)
                .build();
            thread_builder.add_frame(coredump_frame);
        }

        coredump_builder.add_thread(thread_builder.build());
    }

    let coredump_wasm = coredump_builder.serialize().unwrap();
    let coredump_wasm = parse(&coredump_wasm).unwrap();
    let coredump_wasm = core_wasm_ast::traverse::WasmModule::new(Arc::new(coredump_wasm));
    let coredump = coredump_wasm.get_coredump().unwrap();

    let mut out = String::new();
    wasm_printer::wast::coredump::dump_coredump(&mut out, &coredump).unwrap();

    assert_eq!(
        out,
        r#"(module (coredump)
    (process (name "foo.exe"))
    (thread (name "main-thread")
        (func 456 (offset 123))
        (func 0 (offset 789))
    )
    (memory 0)
)"#
    );
}
