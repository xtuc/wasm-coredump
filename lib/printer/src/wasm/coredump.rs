use super::{write_u32, write_utf8};
use core_wasm_ast as ast;

type BoxError = Box<dyn std::error::Error + Sync + Send>;

pub(crate) fn write_section_custom_coredump(
    buffer: &mut Vec<u8>,
    process_info: &ast::coredump::ProcessInfo,
) -> Result<(), BoxError> {
    write_utf8(buffer, "core");

    buffer.push(0x0);
    write_utf8(buffer, &process_info.executable_name);

    Ok(())
}

pub(crate) fn write_section_custom_coredump_stack(
    buffer: &mut Vec<u8>,
    stack: &ast::coredump::CoreStack,
) -> Result<(), BoxError> {
    write_utf8(buffer, "corestack");

    // thread-info
    buffer.push(0x0);
    write_utf8(buffer, &stack.thread_info.thread_name);

    // frames
    write_u32(buffer, stack.frames.len() as u32);
    write_u32(buffer, 0); // size unused

    for frame in &stack.frames {
        write_u32(buffer, frame.code_offset);
        write_u32(buffer, 0); // locals

        // write_u32(buffer, 0); // stack
        // write_u32(buffer, 0); // reserved
    }

    Ok(())
}
