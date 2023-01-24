use super::Expr;
use crate::memory;
use crate::print_value;
use crate::{BoxError, Context};
use colored::Colorize;

pub(crate) fn info<'a, R: gimli::Reader>(
    ctx: &Context<R>,
    what: &'a str,
    args: Vec<Expr>,
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
            let frame = ctx.selected_frame.as_ref().ok_or("no selected frame")?;
            let func = ctx
                .ddbug
                .functions_by_linkage_name
                .get(&frame.binary_name)
                .ok_or(format!("function {} not found", frame.binary_name))?;

            for (name, param) in &ctx.variables {
                let ty = param.ty(&ctx.ddbug).unwrap();

                let addr = memory::get_param_addr(frame, func, &param)?;
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
            println!("{} import(s) =", imports.len());
            for import in imports {
                println!("{}.{}", import.module, import.name)
            }

            Ok(())
        }

        _ => Err(format!("info {} not implemented", what).into()),
    }
}
