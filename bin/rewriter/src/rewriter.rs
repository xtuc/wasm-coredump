//! Guest coredump generation.
//!
//! Informations about the stack is recorded at offset 0 in memory with the
//! following structure:
//!
//! | frame* |
//!
//! Where a `frame` is the Coredump frame encoding.

use crate::runtime::get_runtime;
use core_wasm_ast as ast;
use core_wasm_ast::traverse::{self, Visitor, VisitorContext, WasmModule};
use log::debug;
use std::sync::Arc;
use std::sync::Mutex;

const NO_ENTRY_FUNCIDX_VALUE: i32 = i32::MAX;

type BoxError = Box<dyn std::error::Error>;

pub fn rewrite(
    module_ast: Arc<ast::Module>,
    check_memory_operations: bool,
) -> Result<(), BoxError> {
    let module = WasmModule::new(Arc::clone(&module_ast));

    // Pointer or cursor to the latest frame
    let frames_ptr_global = {
        let expr = ast::Value::new(vec![
            ast::Value::new(ast::Instr::i32_const(0)),
            ast::Value::new(ast::Instr::end),
        ]);
        let global = ast::Global {
            global_type: ast::GlobalType {
                valtype: ast::ValueType::NumType(ast::NumType::I32),
                mutable: true,
            },
            expr,
        };
        module.add_global(&global).unwrap()
    };
    debug!("frames_ptr_global global at {}", frames_ptr_global);

    // Keep track of number of frames
    let frames_count_global = {
        let expr = ast::Value::new(vec![
            ast::Value::new(ast::Instr::i32_const(0)),
            ast::Value::new(ast::Instr::end),
        ]);
        let global = ast::Global {
            global_type: ast::GlobalType {
                valtype: ast::ValueType::NumType(ast::NumType::I32),
                mutable: true,
            },
            expr,
        };
        module.add_global(&global).unwrap()
    };
    debug!("frames_count_global global at {}", frames_count_global);

    let runtime = get_runtime(frames_ptr_global, frames_count_global)?;

    debug!(
        "code section starts at {}",
        module.get_code_section_start_offset().unwrap()
    );

    // Add `is_unwinding` global
    let is_unwinding = {
        let expr = ast::Value::new(vec![
            ast::Value::new(ast::Instr::i32_const(0)),
            ast::Value::new(ast::Instr::end),
        ]);
        let global = ast::Global {
            global_type: ast::GlobalType {
                valtype: ast::ValueType::NumType(ast::NumType::I32),
                mutable: true,
            },
            expr,
        };
        module.add_global(&global).unwrap()
    };
    debug!("is_unwinding global at {}", is_unwinding);

    // Add `entry_funcidx` global. Tracking the exported function
    // was the entrypoint
    let entry_funcidx = {
        let expr = ast::Value::new(vec![
            ast::Value::new(ast::Instr::i32_const(NO_ENTRY_FUNCIDX_VALUE as i64)),
            ast::Value::new(ast::Instr::end),
        ]);
        let global = ast::Global {
            global_type: ast::GlobalType {
                valtype: ast::ValueType::NumType(ast::NumType::I32),
                mutable: true,
            },
            expr,
        };
        module.add_global(&global).unwrap()
    };
    debug!("entry_funcidx global at {}", entry_funcidx);

    // Add `unreachable_shim`
    let unreachable_shim = {
        let t = ast::make_type! {};
        let typeidx = module.add_type(&t);

        let body = ast::body![[
            ast::Value::new(ast::Instr::i32_const(1)),
            ast::Value::new(ast::Instr::global_set(is_unwinding))
        ]];
        let func = ast::Code {
            locals: vec![],
            size: ast::Value::new(0), // printer calculates based on the body
            body: Arc::new(Mutex::new(body)),
        };
        let funcidx = module.add_function(&func, typeidx);
        module.add_func_name(funcidx, "coredump/unreachable_shim");
        funcidx
    };
    debug!("unreachable_shim func at {}", unreachable_shim);

    let write_coredump = {
        let (func, t) = runtime
            .get_export_func("write_coredump")
            .expect("failed to get write_coredump");
        let typeidx = module.add_type(&t);
        let funcidx = module.add_function(&func, typeidx);
        module.add_func_name(funcidx, "coredump/write_coredump");
        funcidx
    };
    debug!("write_coredump func at {}", write_coredump);

    let start_frame = {
        let (func, t) = runtime
            .get_export_func("start_frame")
            .expect("failed to get start_frame");
        let typeidx = module.add_type(&t);
        let funcidx = module.add_function(&func, typeidx);
        module.add_func_name(funcidx, "coredump/start_frame");
        funcidx
    };
    debug!("start_frame func at {}", start_frame);

    let add_i32_local = {
        let (func, t) = runtime
            .get_export_func("add_i32_local")
            .expect("failed to get add_i32_local");
        let typeidx = module.add_type(&t);
        let funcidx = module.add_function(&func, typeidx);
        module.add_func_name(funcidx, "coredump/add_i32_local");
        funcidx
    };
    debug!("add_i32_local func at {}", add_i32_local);

    let add_f32_local = {
        let (func, t) = runtime
            .get_export_func("add_f32_local")
            .expect("failed to get add_f32_local");
        let typeidx = module.add_type(&t);
        let funcidx = module.add_function(&func, typeidx);
        module.add_func_name(funcidx, "coredump/add_f32_local");
        funcidx
    };
    debug!("add_f32_local func at {}", add_f32_local);

    let add_f64_local = {
        let (func, t) = runtime
            .get_export_func("add_f64_local")
            .expect("failed to get add_f64_local");
        let typeidx = module.add_type(&t);
        let funcidx = module.add_function(&func, typeidx);
        module.add_func_name(funcidx, "coredump/add_f64_local");
        funcidx
    };
    debug!("add_f64_local func at {}", add_f64_local);

    let add_i64_local = {
        let (func, t) = runtime
            .get_export_func("add_i64_local")
            .expect("failed to get add_i64_local");
        let typeidx = module.add_type(&t);
        let funcidx = module.add_function(&func, typeidx);
        module.add_func_name(funcidx, "coredump/add_i64_local");
        funcidx
    };
    debug!("add_i64_local func at {}", add_i64_local);

    let visitor = CoredumpTransform {
        is_unwinding,
        entry_funcidx,
        unreachable_shim,
        write_coredump,
        start_frame,

        add_i32_local,
        add_i64_local,
        add_f32_local,
        add_f64_local,

        check_memory_operations,
    };
    traverse::traverse(Arc::clone(&module_ast), Arc::new(visitor));

    Ok(())
}

