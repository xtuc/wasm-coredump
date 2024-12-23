use crate::{memory, BoxError, Context};
use colored::Colorize;
use log::debug;

pub(crate) fn backtrace<'a>(
    ctx: &'a Context<'a>,
    thread: &wasm_coredump_types::CoreStack,
) -> Result<(), BoxError> {
    let mut i = thread.frames.len();
    for frame in &thread.frames {
        i -= 1;
        if let Some(selected_frame) = ctx.selected_frame.borrow().clone() {
            if selected_frame.funcidx == frame.funcidx {
                print!("#{}*\t", i);
            } else {
                print!("#{}\t", i);
            }
        } else {
            print!("#{}\t", i);
        }

        print_frame(ctx, &frame)?;
    }

    Ok(())
}

pub(crate) fn print_frame<'a>(
    ctx: &'a Context<'a>,
    frame: &wasm_coredump_types::StackFrame,
) -> Result<(), BoxError> {
    let coredump = ctx.coredump()?;
    let binary_name = ctx
        .source
        .get_func_name(frame.funcidx)
        .unwrap_or_else(|| "unknown".to_string());

    if let Some(func) = ctx.ddbug.functions_by_linkage_name.get(&binary_name) {
        let mut addr2line = ctx.addr2line.borrow_mut();
        let addr2line = addr2line
            .context(frame.codeoffset as u64, true)
            .map_err(|err| format!("failed to find code offset: {err}"))?
            .unwrap();

        let source = if let Ok(Some(loc)) = addr2line.0.find_location(addr2line.1) {
            format!(
                "{}:{}",
                loc.file.unwrap_or("<unknown>"),
                loc.line.unwrap_or(0)
            )
        } else {
            format!("<location 0x{} not found>", frame.codeoffset)
        };

        let function = {
            let name = func.name().unwrap();

            let params = func
                .details(&ctx.ddbug)
                .parameters()
                .iter()
                .map(|param| {
                    let param_name = if let Some(name) = param.name() {
                        name
                    } else {
                        "???"
                    };

                    // TODO: not always 4 bytes, right?
                    let size_of = 4;

                    let value = match memory::get_param_addr(frame, &func, param) {
                        Ok(abs_addr) => match memory::read(&coredump.data, abs_addr, size_of) {
                            Ok(bytes) => {
                                format!("0x{}", hex::encode(&bytes))
                            }
                            Err(err) => {
                                format!("<failed to load: {}>", err)
                            }
                        },
                        Err(err) => {
                            debug!("failed to get_param_addr: {err}");
                            "???".to_owned()
                        }
                    };

                    format!("{}={}", param_name.green(), value)
                })
                .collect::<Vec<String>>()
                .join(", ");

            format!("{} ({})", name.yellow(), params)
        };

        let addr = format!("{:0>6}", frame.funcidx).blue();
        println!("{} as {} at {}", addr, function, source);
    } else {
        // Functions that are generated by Wasi and don't have a source (ie
        // some Wasi transpolines) don't have a mapping in DWARF.
        let addr = format!("{:0>6}", frame.funcidx).blue();
        println!("{} as {} at <no location>", addr, binary_name);
    }

    Ok(())
}

pub(crate) fn select_frame<'a>(
    ctx: &Context<'a>,
    frame: &wasm_coredump_types::StackFrame,
) -> Result<(), BoxError> {
    // Clear previous selected scope
    ctx.variables.borrow_mut().clear();

    let binary_name = ctx
        .source
        .get_func_name(frame.funcidx)
        .unwrap_or_else(|| "unknown".to_string());

    let func = ctx
        .ddbug
        .functions_by_linkage_name
        .get(&binary_name)
        .ok_or(format!("function {} not found", binary_name))?;

    for param in func.details(&ctx.ddbug).parameters() {
        if let Some(name) = param.name() {
            ctx.variables
                .borrow_mut()
                .insert(name.to_owned(), param.clone());
        }
    }
    Ok(())
}
