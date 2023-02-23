use core_wasm_ast as ast;
use std::sync::Arc;
use std::sync::Mutex;

type BoxError = Box<dyn std::error::Error + Sync + Send>;

#[derive(Default)]
pub struct FrameBuilder {
    funcidx: Option<u32>,
}

impl FrameBuilder {
    pub fn new() -> Self {
        FrameBuilder::default()
    }

    pub fn funcidx(mut self, funcidx: u32) -> Self {
        self.funcidx = Some(funcidx);
        self
    }

    pub fn build(self) -> ast::coredump::StackFrame {
        ast::coredump::StackFrame {
            code_offset: self.funcidx.unwrap(),
            locals: vec![],
            stack: vec![],
        }
    }
}

#[derive(Default)]
pub struct CoredumpBuilder {
    executable_name: String,
    threads: Vec<ast::coredump::CoreStack>,
}

impl CoredumpBuilder {
    pub fn new() -> Self {
        CoredumpBuilder::default()
    }

    pub fn executable_name(mut self, name: &str) -> Self {
        self.executable_name = name.to_owned();
        self
    }

    pub fn add_thread(&mut self, thread: ast::coredump::CoreStack) {
        self.threads.push(thread);
    }

    pub fn serialize(self) -> Result<Vec<u8>, BoxError> {
        let module = ast::Module {
            sections: Arc::new(Mutex::new(vec![])),
        };
        let module = ast::traverse::WasmModule::new(Arc::new(module));

        // Core
        {
            // size will be calcuated during printing
            let size = ast::Value::new(0);
            let section = ast::Section::Custom((
                size,
                Arc::new(Mutex::new(ast::CustomSection::CoredumpCore(
                    ast::coredump::ProcessInfo {
                        executable_name: self.executable_name,
                    },
                ))),
            ));
            module.add_section(section);
        }

        // corestack
        for thread in self.threads {
            // size will be calcuated during printing
            let size = ast::Value::new(0);
            let section = ast::Section::Custom((
                size,
                Arc::new(Mutex::new(ast::CustomSection::CoredumpCoreStack(thread))),
            ));
            module.add_section(section);
        }

        wasm_printer::wasm::print(&module.inner)
    }
}

#[derive(Default)]
pub struct ThreadBuilder {
    thread_name: String,
    frames: Vec<ast::coredump::StackFrame>,
}

impl ThreadBuilder {
    pub fn new() -> Self {
        ThreadBuilder::default()
    }

    pub fn thread_name(mut self, name: &str) -> Self {
        self.thread_name = name.to_owned();
        self
    }

    pub fn add_frame(&mut self, frame: ast::coredump::StackFrame) {
        self.frames.push(frame);
    }

    pub fn build(self) -> ast::coredump::CoreStack {
        ast::coredump::CoreStack {
            frames: self.frames,
            thread_info: ast::coredump::ThreadInfo {
                thread_name: self.thread_name,
            },
        }
    }
}
