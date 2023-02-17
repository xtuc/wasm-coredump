use crate::{decode_name, IResult, InputContext};
use core_wasm_ast as ast;
use log::debug;

pub(crate) fn decode_process_info<'a>(
    ctx: InputContext<'a>,
) -> IResult<InputContext<'a>, ast::coredump::ProcessInfo> {
    let (ctx, t) = ctx.read_u8()?;
    if t != 0 {
        unimplemented!("unsupported process-info type: {}", t);
    }

    let (ctx, name) = decode_name(ctx)?;
    let value = ast::coredump::ProcessInfo {
        executable_name: name,
    };
    Ok((ctx, value))
}

pub(crate) fn decode_thread_info<'a>(
    ctx: InputContext<'a>,
) -> IResult<InputContext<'a>, ast::coredump::ThreadInfo> {
    let (ctx, t) = ctx.read_u8()?;
    if t != 0 {
        unimplemented!("unsupported thread-info type: {}", t);
    }

    let (ctx, name) = decode_name(ctx)?;
    let value = ast::coredump::ThreadInfo { thread_name: name };
    Ok((ctx, value))
}

pub(crate) fn decode_core_stack<'a>(
    ctx: InputContext<'a>,
) -> IResult<InputContext<'a>, ast::coredump::CoreStack> {
    let (ctx, thread_info) = decode_thread_info(ctx)?;
    debug!("thread_info {:?}", thread_info);

    let (ctx, nframes) = ctx.read_u32()?;
    debug!("nframe {}", nframes);

    let (ctx, size) = ctx.read_u32()?;
    debug!("size {}", size);

    let mut frames = vec![];
    let mut ctx = ctx;
    for _ in 0..nframes {
        let res = decode_stack_frame(ctx)?;
        ctx = res.0;
        frames.push(res.1)
    }

    let value = ast::coredump::CoreStack {
        thread_info,
        frames,
    };
    Ok((ctx, value))
}

pub(crate) fn decode_stack_frame<'a>(
    ctx: InputContext<'a>,
) -> IResult<InputContext<'a>, ast::coredump::StackFrame> {
    let (ctx, code_offset) = ctx.read_u32()?;
    let (ctx, count_local) = ctx.read_u32()?;

    let mut locals = Vec::with_capacity(count_local as usize);
    let mut ctx = ctx;
    for _ in 0..count_local {
        let res = ctx.read_u8()?;
        ctx = res.0;
        let t = res.1;

        let res = match t {
            0x01 => (ctx, ast::coredump::Value::Missing),

            0x7F => {
                let (ctx, v) = ctx.read_u32()?;
                (ctx, ast::coredump::Value::I32(v as i32))
            }

            0x7E | 0x7D | 0x7C => {
                todo!()
            }

            b => {
                unimplemented!("value type {}", b)
            }
        };

        ctx = res.0;
        locals.push(res.1);
    }

    let frame = ast::coredump::StackFrame {
        code_offset,
        locals,
        stack: vec![],
    };
    debug!("stack frame {:?}", frame);
    Ok((ctx, frame))
}
