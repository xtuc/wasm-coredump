type BoxError = Box<dyn std::error::Error + Sync + Send>;

#[derive(Default)]
pub struct FrameBuilder {
    funcidx: u32,
    codeoffset: u32,
}

impl FrameBuilder {
    pub fn new() -> Self {
        FrameBuilder::default()
    }

    pub fn funcidx(mut self, funcidx: u32) -> Self {
        self.funcidx = funcidx;
        self
    }

    pub fn codeoffset(mut self, codeoffset: u32) -> Self {
        self.codeoffset = codeoffset;
        self
    }

    pub fn build(self) -> wasm_coredump_types::StackFrame {
        wasm_coredump_types::StackFrame {
            funcidx: self.funcidx,
            codeoffset: self.codeoffset,
            locals: vec![],
            stack: vec![],
        }
    }
}

#[derive(Default)]
pub struct CoredumpBuilder {
    executable_name: String,
    threads: Vec<wasm_coredump_types::CoreStack>,
}

impl CoredumpBuilder {
    pub fn new() -> Self {
        CoredumpBuilder::default()
    }

    pub fn executable_name(mut self, name: &str) -> Self {
        self.executable_name = name.to_owned();
        self
    }

    pub fn add_thread(&mut self, thread: wasm_coredump_types::CoreStack) {
        self.threads.push(thread);
    }

    pub fn serialize(self) -> Result<Vec<u8>, BoxError> {
        let mut module = wasm_encoder::Module::new();

        // core
        {
            let mut data = vec![];
            let process_info = wasm_coredump_types::ProcessInfo {
                executable_name: self.executable_name,
            };
            wasm_coredump_encoder::encode_coredump_process(&mut data, &process_info)?;

            module.section(&wasm_encoder::CustomSection {
                name: "core",
                data: &data,
            });
        }

        // corestack
        for thread in self.threads {
            let mut data = vec![];
            wasm_coredump_encoder::encode_coredump_stack(&mut data, &thread)?;

            module.section(&wasm_encoder::CustomSection {
                name: "corestack",
                data: &data,
            });
        }

        Ok(module.finish())
    }
}

#[derive(Default)]
pub struct ThreadBuilder {
    thread_name: String,
    frames: Vec<wasm_coredump_types::StackFrame>,
}

impl ThreadBuilder {
    pub fn new() -> Self {
        ThreadBuilder::default()
    }

    pub fn thread_name(mut self, name: &str) -> Self {
        self.thread_name = name.to_owned();
        self
    }

    pub fn add_frame(&mut self, frame: wasm_coredump_types::StackFrame) {
        self.frames.push(frame);
    }

    pub fn build(self) -> wasm_coredump_types::CoreStack {
        wasm_coredump_types::CoreStack {
            frames: self.frames,
            thread_info: wasm_coredump_types::ThreadInfo {
                thread_name: self.thread_name,
            },
        }
    }
}
