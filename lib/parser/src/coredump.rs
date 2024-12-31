use crate::{decode_name, IResult, InputContext};
use log::debug;

pub(crate) fn decode_process_info<'a>(
    ctx: InputContext<'a>,
) -> IResult<InputContext<'a>, wasm_coredump_types::ProcessInfo> {
    let (ctx, t) = ctx.read_u8()?;
    if t != 0 {
        unimplemented!("unsupported process-info type: {}", t);
    }

    let (ctx, name) = decode_name(ctx)?;
    let value = wasm_coredump_types::ProcessInfo {
        executable_name: name,
    };
    Ok((ctx, value))
}

pub(crate) fn decode_thread_info<'a>(
    ctx: InputContext<'a>,
) -> IResult<InputContext<'a>, wasm_coredump_types::ThreadInfo> {
    let (ctx, t) = ctx.read_u8()?;
    if t != 0 {
        unimplemented!("unsupported thread-info type: {}", t);
    }

    let (ctx, name) = decode_name(ctx)?;
    let value = wasm_coredump_types::ThreadInfo { thread_name: name };
    Ok((ctx, value))
}

pub(crate) fn decode_core_stack<'a>(
    ctx: InputContext<'a>,
) -> IResult<InputContext<'a>, wasm_coredump_types::CoreStack> {
    let (ctx, thread_info) = decode_thread_info(ctx)?;
    debug!("thread_info {:?}", thread_info);

    let (ctx, nframes) = ctx.read_leb128()?;
    debug!("nframe {}", nframes);

    let mut frames = vec![];
    let mut ctx = ctx;
    for _ in 0..nframes {
        let res = decode_stack_frame(ctx)?;
        ctx = res.0;
        frames.push(res.1)
    }

    let value = wasm_coredump_types::CoreStack {
        thread_info,
        frames,
    };
    Ok((ctx, value))
}

pub(crate) fn decode_stack_frame<'a>(
    ctx: InputContext<'a>,
) -> IResult<InputContext<'a>, wasm_coredump_types::StackFrame> {
    let (ctx, v) = ctx.read_u8()?;
    if v != 0 {
        unimplemented!("unsupported frame type {}", v);
    }

    let (ctx, instanceidx) = ctx.read_leb128()?;
    let (ctx, funcidx) = ctx.read_leb128()?;
    let (ctx, codeoffset) = ctx.read_leb128()?;
    let (ctx, count_local) = ctx.read_leb128()?;
    let (ctx, _count_stack) = ctx.read_leb128()?;

    let mut locals = Vec::with_capacity(count_local as usize);
    let mut ctx = ctx;
    for _ in 0..count_local {
        let res = ctx.read_u8()?;
        ctx = res.0;
        let t = res.1;

        let res = match t {
            0x01 => (ctx, wasm_coredump_types::Value::Missing),

            0x7F => {
                let (ctx, v) = ctx.read_i32()?;
                (ctx, wasm_coredump_types::Value::I32(v))
            }

            0x7E => {
                let (ctx, v) = ctx.read_i64()?;
                (ctx, wasm_coredump_types::Value::I64(v))
            }

            0x7D => {
                let (ctx, v) = ctx.read_f32()?;
                (ctx, wasm_coredump_types::Value::F32(v))
            }

            0x7C => {
                let (ctx, v) = ctx.read_f64()?;
                (ctx, wasm_coredump_types::Value::F64(v))
            }

            b => {
                unimplemented!("value type {}", b)
            }
        };

        ctx = res.0;
        locals.push(res.1);
    }

    let frame = wasm_coredump_types::StackFrame {
        instanceidx,
        funcidx,
        codeoffset,
        locals,
        stack: vec![],
    };
    debug!("stack frame {:?}", frame);
    Ok((ctx, frame))
}
