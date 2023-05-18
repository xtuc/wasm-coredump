use super::{CoredumpBuilder, FrameBuilder, ThreadBuilder};

#[test]
fn test_basic() {
    let mut coredump_builder = CoredumpBuilder::new().executable_name("foo.exe");

    {
        let mut thread_builder = ThreadBuilder::new().thread_name("main-thread");

        {
            let coredump_frame = FrameBuilder::new().codeoffset(123).funcidx(456).build();
            thread_builder.add_frame(coredump_frame);
        }
        {
            let coredump_frame = FrameBuilder::new().codeoffset(789).funcidx(0).build();
            thread_builder.add_frame(coredump_frame);
        }

        coredump_builder.add_thread(thread_builder.build());
    }

    let coredump = coredump_builder.build();

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
