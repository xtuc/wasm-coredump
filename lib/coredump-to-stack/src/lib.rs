use rustc_demangle::demangle;
use std::collections::HashMap;
use std::sync::Arc;
use wasmgdb_ddbug_parser as ddbug_parser;

type BoxError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct Frame {
    pub name: String,
    pub location: FrameLocation,
}

#[derive(Debug)]
pub struct FrameLocation {
    pub file: String,
    pub line: u32,
}

impl FrameLocation {
    fn unknown() -> Self {
        Self {
            file: "unknown.rs".to_owned(),
            line: 0,
        }
    }
}

pub struct CoredumpToStack {
    coredump: core_wasm_ast::Module,

    /// Function names from the name custom section
    func_names: Option<HashMap<u32, String>>,

    /// Wasm module containing debugging information, not necessarily valid Wasm.
    debug_module: Option<Vec<u8>>,
}

impl CoredumpToStack {
    pub fn new(coredump_bytes: &[u8]) -> Result<Self, BoxError> {
        let coredump = wasm_parser::parse(coredump_bytes)
            .map_err(|err| format!("failed to parse Wasm module: {}", err))?;

        Ok(Self {
            coredump,
            func_names: None,
            debug_module: None,
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
            debug_module: None,
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
            debug_module: Some(bytes.to_owned()),
        })
    }

    pub fn stack(self) -> Result<Vec<Frame>, BoxError> {
        let coredump_wasm = core_wasm_ast::traverse::WasmModule::new(Arc::new(self.coredump));

        let func_names = self
            .func_names
            .ok_or::<BoxError>("missing name section".into())?;
        let coredump = coredump_wasm.get_coredump()?;

        let mut out_frames = vec![];

        let arena = ddbug_parser::Arena::new();
        #[allow(unused_assignments)]
        // file is used in the functions_by_linkage_name condition, we just
        // moved file here to increase its lifetime.
        let mut file = None;

        let functions_by_linkage_name = if let Some(debug_module) = &self.debug_module {
            let object = object::read::File::parse(debug_module.as_slice()).unwrap();
            file = Some(
                ddbug_parser::File::parse_object(
                    &object,
                    &object,
                    "module.wasm".to_owned(),
                    &arena,
                )
                .unwrap(),
            );
            let mut ddbug = ddbug_parser::FileHash::new(&file.as_ref().unwrap());

            let mut new = HashMap::new();

            // For Rust, demangle names in case the name section contains the names
            // unmangled.
            for (k, v) in ddbug.functions_by_linkage_name.iter() {
                new.insert(demangle(&k).to_string(), v.clone());
            }

            ddbug.functions_by_linkage_name.extend(new);
            Some(ddbug.functions_by_linkage_name)
        } else {
            // Without the Wasm module with debugging information we have little
            // information about the functions, only their linkage name.
            None
        };

        let mut frames = coredump.stacks[0].frames.clone();
        frames.reverse();

        for frame in frames {
            let linkage_name = func_names
                .get(&frame.funcidx)
                .unwrap_or(&format!("<unknown-func{}>", frame.funcidx))
                .to_owned();

            if let Some(functions_by_linkage_name) = &functions_by_linkage_name {
                if let Some(function) = functions_by_linkage_name.get(&linkage_name) {
                    let mut name = "".to_owned();

                    if let Some(ns) = function.namespace() {
                        name += &format!("{}::", ns.name().unwrap());
                    }
                    name += function.name().unwrap_or(&linkage_name);

                    let file = format!(
                        "{}/{}",
                        function.source().directory().unwrap_or(""),
                        function.source().file().unwrap_or("unknown.rs")
                    );

                    let location = FrameLocation {
                        file,
                        line: function.source().line(),
                    };

                    out_frames.push(Frame { name, location })
                } else {
                    let location = FrameLocation::unknown();
                    out_frames.push(Frame {
                        name: linkage_name,
                        location,
                    })
                }
            } else {
                let location = FrameLocation::unknown();
                out_frames.push(Frame {
                    name: linkage_name,
                    location,
                })
            }
        }

        Ok(out_frames)
    }
}
