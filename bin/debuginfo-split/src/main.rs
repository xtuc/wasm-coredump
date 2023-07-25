use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

type BoxError = Box<dyn std::error::Error>;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// Source WebAssembly module
    source: String,
}

fn main() -> Result<(), BoxError> {
    let args = Args::parse();

    let mut input = Vec::new();
    {
        let mut file = File::open(&args.source).expect("File not found");
        file.read_to_end(&mut input)
            .expect("Error while reading file");
    }

    let input = wasm_parser::parse(&input)
        .map_err(|err| format!("failed to parse Wasm module: {}", err))?;
    let wasm = core_wasm_ast::traverse::WasmModule::new(Arc::new(input));

    let debug_wasm = core_wasm_ast::traverse::WasmModule::new(Arc::new(core_wasm_ast::Module {
        sections: Arc::new(Mutex::new(vec![])),
    }));

    let mut custom_sections_to_remove = vec![];

    for section in wasm.get_custom_sections() {
        match section {
            core_wasm_ast::CustomSection::Unknown(name, _) => {
                custom_sections_to_remove.push(name.clone());
            }
            core_wasm_ast::CustomSection::Name(n) => {
                debug_wasm.add_custom_section(core_wasm_ast::CustomSection::Name(n.clone()));

                custom_sections_to_remove.push("name".to_owned());
            }
            _ => {}
        }
    }

    println!("removing custom sections: {custom_sections_to_remove:?}");
    for section in &custom_sections_to_remove {
        wasm.remove_custom_section(section);
    }

    let build_id = Uuid::new_v4();
    wasm.set_build_id(build_id.as_bytes());

    // FIXME add debug id to correlate debugging symbols with binary

    // override input with stripped down version
    {
        let bytes = wasm_printer::wasm::print(&wasm.into_inner()).unwrap();
        let mut file = File::create(&args.source)?;
        file.write_all(&bytes)?;
    }

    // write debug wasm
    {
        let filename = format!("debug-{}.wasm", build_id);
        let mut file = File::create(&filename)?;

        let bytes = wasm_printer::wasm::print(&debug_wasm.into_inner()).unwrap();
        file.write_all(&bytes)?;

        println!("Wrote debugging infos {}", filename);
    }

    Ok(())
}
