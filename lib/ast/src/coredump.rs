#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub executable_name: String,
}

#[derive(Debug, Clone)]
pub struct ThreadInfo {
    pub thread_name: String,
}

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub code_offset: u32,
    pub locals: Vec<u32>,
    pub stack: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct CoreStack {
    pub thread_info: ThreadInfo,
    pub frames: Vec<StackFrame>,
}

#[derive(Debug, Clone)]
pub struct Coredump {
    pub process_info: ProcessInfo,
    pub stacks: Vec<CoreStack>,
    pub data: Vec<u8>,
}