pub fn locals_flatten(locals: Vec<ast::CodeLocal>) -> Vec<ast::CodeLocal> {
    let mut out = Vec::new();

    for local in locals {
        for _ in 0..local.count {
            out.push(ast::CodeLocal {
                count: 1,
                value_type: local.value_type.clone(),
            });
        }
    }

    out
}

struct CoredumpTransform {
    is_unwinding: u32,
    entry_funcidx: u32,
    unreachable_shim: u32,
    write_coredump: u32,
    start_frame: u32,

    add_i32_local: u32,
    add_i64_local: u32,
    add_f32_local: u32,
    add_f64_local: u32,

    check_memory_operations: bool,
}

fn prepend<T>(v: Vec<T>, s: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut tmp: Vec<_> = s.to_owned();
    tmp.extend(v);
    tmp
}

impl Visitor for CoredumpTransform {
    fn visit_code<'a>(&self, ctx: &'_ mut VisitorContext<'a, ast::Code>, funcidx: u32) {
        if ctx.module.is_func_exported(funcidx) {
            let mut func_body = ctx.node.body.lock().unwrap();

            let mut new_code = vec![];

            new_code.push(ast::Value::new(ast::Instr::global_get(self.entry_funcidx)));
            new_code.push(ast::Value::new(ast::Instr::i32_const(
                NO_ENTRY_FUNCIDX_VALUE as i64,
            )));
            new_code.push(ast::Value::new(ast::Instr::i32_eq));

            let mut if_body = vec![];
            {
                if_body.push(ast::Value::new(ast::Instr::i32_const(funcidx as i64)));
                if_body.push(ast::Value::new(ast::Instr::global_set(self.entry_funcidx)));
                if_body.push(ast::Value::new(ast::Instr::end));
            }

            let if_body = ast::Value::new(if_body);
            let if_node = ast::Instr::If(ast::BlockType::Empty, Arc::new(Mutex::new(if_body)));
            new_code.push(ast::Value::new(if_node));

            func_body.value = prepend(func_body.value.clone(), &new_code);
        }
    }

    fn visit_instr<'a>(&self, ctx: &mut VisitorContext<'a, ast::Value<ast::Instr>>) {
        let curr_funcidx = ctx.curr_funcidx.unwrap_or_default();
        let curr_func_type = ctx.module.get_func_type(curr_funcidx);

        // Don't transform our own runtime functions
        if curr_funcidx == self.unreachable_shim
            || curr_funcidx == self.write_coredump
            || curr_funcidx == self.start_frame
            || curr_funcidx == self.add_i32_local
            || curr_funcidx == self.add_i64_local
            || curr_funcidx == self.add_f32_local
            || curr_funcidx == self.add_f64_local
        {
            return;
        }

        // Replace the `unreachable` instruction with our runtime, for all
        // instructions except the one in our runtime.
        if matches!(ctx.node.value, ast::Instr::unreachable) {
            // call unreachable_shim
            {
                let unreachable_shim = Arc::new(Mutex::new(ast::Value::new(self.unreachable_shim)));
                ctx.insert_node_before(ast::Instr::call(unreachable_shim));
            }

            // create stack frame
            {
                let func_locals = ctx.module.func_locals(curr_funcidx);
                let locals = locals_flatten(func_locals);

                let param_count = curr_func_type.params.len();

                // In Wasm DWARF the offset is relative to the start of the
                // code section.
                // https://yurydelendik.github.io/webassembly-dwarf/#pc
                // let code_offset = ctx.node.start_offset as i64
                //     - ctx.module.get_code_section_start_offset().unwrap() as i64;
                // body.push(ast::Value::new(ast::Instr::i32_const(code_offset as i64)));
                // FIXME: we use the funcidx because the code offset isn't accurate
                // or buggy.
                ctx.insert_node_before(ast::Instr::i32_const(curr_funcidx as i64));
                ctx.insert_node_before(ast::Instr::i32_const((locals.len() + param_count) as i64)); // value count

                let start_frame = Arc::new(Mutex::new(ast::Value::new(self.start_frame)));
                ctx.insert_node_before(ast::Instr::call(start_frame)); // value count

                // Collect function params
                // TODO; eventually share code with locals
                {
                    let mut i = 0;
                    for param in &curr_func_type.params {
                        ctx.insert_node_before(ast::Instr::local_get(i as u32));

                        match param {
                            ast::ValueType::NumType(ast::NumType::I64) => {
                                let add_i64_local =
                                    Arc::new(Mutex::new(ast::Value::new(self.add_i64_local)));
                                ctx.insert_node_before(ast::Instr::call(add_i64_local));
                            }

                            ast::ValueType::NumType(ast::NumType::F64) => {
                                let add_f64_local =
                                    Arc::new(Mutex::new(ast::Value::new(self.add_f64_local)));
                                ctx.insert_node_before(ast::Instr::call(add_f64_local));
                            }

                            ast::ValueType::NumType(ast::NumType::F32) => {
                                let add_f32_local =
                                    Arc::new(Mutex::new(ast::Value::new(self.add_f32_local)));
                                ctx.insert_node_before(ast::Instr::call(add_f32_local));
                            }

                            ast::ValueType::NumType(ast::NumType::I32) => {
                                let add_i32_local =
                                    Arc::new(Mutex::new(ast::Value::new(self.add_i32_local)));
                                ctx.insert_node_before(ast::Instr::call(add_i32_local));
                            }
                        }

                        i += 1;
                    }
                }

                // Collect locals (so after the function params)
                // Usually Rust stores base/stack pointers the first few locals.
                // TODO; eventually share code with params
                let mut local_count = curr_func_type.params.len() as u32;

                for local in locals {
                    ctx.insert_node_before(ast::Instr::local_get(local_count));

                    if local.value_type == ast::ValueType::NumType(ast::NumType::I64) {
                        let add_i64_local =
                            Arc::new(Mutex::new(ast::Value::new(self.add_i64_local)));
                        ctx.insert_node_before(ast::Instr::call(add_i64_local));
                    }

                    if local.value_type == ast::ValueType::NumType(ast::NumType::F64) {
                        let add_f64_local =
                            Arc::new(Mutex::new(ast::Value::new(self.add_f64_local)));
                        ctx.insert_node_before(ast::Instr::call(add_f64_local));
                    }

                    if local.value_type == ast::ValueType::NumType(ast::NumType::F32) {
                        let add_f32_local =
                            Arc::new(Mutex::new(ast::Value::new(self.add_f32_local)));
                        ctx.insert_node_before(ast::Instr::call(add_f32_local));
                    }

                    if local.value_type == ast::ValueType::NumType(ast::NumType::I32) {
                        let add_i32_local =
                            Arc::new(Mutex::new(ast::Value::new(self.add_i32_local)));
                        ctx.insert_node_before(ast::Instr::call(add_i32_local));
                    }

                    local_count += 1;
                }
            }

            // Return from the current function
            // Add values on the stack to satisfy the current function result
            // type. Values don't need to be meaningful.
            {
                for result in &curr_func_type.results {
                    match result {
                        ast::ValueType::NumType(ast::NumType::I32) => {
                            ctx.insert_node_before(ast::Instr::i32_const(666));
                        }
                        ast::ValueType::NumType(ast::NumType::I64) => {
                            ctx.insert_node_before(ast::Instr::i64_const(666));
                        }
                        ast::ValueType::NumType(ast::NumType::F32) => {
                            ctx.insert_node_before(ast::Instr::f32_const(666.0));
                        }
                        ast::ValueType::NumType(ast::NumType::F64) => {
                            ctx.insert_node_before(ast::Instr::f64_const(666.0));
                        }
                    }
                }
            }

            ctx.replace_node(ast::Instr::Return);

            // We don't need to continue in the func, it's unreachable.
            ctx.stop_traversal();
            return;
        }

        if self.check_memory_operations {
            if matches!(ctx.node.value, ast::Instr::i32_load(_, _)) {
                let curr_funcidx = ctx.curr_funcidx.unwrap();
                // At this point we have one i32 on the stack; the memory address.
                // Save it in a local.
                // FIXME: check if function already has our local
                let address_local = {
                    let local = ast::CodeLocal {
                        count: 1,
                        value_type: ast::ValueType::NumType(ast::NumType::I32),
                    };
                    let localidx = ctx.module.func_locals_count(curr_funcidx);
                    assert!(ctx.module.add_func_local(curr_funcidx, local));
                    ctx.insert_node_before(ast::Instr::local_tee(localidx));

                    localidx
                };

                // Compute the amount of memory available
                // In the future we could compute it ahead of time, after
                // each memory.grow/shrink call.
                {
                    ctx.insert_node_before(ast::Instr::memory_size(0));
                    ctx.insert_node_before(ast::Instr::i32_const(16 * 1024)); // page size
                    ctx.insert_node_before(ast::Instr::i32_mul);
                }

                // Check that the available memory is greater than the address
                // it's trying to read from.
                ctx.insert_node_before(ast::Instr::i32_gt_u);

                // Build the consequent branch
                let consequent = {
                    if ctx.module.is_func_exported(curr_funcidx) {
                        // The function with the memory fault is exported, at the
                        // edge of the module, write the coredump instead of
                        // unwinding.
                        let write_coredump =
                            Arc::new(Mutex::new(ast::Value::new(self.write_coredump)));
                        ast::Value::new(vec![
                            ast::Value::new(ast::Instr::call(write_coredump)),
                            ast::Value::new(ast::Instr::unreachable),
                            ast::Value::new(ast::Instr::end),
                        ])
                    } else {
                        // FIXME: add a frame here to mark the callsite

                        let unreachable_shim =
                            Arc::new(Mutex::new(ast::Value::new(self.unreachable_shim)));
                        let mut body = vec![ast::Value::new(ast::Instr::call(unreachable_shim))];

                        // create stack frame
                        // FIXME: duplicated with line 226
                        {
                            let func_locals = ctx.module.func_locals(curr_funcidx);
                            let locals = locals_flatten(func_locals);

                            let param_count = curr_func_type.params.len();

                            // In Wasm DWARF the offset is relative to the start of the
                            // code section.
                            // https://yurydelendik.github.io/webassembly-dwarf/#pc
                            // let code_offset = ctx.node.start_offset as i64
                            //     - ctx.module.get_code_section_start_offset().unwrap() as i64;
                            // body.push(ast::Value::new(ast::Instr::i32_const(code_offset as i64)));
                            // FIXME: we use the funcidx because the code offset isn't accurate
                            // or buggy.
                            body.push(ast::Value::new(ast::Instr::i32_const(curr_funcidx as i64)));
                            body.push(ast::Value::new(ast::Instr::i32_const(
                                (locals.len() + param_count) as i64,
                            ))); // value count

                            let start_frame =
                                Arc::new(Mutex::new(ast::Value::new(self.start_frame)));
                            body.push(ast::Value::new(ast::Instr::call(start_frame))); // value count

                            // Collect function params
                            // TODO; eventually share code with locals
                            {
                                let mut i = 0;
                                for param in &curr_func_type.params {
                                    body.push(ast::Value::new(ast::Instr::local_get(i as u32)));

                                    match param {
                                        ast::ValueType::NumType(ast::NumType::I64) => {
                                            let add_i64_local = Arc::new(Mutex::new(
                                                ast::Value::new(self.add_i64_local),
                                            ));
                                            body.push(ast::Value::new(ast::Instr::call(
                                                add_i64_local,
                                            )));
                                        }

                                        ast::ValueType::NumType(ast::NumType::F64) => {
                                            let add_f64_local = Arc::new(Mutex::new(
                                                ast::Value::new(self.add_f64_local),
                                            ));
                                            body.push(ast::Value::new(ast::Instr::call(
                                                add_f64_local,
                                            )));
                                        }

                                        ast::ValueType::NumType(ast::NumType::F32) => {
                                            let add_f32_local = Arc::new(Mutex::new(
                                                ast::Value::new(self.add_f32_local),
                                            ));
                                            body.push(ast::Value::new(ast::Instr::call(
                                                add_f32_local,
                                            )));
                                        }

                                        ast::ValueType::NumType(ast::NumType::I32) => {
                                            let add_i32_local = Arc::new(Mutex::new(
                                                ast::Value::new(self.add_i32_local),
                                            ));
                                            body.push(ast::Value::new(ast::Instr::call(
                                                add_i32_local,
                                            )));
                                        }
                                    }

                                    i += 1;
                                }
                            }

                            // Collect the base/stack pointer, usually Rust stores it in
                            // the first few locals (so after the function params).
                            let mut local_count = curr_func_type.params.len() as u32;

                            for local in locals {
                                body.push(ast::Value::new(ast::Instr::local_get(local_count)));

                                if local.value_type == ast::ValueType::NumType(ast::NumType::I64) {
                                    let add_i64_local =
                                        Arc::new(Mutex::new(ast::Value::new(self.add_i64_local)));
                                    body.push(ast::Value::new(ast::Instr::call(add_i64_local)));
                                }

                                if local.value_type == ast::ValueType::NumType(ast::NumType::F64) {
                                    let add_f64_local =
                                        Arc::new(Mutex::new(ast::Value::new(self.add_f64_local)));
                                    body.push(ast::Value::new(ast::Instr::call(add_f64_local)));
                                }

                                if local.value_type == ast::ValueType::NumType(ast::NumType::F32) {
                                    let add_f32_local =
                                        Arc::new(Mutex::new(ast::Value::new(self.add_f32_local)));
                                    body.push(ast::Value::new(ast::Instr::call(add_f32_local)));
                                }

                                if local.value_type == ast::ValueType::NumType(ast::NumType::I32) {
                                    let add_i32_local =
                                        Arc::new(Mutex::new(ast::Value::new(self.add_i32_local)));
                                    body.push(ast::Value::new(ast::Instr::call(add_i32_local)));
                                }

                                local_count += 1;
                            }
                        }

                        // Add values on the stack to satisfy the current function result
                        // type. Values don't need to be meaningful.
                        {
                            for result in &curr_func_type.results {
                                let instr = match result {
                                    ast::ValueType::NumType(ast::NumType::I32) => {
                                        ast::Instr::i32_const(667)
                                    }
                                    ast::ValueType::NumType(ast::NumType::I64) => {
                                        ast::Instr::i64_const(667)
                                    }
                                    ast::ValueType::NumType(ast::NumType::F32) => {
                                        ast::Instr::f32_const(667.0)
                                    }
                                    ast::ValueType::NumType(ast::NumType::F64) => {
                                        ast::Instr::f64_const(667.0)
                                    }
                                };
                                body.push(ast::Value::new(instr));
                            }
                        }

                        body.push(ast::Value::new(ast::Instr::Return));
                        body.push(ast::Value::new(ast::Instr::end));
                        ast::Value::new(body)
                    }
                };

                ctx.insert_node_before(ast::Instr::If(
                    ast::BlockType::Empty,
                    Arc::new(Mutex::new(consequent)),
                ));

                ctx.insert_node_before(ast::Instr::local_get(address_local));
            }
        }

        // After each call, check if we are unwinding the stack and need to continue
        // to do so. Unless we are in a function that is exported, ie the edge
        // of the module, in that case throw.
        if matches!(
            ctx.node.value,
            ast::Instr::call(_) | ast::Instr::call_indirect(_, _)
        ) {
            ctx.insert_node_after(ast::Instr::global_get(self.is_unwinding));

            // Insert if is_unwinding branch
            {
                let mut body = vec![];

                // create stack frame
                {
                    let func_locals = ctx.module.func_locals(curr_funcidx);
                    let locals = locals_flatten(func_locals);

                    let param_count = curr_func_type.params.len();

                    // In Wasm DWARF the offset is relative to the start of the
                    // code section.
                    // https://yurydelendik.github.io/webassembly-dwarf/#pc
                    // let code_offset = ctx.node.start_offset as i64
                    //     - ctx.module.get_code_section_start_offset().unwrap() as i64;
                    // body.push(ast::Value::new(ast::Instr::i32_const(code_offset as i64)));
                    // FIXME: we use the funcidx because the code offset isn't accurate
                    // or buggy.
                    body.push(ast::Value::new(ast::Instr::i32_const(curr_funcidx as i64)));
                    body.push(ast::Value::new(ast::Instr::i32_const(
                        (locals.len() + param_count) as i64,
                    )));

                    let start_frame = Arc::new(Mutex::new(ast::Value::new(self.start_frame)));
                    body.push(ast::Value::new(ast::Instr::call(start_frame)));

                    // TODO: for now we don't care about function arguments
                    // because seems that Rust doesn't really use them anyway.
                    for i in 0..param_count {
                        body.push(ast::Value::new(ast::Instr::i32_const(669 + i as i64)));

                        let add_i32_local =
                            Arc::new(Mutex::new(ast::Value::new(self.add_i32_local)));
                        body.push(ast::Value::new(ast::Instr::call(add_i32_local)));
                    }

                    // Collect the base/stack pointer, usually Rust stores it in
                    // the first few locals (so after the function params).
                    let mut local_count = curr_func_type.params.len() as u32;

                    for local in locals {
                        body.push(ast::Value::new(ast::Instr::local_get(local_count)));

                        if local.value_type == ast::ValueType::NumType(ast::NumType::I64) {
                            let add_i64_local =
                                Arc::new(Mutex::new(ast::Value::new(self.add_i64_local)));
                            body.push(ast::Value::new(ast::Instr::call(add_i64_local)));
                        }

                        if local.value_type == ast::ValueType::NumType(ast::NumType::F64) {
                            let add_f64_local =
                                Arc::new(Mutex::new(ast::Value::new(self.add_f64_local)));
                            body.push(ast::Value::new(ast::Instr::call(add_f64_local)));
                        }

                        if local.value_type == ast::ValueType::NumType(ast::NumType::F32) {
                            let add_f32_local =
                                Arc::new(Mutex::new(ast::Value::new(self.add_f32_local)));
                            body.push(ast::Value::new(ast::Instr::call(add_f32_local)));
                        }

                        if local.value_type == ast::ValueType::NumType(ast::NumType::I32) {
                            let add_i32_local =
                                Arc::new(Mutex::new(ast::Value::new(self.add_i32_local)));
                            body.push(ast::Value::new(ast::Instr::call(add_i32_local)));
                        }

                        local_count += 1;
                    }
                }

                // if we are back to the entrypoint...
                {
                    body.push(ast::Value::new(ast::Instr::global_get(self.entry_funcidx)));
                    body.push(ast::Value::new(ast::Instr::i32_const(curr_funcidx as i64)));
                    body.push(ast::Value::new(ast::Instr::i32_eq));

                    let mut if_body = vec![];

                    // We are at the edge of the module, stop unwinding the
                    // stack and trap.
                    let write_coredump = Arc::new(Mutex::new(ast::Value::new(self.write_coredump)));
                    if_body.push(ast::Value::new(ast::Instr::call(write_coredump)));
                    if_body.push(ast::Value::new(ast::Instr::unreachable));
                    if_body.push(ast::Value::new(ast::Instr::else_end));

                    // Add values on the stack to satisfy the current function result
                    // type. Values don't need to be meaningful.
                    {
                        for result in &curr_func_type.results {
                            let instr = match result {
                                ast::ValueType::NumType(ast::NumType::I32) => {
                                    ast::Instr::i32_const(667)
                                }
                                ast::ValueType::NumType(ast::NumType::I64) => {
                                    ast::Instr::i64_const(667)
                                }
                                ast::ValueType::NumType(ast::NumType::F32) => {
                                    ast::Instr::f32_const(667.0)
                                }
                                ast::ValueType::NumType(ast::NumType::F64) => {
                                    ast::Instr::f64_const(667.0)
                                }
                            };
                            if_body.push(ast::Value::new(instr));
                        }
                    }

                    if_body.push(ast::Value::new(ast::Instr::Return));
                    if_body.push(ast::Value::new(ast::Instr::end));

                    let if_body = ast::Value::new(if_body);
                    let if_node =
                        ast::Instr::If(ast::BlockType::Empty, Arc::new(Mutex::new(if_body)));
                    body.push(ast::Value::new(if_node));
                }

                body.push(ast::Value::new(ast::Instr::end));

                let body = ast::Value::new(body);
                let if_node = ast::Instr::If(ast::BlockType::Empty, Arc::new(Mutex::new(body)));
                ctx.insert_node_after(if_node);
            }
            return;
        }
    }
}
