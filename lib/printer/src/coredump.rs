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
            writeln!(out, "{}(local u32 {})", tab, local)?;
        }
    }
    writeln!(out, "{})", tab)?;
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
