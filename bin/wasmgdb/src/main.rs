use clap::Parser;
use rustc_demangle::demangle;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use wasmgdb_ddbug_parser as ddbug_parser;

mod commands;
mod context;
mod memory;
mod repl;

pub(crate) use context::Context;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// Source WebAssembly module
    source: String,
    /// Coredump
    coredump: Option<String>,
}

pub(crate) type BoxError = Box<dyn std::error::Error>;

pub fn main() -> Result<(), BoxError> {
    env_logger::init();

    let args = Args::parse();
    let source_filename = args.source;

    let coredump = if let Some(coredump_filename) = args.coredump {
        let mut coredump = Vec::new();
        {
            let mut file = File::open(coredump_filename).expect("File not found");
            file.read_to_end(&mut coredump)
                .expect("Error while reading file");
        }

        let coredump_wasm = wasm_parser::parse(&coredump)
            .map_err(|err| format!("failed to parse Wasm module: {}", err))?;
        let coredump_wasm = core_wasm_ast::traverse::WasmModule::new(Arc::new(coredump_wasm));
        let coredump = coredump_wasm.get_coredump()?;
        Some(coredump)
    } else {
        None
    };

    let ctx = ddbug_parser::File::parse(source_filename.clone()).unwrap();
    let mut ddbug = ddbug_parser::FileHash::new(ctx.file());
    let mut new = HashMap::new();

    // For Rust, demangle names in case the name section contains the names
    // unmangled.
    for (k, v) in ddbug.functions_by_linkage_name.iter() {
        new.insert(demangle(&k).to_string(), v.clone());
    }

    ddbug.functions_by_linkage_name.extend(new);

    let mut source = Vec::new();
    {
        let mut file = File::open(&source_filename).expect("File not found");
        file.read_to_end(&mut source)
            .expect("Error while reading file");
    }

    let addr2line = wasm_tools::addr2line::Addr2lineModules::parse(&source).unwrap();

    let source = wasm_parser::parse(&source)
        .map_err(|err| format!("failed to parse Wasm module: {}", err))?;
    let source = core_wasm_ast::traverse::WasmModule::new(Arc::new(source));

    let ctx = Context {
        ddbug,
        coredump: RefCell::new(coredump),
        source,
        addr2line: RefCell::new(addr2line),
        variables: RefCell::new(HashMap::new()),

        selected_frame: RefCell::new(None),
        selected_thread: RefCell::new(Some(0)), // auto select the first thread

        break_points: Mutex::new(HashSet::new()),
    };

    repl::repl(&ctx)
}
