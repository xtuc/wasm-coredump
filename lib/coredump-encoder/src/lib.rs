#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use core::convert::Infallible;
use wasm_coredump_types as types;

fn write_unsigned_leb128(buffer: &mut Vec<u8>, n: u64) {
    leb128::write::unsigned(buffer, n).expect("could not write LEB128");
}

fn write_utf8(buffer: &mut Vec<u8>, v: &str) {
    let bytes = v.as_bytes().to_vec();
    write_unsigned_leb128(buffer, bytes.len() as u64);
    buffer.extend_from_slice(&bytes);
}

pub fn encode_coredump_process(
    buffer: &mut Vec<u8>,
    process_info: &types::ProcessInfo,
) -> Result<(), Infallible> {
    buffer.push(0x0);
    write_utf8(buffer, &process_info.executable_name);

    Ok(())
}

pub fn encode_coredump_stack(
    buffer: &mut Vec<u8>,
    stack: &types::CoreStack,
) -> Result<(), Infallible> {
    // thread-info
    {
        buffer.push(0x0); // version 0
        write_utf8(buffer, &stack.thread_info.thread_name);
    }

    // frames
    write_unsigned_leb128(buffer, stack.frames.len() as u64);

    for frame in &stack.frames {
        buffer.push(0x0); // version 0
        write_unsigned_leb128(buffer, frame.funcidx as u64);
        write_unsigned_leb128(buffer, frame.codeoffset as u64);
        write_unsigned_leb128(buffer, 0); // locals vec size
        write_unsigned_leb128(buffer, 0); // stack vec size
    }

    Ok(())
}
