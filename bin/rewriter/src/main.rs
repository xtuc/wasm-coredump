


use log::info;

use std::io::stdin;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::time::Instant;

mod rewriter;

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    env_logger::init();

    let mut input = Vec::new();
    stdin().read_to_end(&mut input)?;

    let now = Instant::now();
    let module = Arc::new(
        wasm_parser::parse(&input)
            .map_err(|err| format!("failed to parse Wasm module: {}", err))?,
    );
    let elapsed = now.elapsed();
    info!("decode: {:.2?}", elapsed);

    let now = Instant::now();
    rewriter::rewrite(Arc::clone(&module))?;
    let elapsed = now.elapsed();
    info!("transform: {:.2?}", elapsed);

    let now = Instant::now();
    input = wasm_printer::print(&module)?;
    let elapsed = now.elapsed();
    info!("print: {:.2?}", elapsed);

    stdout().write_all(&input)?;
    Ok(())
}
