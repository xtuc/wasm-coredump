use std::io::Write;
use wasm_coredump_types as types;

type BoxError = Box<dyn std::error::Error + Sync + Send>;

fn write_u32(buffer: &mut Vec<u8>, n: u32) {
    buffer.extend_from_slice(&n.to_le_bytes());
}

pub(crate) fn write_unsigned_leb128(buffer: &mut Vec<u8>, n: u64) {
    leb128::write::unsigned(buffer, n).expect("could not write LEB128");
}

fn write_utf8(buffer: &mut Vec<u8>, v: &str) {
    let bytes = v.as_bytes().to_vec();
    write_unsigned_leb128(buffer, bytes.len() as u64);
    buffer.write_all(&bytes).unwrap();
}

pub fn encode_coredump_process(
    buffer: &mut Vec<u8>,
    process_info: &types::ProcessInfo,
) -> Result<(), BoxError> {
    buffer.push(0x0);
    write_utf8(buffer, &process_info.executable_name);

    Ok(())
}

pub fn encode_coredump_stack(
    buffer: &mut Vec<u8>,
    stack: &types::CoreStack,
) -> Result<(), BoxError> {
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
