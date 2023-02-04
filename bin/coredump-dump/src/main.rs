use std::io;
use std::io::Read;
use std::sync::Arc;

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;

    let coredump_wasm = wasm_parser::parse(&input)
        .map_err(|err| format!("failed to parse Wasm module: {}", err))?;
    let coredump_wasm = core_wasm_ast::traverse::WasmModule::new(Arc::new(coredump_wasm));
    let coredump = coredump_wasm.get_coredump()?;

    let mut out = String::new();
    wasm_printer::wast::coredump::dump_coredump(&mut out, &coredump)?;

    println!("{}", out);
    Ok(())
}
