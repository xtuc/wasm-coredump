use core_wasm_ast as ast;
use std::fmt::Write;

type BoxError = Box<dyn std::error::Error>;

const TAB: &str = "    ";

pub fn dump_coredump<W: Write>(
    out: &mut W,
    coredump: &ast::coredump::Coredump,
) -> Result<(), BoxError> {
    writeln!(out, "(module (coredump)")?;
    dump_process_info(out, 1, &coredump.process_info)?;
    for stack in &coredump.stacks {
        dump_stack(out, 1, &stack)?;
    }
    dump_data(out, 1, &coredump.data)?;
    dump_memory(out, 1, &coredump.memory)?;
    write!(out, ")")?;

    Ok(())
}

fn dump_process_info<W: Write>(
    out: &mut W,
    depth: usize,
    process_info: &ast::coredump::ProcessInfo,
) -> Result<(), BoxError> {
    let tab = TAB.repeat(depth);
    write!(out, "{}(process", tab)?;
    write!(out, " (name \"{}\")", process_info.executable_name)?;
    writeln!(out, ")")?;
    Ok(())
}

fn dump_stack<W: Write>(
    out: &mut W,
    depth: usize,
    stack: &ast::coredump::CoreStack,
) -> Result<(), BoxError> {
    let tab = TAB.repeat(depth);
    write!(out, "{}(thread", tab)?;
    writeln!(out, " (name \"{}\")", stack.thread_info.thread_name)?;
    {
        for frame in &stack.frames {
            dump_frame(out, depth + 1, frame)?;
        }
    }
    writeln!(out, "{})", tab)?;
    Ok(())
}

fn dump_frame<W: Write>(
    out: &mut W,
    depth: usize,
    frame: &ast::coredump::StackFrame,
) -> Result<(), BoxError> {
    let tab = TAB.repeat(depth);
    writeln!(out, "{}(func {}", tab, frame.code_offset)?;
    {
        let tab = TAB.repeat(depth + 1);
        for local in &frame.locals {
            write!(out, "{}(local ", tab)?;
            dump_value_type(out, 0, local)?;
            write!(out, " ")?;
            dump_value(out, 0, local)?;
            writeln!(out, ")")?;
        }
    }
    writeln!(out, "{})", tab)?;
    Ok(())
}

fn dump_value<W: Write>(
    out: &mut W,
    _depth: usize,
    value: &ast::coredump::Value,
) -> Result<(), BoxError> {
    match value {
        ast::coredump::Value::Missing => {
            write!(out, "(optimized out)")?;
        }
        ast::coredump::Value::I32(v) => {
            write!(out, "{}", v)?;
        }
        ast::coredump::Value::I64(_v) => {
            todo!()
        }
        ast::coredump::Value::F32(_v) => {
            todo!()
        }
        ast::coredump::Value::F64(_v) => {
            todo!()
        }
    }

    Ok(())
}

fn dump_value_type<W: Write>(
    out: &mut W,
    _depth: usize,
    value: &ast::coredump::Value,
) -> Result<(), BoxError> {
    let v = match value {
        ast::coredump::Value::Missing => "",
        ast::coredump::Value::I32(_) => "i32",
        ast::coredump::Value::I64(_) => "i64",
        ast::coredump::Value::F32(_) => "f32",
        ast::coredump::Value::F64(_) => "f64",
    };
    write!(out, "{}", v)?;

    Ok(())
}

fn dump_data<W: Write>(out: &mut W, depth: usize, data: &[u8]) -> Result<(), BoxError> {
    let tab = TAB.repeat(depth);
    write!(out, "{}(data", tab)?;
    write!(out, " (i32.const 0)")?;
    write!(out, " \"...{} bytes\"", data.len())?;
    writeln!(out, ")")?;
    Ok(())
}

fn dump_memory<W: Write>(
    out: &mut W,
    depth: usize,
    memories: &Vec<ast::Memory>,
) -> Result<(), BoxError> {
    let tab = TAB.repeat(depth);

    for memory in memories {
        write!(out, "{}(memory {}", tab, memory.min.value)?;
        if let Some(max) = memory.max {
            writeln!(out, " {})", max)?;
        } else {
            writeln!(out, ")")?;
        }
    }
    Ok(())
}
