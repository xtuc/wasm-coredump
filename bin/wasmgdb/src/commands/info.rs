use super::Expr;
use crate::memory;
use crate::repl::print_value;
use crate::{BoxError, Context};
use colored::Colorize;

pub(crate) fn info<'src, 'input>(
    ctx: &'src Context<'src>,
    what: &'input str,
    args: Vec<Expr<'input>>,
) -> Result<(), BoxError> {
    match what {
        "types" => {
            if ctx.ddbug.types.len() == 0 {
                println!("no types defined.");
            }
            for (_, t) in &ctx.ddbug.types {
                println!("{}", t);
            }

            Ok(())
        }

        "locals" => {
            let selected_frame = ctx.selected_frame.borrow();
            let selected_frame = selected_frame.as_ref().ok_or("no selected frame")?;
            let binary_name = ctx
                .source
                .get_func_name(selected_frame.funcidx)
                .unwrap_or_else(|| "unknown".to_string());
            let func = ctx
                .ddbug
                .functions_by_linkage_name
                .get(&binary_name)
                .ok_or(format!("function {} not found", binary_name))?;

            for (name, param) in ctx.variables.borrow().iter() {
                let ty = param.ty(&ctx.ddbug).unwrap();

                let addr = memory::get_param_addr(selected_frame, func, &param)?;
                let value = print_value(ctx, addr, ty.as_ref(), 0)?;

                println!("{}: {}", name, value)
            }

            Ok(())
        }

        "symbol" => {
            let funcidx = args.get(0).ok_or("no func address or index specified")?;
            let funcidx = if let Expr::Int(funcidx) = funcidx {
                *funcidx as u32
            } else {
                return Err("Func index must be specified".into());
            };

            let func_name = ctx
                .source
                .get_func_name(funcidx)
                .unwrap_or_else(|| "unknown".to_string());

            if let Some(func) = ctx.ddbug.functions_by_linkage_name.get(&func_name) {
                let source = format!(
                    "{}/{}",
                    func.source()
                        .directory()
                        .unwrap_or_else(|| "<directory not found>"),
                    func.source().file().unwrap_or_else(|| "<file not found>")
                );

                let name = func.name().unwrap();
                println!("{} as {} at {}", funcidx.to_string().blue(), name, source);
            } else {
                println!(
                    "{} as ??? ({}) at <unknown>",
                    funcidx.to_string().blue(),
                    func_name
                );
            }
            Ok(())
        }

        "imports" => {
            let imports = ctx.source.imports();
            println!("{} import(s).", imports.len());
            let mut funcidx = 0;
            for import in imports {
                let mut printer = wasm_printer::WastPrinter {
                    module: &ctx.source,
                    out: &mut "".to_owned(),
                };
                printer.print_import(import)?;

                println!("#{}\t{}", format!("{:0>6}", funcidx).blue(), printer.out);
                funcidx += 1;
            }

            Ok(())
        }

        "globals" => {
            let globals = ctx.source.globals();
            println!("{} global(s).", globals.len());
            let mut globalidx = 0;
            for global in globals {
                let mut printer = wasm_printer::WastPrinter {
                    module: &ctx.source,
                    out: &mut "".to_owned(),
                };
                printer.print_global(global)?;

                let value = if global.global_type.mutable {
                    "???".to_owned()
                } else {
                    global.compute_value().to_string()
                };

                println!(
                    "#{}\t{} = {}",
                    format!("{:0>6}", globalidx).blue(),
                    printer.out,
                    value
                );
                globalidx += 1;
            }

            Ok(())
        }

        "functions" => {
            if ctx.ddbug.functions_by_address.len() == 0 {
                println!("no functions defined.");
            }
            let func_name = ctx.source.func_names.lock().unwrap();
            for (funcidx, func_name) in func_name.iter() {
                if let Some(func) = ctx.ddbug.functions_by_linkage_name.get(func_name) {
                    let source = format!(
                        "{}/{}",
                        func.source()
                            .directory()
                            .unwrap_or_else(|| "<directory not found>"),
                        func.source().file().unwrap_or_else(|| "<file not found>")
                    );

                    let name = func.name().unwrap();
                    println!("{} as {} at {}", funcidx.to_string().blue(), name, source);
                } else {
                    println!(
                        "{} as ??? ({}) at <unknown>",
                        funcidx.to_string().blue(),
                        func_name
                    );
                }
            }

            Ok(())
        }

        "process" => {
            let coredump = ctx.coredump()?;
            println!(
                "executable-name = {}",
                coredump.process_info.executable_name
            );
            Ok(())
        }

        _ => Err(format!("info {} not implemented", what).into()),
    }
}
