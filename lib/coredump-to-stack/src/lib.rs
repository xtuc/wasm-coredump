use std::collections::HashMap;
use std::sync::Arc;

type BoxError = Box<dyn std::error::Error>;

pub struct Frame {
    pub name: String,
}

pub fn coredump_to_stack(coredump: &[u8], name_section: &[u8]) -> Result<Vec<Frame>, BoxError> {
    let coredump_wasm = wasm_parser::parse(&coredump)
        .map_err(|err| format!("failed to parse Wasm module: {}", err))?;
    let coredump_wasm = core_wasm_ast::traverse::WasmModule::new(Arc::new(coredump_wasm));

    // add names
    let func_names = {
        let mut map: HashMap<u32, String> = HashMap::new();

        let name_section = wasm_parser::parse_custom_section_name(name_section)?;
        let func_names = name_section.func_names.unwrap();
        let func_names = func_names.lock().unwrap();

        for (funcidx, name) in func_names.iter() {
            map.insert(*funcidx, name.clone());
        }

        map
    };

    let coredump = coredump_wasm.get_coredump()?;

    let mut frames = vec![];

    for frame in &coredump.stacks[0].frames {
        frames.push(Frame {
            name: func_names.get(&frame.funcidx).unwrap().to_owned(),
        })
    }

    Ok(frames)
}
