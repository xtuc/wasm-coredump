//! Rust API for building [Wasm Coredump].
//!
//! # Examples
//!
//! ```
//! let mut coredump_builder = wasm_coredump_builder::CoredumpBuilder::new()
//!         .executable_name("/usr/bin/true.exe");
//!
//! {
//!     let mut thread_builder = wasm_coredump_builder::ThreadBuilder::new()
//!         .thread_name("main");
//!
//!     let coredump_frame = wasm_coredump_builder::FrameBuilder::new()
//!         .codeoffset(123)
//!         .funcidx(6)
//!         .build();
//!     thread_builder.add_frame(coredump_frame);
//!
//!     coredump_builder.add_thread(thread_builder.build());
//! }
//!
//! let coredump = coredump_builder.serialize().unwrap();
//! ```
//!
//! [Wasm Coredump]: https://github.com/WebAssembly/tool-conventions/blob/main/Coredump.md
#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::Infallible;

#[cfg(test)]
mod test;

#[derive(Default)]
/// Coredump stack frame builder
pub struct FrameBuilder {
    funcidx: u32,
    codeoffset: u32,
}

impl FrameBuilder {
    /// Create a new stack frame builder.
    pub fn new() -> Self {
        FrameBuilder::default()
    }

    /// WebAssembly function index in the module.
    pub fn funcidx(mut self, funcidx: u32) -> Self {
        self.funcidx = funcidx;
        self
    }

    /// Binary offset of the instruction, relative to the function's start.
    pub fn codeoffset(mut self, codeoffset: u32) -> Self {
        self.codeoffset = codeoffset;
        self
    }

    /// Build the coredump stack frame
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
/// Coredump builder
pub struct CoredumpBuilder {
    executable_name: String,
    threads: Vec<wasm_coredump_types::CoreStack>,
    memory: (u32, Option<u32>),
    data: Vec<u8>,
}

impl CoredumpBuilder {
    /// Create a new coredump builder
    pub fn new() -> Self {
        CoredumpBuilder::default()
    }

    /// Set the executable name
    pub fn executable_name(mut self, name: &str) -> Self {
        self.executable_name = name.to_owned();
        self
    }

    /// Set the complete process image
    /// Note that partial process image dump are not supported yet.
    pub fn data(mut self, bytes: &[u8]) -> Self {
        self.data = bytes.to_owned();
        self
    }

    /// Indicate the process memory usage
    pub fn memory(mut self, min: u32, max: Option<u32>) -> Self {
        self.memory = (min, max);
        self
    }

    /// Add a thread to the coredump
    pub fn add_thread(&mut self, thread: wasm_coredump_types::CoreStack) {
        self.threads.push(thread);
    }

    /// Build the coredump
    pub fn build(self) -> wasm_coredump_types::Coredump {
        wasm_coredump_types::Coredump {
            process_info: wasm_coredump_types::ProcessInfo {
                executable_name: self.executable_name,
            },
            stacks: self.threads,
            memory: vec![self.memory],
            data: self.data,
        }
    }

    /// Serialize the coredump to bytes, using the Wasm binary format.
    pub fn serialize(self) -> Result<Vec<u8>, Infallible> {
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

        // memory
        {
            let mut memories = wasm_encoder::MemorySection::new();
            memories.memory(wasm_encoder::MemoryType {
                minimum: self.memory.0 as u64,
                maximum: self.memory.1.map(|v| v as u64),
                memory64: false,
                shared: false,
            });

            module.section(&memories);
        }

        // data
        {
            let mut data = wasm_encoder::DataSection::new();
            let offset = wasm_encoder::ConstExpr::i32_const(0);
            data.active(0, &offset, self.data);

            module.section(&data);
        }

        Ok(module.finish())
    }
}

#[derive(Default)]
/// Coredump thread builder
pub struct ThreadBuilder {
    thread_name: String,
    frames: Vec<wasm_coredump_types::StackFrame>,
}

impl ThreadBuilder {
    /// Create a new thread builder
    pub fn new() -> Self {
        ThreadBuilder::default()
    }

    /// Set the thread name. By conventions "main" should be set for single
    /// threaded applications.
    pub fn thread_name(mut self, name: &str) -> Self {
        self.thread_name = name.to_owned();
        self
    }

    /// Add a stack frame to the thread
    pub fn add_frame(&mut self, frame: wasm_coredump_types::StackFrame) {
        self.frames.push(frame);
    }

    /// Build the thread
    pub fn build(self) -> wasm_coredump_types::CoreStack {
        wasm_coredump_types::CoreStack {
            frames: self.frames,
            thread_info: wasm_coredump_types::ThreadInfo {
                thread_name: self.thread_name,
            },
        }
    }
}
