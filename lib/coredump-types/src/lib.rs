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
    pub funcidx: u32,
    pub codeoffset: u32,
    pub locals: Vec<Value>,
    pub stack: Vec<Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Missing,
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Value {
    pub fn as_i32(&self) -> i32 {
        match self {
            Value::I32(v) => *v,
            _ => unreachable!(),
        }
    }
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
    pub memory: Vec<(u32, Option<u32>)>,
    pub data: Vec<u8>,
}
