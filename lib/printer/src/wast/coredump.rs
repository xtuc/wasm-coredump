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
    for global in &coredump.globals {
        dump_global(out, 1, global)?;
    }
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

fn dump_global<W: Write>(out: &mut W, depth: usize, global: &ast::Global) -> Result<(), BoxError> {
    let tab = TAB.repeat(depth);

    write!(out, "{}(global ", tab)?;
    let t = match global.global_type.valtype {
        ast::ValueType::NumType(ast::NumType::F32) => "f32",
        ast::ValueType::NumType(ast::NumType::F64) => "f64",
        ast::ValueType::NumType(ast::NumType::I32) => "i32",
        ast::ValueType::NumType(ast::NumType::I64) => "i64",
    };
    write!(out, "{} ", t)?;
    writeln!(out, " (i32.const {}))", global.compute_value())?;
    Ok(())
}
