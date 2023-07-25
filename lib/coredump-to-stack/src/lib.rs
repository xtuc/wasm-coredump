use std::collections::HashMap;
use std::sync::Arc;

type BoxError = Box<dyn std::error::Error>;

pub struct Frame {
    pub name: String,
}

pub struct CoredumpToStack {
    coredump: core_wasm_ast::Module,

    /// Function names from the name custom section
    func_names: Option<HashMap<u32, String>>,
}

impl CoredumpToStack {
    pub fn new(coredump_bytes: &[u8]) -> Result<Self, BoxError> {
        let coredump = wasm_parser::parse(coredump_bytes)
            .map_err(|err| format!("failed to parse Wasm module: {}", err))?;

        Ok(Self {
            coredump,
            func_names: None,
        })
    }

    pub fn with_name_section(self, bytes: &[u8]) -> Result<Self, BoxError> {
        let name_section = wasm_parser::parse_custom_section_name(bytes)?;
        let func_names = name_section
            .func_names
            .ok_or::<BoxError>("missing function names in name section".into())?;
        let func_names = func_names.lock().unwrap();

        Ok(Self {
            coredump: self.coredump,
            func_names: Some(func_names.clone()),
        })
    }

    pub fn with_debug_module(self, bytes: &[u8]) -> Result<Self, BoxError> {
        let module = wasm_parser::parse(&bytes)
            .map_err(|err| format!("failed to parse Wasm module: {}", err))?;
        let module = core_wasm_ast::traverse::WasmModule::new(Arc::new(module));

        let func_names = module.func_names.lock().unwrap();

        Ok(Self {
            coredump: self.coredump,
            func_names: Some(func_names.clone()),
        })
    }

    pub fn stack(self) -> Result<Vec<Frame>, BoxError> {
        let coredump_wasm = core_wasm_ast::traverse::WasmModule::new(Arc::new(self.coredump));

        let func_names = self
            .func_names
            .ok_or::<BoxError>("missing name section".into())?;
        let coredump = coredump_wasm.get_coredump()?;

        let mut frames = vec![];

        for frame in &coredump.stacks[0].frames {
            frames.push(Frame {
                name: func_names.get(&frame.funcidx).unwrap().to_owned(),
            })
        }

        Ok(frames)
    }
}
