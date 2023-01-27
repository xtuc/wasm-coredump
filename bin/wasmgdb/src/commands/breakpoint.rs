use crate::repl::Context;
use crate::BoxError;
use colored::Colorize;
use core_wasm_ast as ast;
use core_wasm_ast::traverse::{self, Visitor, VisitorContext, WasmModule};
use std::sync::Arc;

pub(crate) fn set_breakpoint(ctx: &mut Context, pos: u32) -> Result<(), BoxError> {
    ctx.break_points.lock().unwrap().insert(pos);

    let module = WasmModule::new(Arc::clone(&ctx.source.inner));
    let func_name = module.get_func_name(pos).ok_or("function not found")?;

    let func = {
        let func = ctx.ddbug.functions_by_linkage_name.get(&func_name).unwrap();
        let source = format!(
            "{}/{}",
            func.source()
                .directory()
                .unwrap_or_else(|| "<directory not found>"),
            func.source().file().unwrap_or_else(|| "<file not found>")
        );

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

                    format!("{}", param_name.green())
                })
                .collect::<Vec<String>>()
                .join(", ");

            format!("{} ({})", name.yellow(), params)
        };

        let addr = format!("{:0>6}", pos).blue();
        format!("{} as {} at {}", addr, function, source)
    };

    // FIXME: move to rewriter.
    let visitor = AddBreakpointVisitor { funcidx: pos };
    traverse::traverse(Arc::clone(&&ctx.source.inner), Arc::new(visitor));

    println!("Breakpoint added on {}", func);
    Ok(())
}

struct AddBreakpointVisitor {
    funcidx: u32,
}

impl Visitor for AddBreakpointVisitor {
    fn visit_code<'a>(&self, ctx: &'_ mut VisitorContext<'a, ast::Code>, funcidx: u32) {
        if funcidx == self.funcidx {
            let breakpoint = vec![ast::Value::new(ast::Instr::unreachable)];

            let mut body = ctx.node.body.lock().unwrap();
            body.value = prepend(body.value.clone(), &breakpoint);
        }
    }
}

fn prepend<T>(v: Vec<T>, s: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut tmp: Vec<_> = s.to_owned();
    tmp.extend(v);
    tmp
}
