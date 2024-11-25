use clap::Parser;
use log::info;
use std::io::stdin;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::time::Instant;

mod rewriter;
mod runtime;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    /// Wraps each memory operation.
    /// This will likely reduce significantly your program's performance.
    check_memory_operations: bool,

    #[arg(long)]
    /// Enable debugging, mostly useful for developing this tooling.
    debug: bool,

    #[arg(long)]
    /// Enable WASI support.
    wasi: bool,
}

type BoxError = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxError> {
    let args = Args::parse();

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

    let opts = rewriter::RewritingOpts {
        check_memory_operations: args.check_memory_operations,
        debug: args.debug,
        wasi: args.wasi,
    };
    rewriter::rewrite(Arc::clone(&module), opts)?;
    let elapsed = now.elapsed();
    info!("transform: {:.2?}", elapsed);

    let now = Instant::now();
    input = wasm_printer::wasm::print(&module)
        .map_err(|err| format!("failed to print Wasm module: {}", err))?;
    let elapsed = now.elapsed();
    info!("print: {:.2?}", elapsed);

    stdout().write_all(&input)?;
    Ok(())
}
