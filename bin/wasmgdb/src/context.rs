use crate::BoxError;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use wasmgdb_ddbug_parser as ddbug_parser;

pub(crate) struct Context<'src> {
    pub(crate) selected_frame: RefCell<Option<wasm_coredump_types::StackFrame>>,
    pub(crate) selected_thread: RefCell<Option<usize>>,

    /// Variables present in the selected scope
    pub(crate) variables: RefCell<HashMap<String, ddbug_parser::Parameter<'src>>>,

    pub(crate) coredump: RefCell<Option<wasm_coredump_types::Coredump>>,

    /// DWARF informations
    pub(crate) ddbug: ddbug_parser::FileHash<'src>,

    /// Source Wasm module
    pub(crate) source: core_wasm_ast::traverse::WasmModule,

    pub(crate) break_points: Mutex<HashSet<u32>>,

    pub(crate) addr2line: RefCell<wasm_tools::addr2line::Addr2lineModules<'src>>,
}

impl<'a> Context<'a> {
    pub(crate) fn coredump(&self) -> Result<wasm_coredump_types::Coredump, BoxError> {
        self.coredump
            .borrow()
            .as_ref()
            .map(|c| c.clone())
            .ok_or("No coredump present".into())
    }

    pub(crate) fn thread(&self) -> Result<wasm_coredump_types::CoreStack, BoxError> {
        let coredump = self.coredump()?;

        self.selected_thread
            .borrow()
            .map(|idx| coredump.stacks[idx].clone())
            .ok_or("No frame selected".into())
    }
}
