//! Handles parsing of `wasm-edit` coredump

use crate::BoxError;
use log::debug;

/// Struct collect while unwinding by wasm-edit
// The stack frame relies on Wasm funcidx instead of instruction binary offset
// because it has been buggy and hard to debug in the past.
// TODO: eventually switch back to code offsets.
#[derive(Debug, Clone)]
pub(crate) struct StackFrame {
    pub(crate) funcidx: u32,
    pub(crate) binary_name: String,
    pub(crate) locals: Vec<u32>,
}

pub(crate) fn decode_coredump(
    source: &core_wasm_ast::traverse::WasmModule,
    coredump: &[u8],
) -> Result<Vec<StackFrame>, BoxError> {
    let mut addr = 0usize;
    let nframe = u32::from_le_bytes(coredump[addr..addr + 4].try_into().unwrap());
    addr += 4;
    let next_frame = u32::from_le_bytes(coredump[addr..addr + 4].try_into().unwrap());
    addr += 4;

    debug!("number of frames: {}, next_frame: {}", nframe, next_frame);

    let mut stack_frames = Vec::with_capacity(nframe as usize);

    for i in 0..nframe {
        let funcidx = u32::from_le_bytes(coredump[addr..addr + 4].try_into().unwrap());
        addr += 4;
        let count_local = u32::from_le_bytes(coredump[addr..addr + 4].try_into().unwrap());
        addr += 4;

        let mut locals = Vec::with_capacity(count_local as usize);
        for _ in 0..count_local {
            let local = u32::from_le_bytes(coredump[addr..addr + 4].try_into().unwrap());
            locals.push(local);
            addr += 4;
        }

        let frame = StackFrame {
            binary_name: source
                .get_func_name(funcidx)
                .unwrap_or_else(|| "unknown".to_string()),
            funcidx,
            locals,
        };
        debug!("#{} stack frame {:?}", nframe - i - 1, frame);
        stack_frames.push(frame);
    }
    if nframe > 0 {
        assert_eq!(next_frame as usize, addr);
    }

    Ok(stack_frames)
}
