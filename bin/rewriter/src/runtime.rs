use core_wasm_ast as ast;
use core_wasm_ast::traverse::{self, Visitor, VisitorContext, WasmModule};
use std::sync::Arc;

type BoxError = Box<dyn std::error::Error>;

pub(crate) fn get_runtime(
    frames_ptr_global: u32,
    frames_count_global: u32,
) -> Result<WasmModule, BoxError> {
    let contents = include_bytes!("../runtime.wasm");
    let module_ast = Arc::new(
        wasm_parser::parse(contents)
            .map_err(|err| format!("failed to parse runtime Wasm module: {}", err))?,
    );

    let visitor = RuntimeTransform {
        frames_ptr_global,
        frames_count_global,
    };
    traverse::traverse(Arc::clone(&module_ast), Arc::new(visitor));

    let module = WasmModule::new(Arc::clone(&module_ast));
    return Ok(module);
}

struct RuntimeTransform {
    frames_ptr_global: u32,
    frames_count_global: u32,
}

impl Visitor for RuntimeTransform {
    fn visit_instr<'a>(&self, ctx: &mut VisitorContext<'a, ast::Value<ast::Instr>>) {
        if let ast::Instr::global_get(globalidx) = ctx.node.value {
            match globalidx {
                0 => ctx.replace_node(ast::Instr::global_get(self.frames_ptr_global)),
                1 => ctx.replace_node(ast::Instr::global_get(self.frames_count_global)),
                _ => unreachable!(),
            }
        }

        if let ast::Instr::global_set(globalidx) = ctx.node.value {
            match globalidx {
                0 => ctx.replace_node(ast::Instr::global_set(self.frames_ptr_global)),
                1 => ctx.replace_node(ast::Instr::global_set(self.frames_count_global)),
                _ => unreachable!(),
            };
        }
    }
}
