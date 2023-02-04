use crate::repl::Context;
use crate::BoxError;
use std::sync::Arc;
use wasmtime_wasi::sync::WasiCtxBuilder;

pub(crate) fn run<'a>(ctx: &mut Context<'a>) -> Result<(), BoxError> {
    let module = ctx.source.inner.clone();
    wasm_coredump_rewriter::rewrite(Arc::clone(&module))?;

    let engine = wasmtime::Engine::default();
    let mut linker = wasmtime::Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = wasmtime::Store::new(&engine, wasi);

    let bytes = wasm_printer::wasm::print(&module)?;
    let wasmtime_module = wasmtime::Module::new(&engine, bytes)?;

    linker.module(&mut store, "", &wasmtime_module)?;
    let instance = linker.instantiate(&mut store, &wasmtime_module)?;
    let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;

    match start.call(&mut store, ()) {
        Err(err) => {
            println!("program failed: {}", err);

            // Extract coredump
            let mem = instance
                .get_memory(&mut store, "memory")
                .ok_or("failed to get memory")?;
            let data = mem.data(&mut store);

            let coredump_wasm = wasm_parser::parse(&data)
                .map_err(|err| format!("failed to parse Wasm module: {}", err))?;
            let coredump_wasm = core_wasm_ast::traverse::WasmModule::new(Arc::new(coredump_wasm));

            ctx.coredump = Some(coredump_wasm.get_coredump()?);
            ctx.selected_thread = Some(0);
        }
        Ok(o) => {
            println!("program exited successfully: {:?}", o);
        }
    };

    Ok(())
}
