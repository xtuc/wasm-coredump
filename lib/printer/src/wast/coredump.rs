use std::fmt::Write;

type BoxError = Box<dyn std::error::Error>;

const TAB: &str = "    ";

pub fn dump_coredump<W: Write>(
    out: &mut W,
    coredump: &wasm_coredump_types::Coredump,
) -> Result<(), BoxError> {
    writeln!(out, "(module (coredump)")?;
    dump_process_info(out, 1, &coredump.process_info)?;
    for stack in &coredump.stacks {
        dump_stack(out, 1, &stack)?;
    }
    if !coredump.data.is_empty() {
        dump_data(out, 1, &coredump.data)?;
    }
    dump_memory(out, 1, &coredump.memory)?;
    write!(out, ")")?;

    Ok(())
}

fn dump_process_info<W: Write>(
    out: &mut W,
    depth: usize,
    process_info: &wasm_coredump_types::ProcessInfo,
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
    stack: &wasm_coredump_types::CoreStack,
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
    frame: &wasm_coredump_types::StackFrame,
) -> Result<(), BoxError> {
    let tab = TAB.repeat(depth);

    if frame.locals.len() > 0 {
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
    } else {
        writeln!(out, "{}(func {})", tab, frame.code_offset)?;
    }
    Ok(())
}

fn dump_value<W: Write>(
    out: &mut W,
    _depth: usize,
    value: &wasm_coredump_types::Value,
) -> Result<(), BoxError> {
    match value {
        wasm_coredump_types::Value::Missing => {
            write!(out, "(optimized out)")?;
        }
        wasm_coredump_types::Value::I32(v) => {
            write!(out, "{}", v)?;
        }
        wasm_coredump_types::Value::I64(v) => {
            write!(out, "{}", v)?;
        }
        wasm_coredump_types::Value::F32(v) => {
            write!(out, "{}", v)?;
        }
        wasm_coredump_types::Value::F64(v) => {
            write!(out, "{}", v)?;
        }
    }

    Ok(())
}

fn dump_value_type<W: Write>(
    out: &mut W,
    _depth: usize,
    value: &wasm_coredump_types::Value,
) -> Result<(), BoxError> {
    let v = match value {
        wasm_coredump_types::Value::Missing => "",
        wasm_coredump_types::Value::I32(_) => "i32",
        wasm_coredump_types::Value::I64(_) => "i64",
        wasm_coredump_types::Value::F32(_) => "f32",
        wasm_coredump_types::Value::F64(_) => "f64",
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
    memories: &Vec<(u32, Option<u32>)>,
) -> Result<(), BoxError> {
    let tab = TAB.repeat(depth);

    for memory in memories {
        write!(out, "{}(memory {}", tab, memory.0)?;
        if let Some(max) = memory.1 {
            writeln!(out, " {})", max)?;
        } else {
            writeln!(out, ")")?;
        }
    }
    Ok(())
}
