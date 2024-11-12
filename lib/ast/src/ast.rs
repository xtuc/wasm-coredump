use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

#[macro_export]
macro_rules! body {
    ($( $v:expr ),*) => {{
        let mut body = vec![];
        $(
            body.extend_from_slice(&$v);
        )*
        body.push(ast::Value::new(ast::Instr::end));
        ast::Value::new(body)
    }};
}

#[macro_export]
macro_rules! make_type {
    {} => {
        ast::Type {
            params: vec![],
            results: vec![],
        }
    };

    {($( $arg:ident ),*) -> ()} => {{
        use ast::NumType::*;

        let mut t = ast::Type {
            params: vec![],
            results: vec![],
        };
        $(
            t.params.push(ast::ValueType::NumType($arg));
        )*;

        t
    }};

    {($( $arg:ident ),*) -> $res:ident} => {{
        use ast::NumType::*;

        let mut t = ast::Type {
            params: vec![],
            results: vec![ast::ValueType::NumType($res)],
        };
        $(
            t.params.push(ast::ValueType::NumType($arg));
        )*;

        t
    }};
}
pub use body;
pub use make_type;

#[macro_export]
macro_rules! make_value {
    ($v:expr) => {
        Arc::new(Mutex::new(ast::Value::new($v)))
    };
}
pub use make_value;

#[derive(Debug, PartialEq, Clone)]
pub struct Value<T> {
    pub value: T,
    pub start_offset: usize,
    pub end_offset: usize,
}
impl<T> Value<T> {
    pub fn new(value: T) -> Self {
        Self {
            start_offset: 0,
            end_offset: 0,
            value,
        }
    }
}

pub type MutableValue<T> = Arc<Mutex<Value<T>>>;

#[derive(Debug, Clone)]
pub struct Memory {
    pub min: Value<u32>,
    pub max: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Code {
    pub size: Value<u32>,
    pub locals: Vec<CodeLocal>,
    pub body: MutableValue<Vec<Value<Instr>>>,
}

#[derive(Debug, Clone)]
pub struct CodeLocal {
    pub count: u32,
    pub value_type: ValueType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    NumType(NumType),
    // VectorType(),
    // RefType(),
}

#[derive(Debug, PartialEq, Clone)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, Clone)]
pub enum Reftype {
    Func,
    Extern,
}

#[derive(Debug, Clone)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub reftype: Reftype,
    pub limits: Limits,
}

#[derive(Debug, Clone)]
pub struct DataSegment {
    pub offset: Option<Value<Vec<Value<Instr>>>>,
    pub bytes: Vec<u8>,
    pub mode: DataSegmentMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataSegmentMode {
    Passive,
    Active,
}

impl DataSegment {
    pub fn compute_offset(&self) -> i64 {
        let expr = &self.offset.as_ref().unwrap().value;
        for instr in expr {
            if let Instr::i32_const(v) = instr.value {
                return v;
            }
        }

        unreachable!("malformed data expression: {:?}", expr)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BlockType {
    Empty,
    ValueType(ValueType),
    Typeidx(u32),
}

#[derive(Debug, Clone)]
pub enum Instr {
    unreachable,
    nop,

    call(MutableValue<u32>),
    call_indirect(u32, u32),

    drop,
    select,

    local_get(u32),
    local_set(u32),
    local_tee(u32),
    global_get(u32),
    global_set(u32),
    table_get(u32),
    table_set(u32),

    i32_load(MutableValue<u32>, u32),
    i64_load(MutableValue<u32>, u32),
    f32_load(MutableValue<u32>, u32),
    f64_load(MutableValue<u32>, u32),
    i32_load8_s(MutableValue<u32>, u32),
    i32_load8_u(MutableValue<u32>, u32),
    i32_load16_s(MutableValue<u32>, u32),
    i32_load16_u(MutableValue<u32>, u32),
    i64_load8_s(MutableValue<u32>, u32),
    i64_load8_u(MutableValue<u32>, u32),
    i64_load16_s(MutableValue<u32>, u32),
    i64_load16_u(MutableValue<u32>, u32),
    i64_load32_s(MutableValue<u32>, u32),
    i64_load32_u(MutableValue<u32>, u32),

    i32_store(MutableValue<u32>, u32),
    i64_store(MutableValue<u32>, u32),
    f32_store(MutableValue<u32>, u32),
    f64_store(MutableValue<u32>, u32),
    i32_store8(MutableValue<u32>, u32),
    i32_store16(MutableValue<u32>, u32),
    i64_store8(MutableValue<u32>, u32),
    i64_store16(MutableValue<u32>, u32),
    i64_store32(MutableValue<u32>, u32),

    memory_size(u8),
    memory_grow(u8),
    memory_copy(u8, u8),
    memory_fill(u8),

    br(u32),
    br_if(u32),
    br_table(Vec<u32>, u32),
    else_end,
    end,
    Return,
    Block(BlockType, MutableValue<Vec<Value<Instr>>>),
    Loop(BlockType, MutableValue<Vec<Value<Instr>>>),
    If(BlockType, MutableValue<Vec<Value<Instr>>>),

    i32_const(i64),
    i64_const(i64),
    f32_const(f32),
    f64_const(f64),
    i32_eqz,
    i32_eq,
    i32_ne,
    i32_lt_s,
    i32_lt_u,
    i32_gt_s,
    i32_gt_u,
    i32_le_s,
    i32_le_u,
    i32_ge_s,
    i32_ge_u,

    i64_eqz,
    i64_eq,
    i64_ne,
    i64_lt_s,
    i64_lt_u,
    i64_gt_s,
    i64_gt_u,
    i64_le_s,
    i64_le_u,
    i64_ge_s,
    i64_ge_u,

    f32_eq,
    f32_ne,
    f32_lt,
    f32_gt,
    f32_le,
    f32_ge,

    f64_eq,
    f64_ne,
    f64_lt,
    f64_gt,
    f64_le,
    f64_ge,

    i32_clz,
    i32_ctz,
    i32_popcnt,
    i32_add,
    i32_sub,
    i32_mul,
    i32_div_s,
    i32_div_u,
    i32_rem_s,
    i32_rem_u,
    i32_and,
    i32_or,
    i32_xor,
    i32_shl,
    i32_shr_s,
    i32_shr_u,
    i32_rotl,
    i32_rotr,

    i64_clz,
    i64_ctz,
    i64_popcnt,
    i64_add,
    i64_sub,
    i64_mul,
    i64_div_s,
    i64_div_u,
    i64_rem_s,
    i64_rem_u,
    i64_and,
    i64_or,
    i64_xor,
    i64_shl,
    i64_shr_s,
    i64_shr_u,
    i64_rotl,
    i64_rotr,

    f32_abs,
    f32_neg,
    f32_ceil,
    f32_floor,
    f32_trunc,
    f32_nearest,
    f32_sqrt,
    f32_add,
    f32_sub,
    f32_mul,
    f32_div,
    f32_min,
    f32_max,
    f32_copysign,

    f64_abs,
    f64_neg,
    f64_ceil,
    f64_floor,
    f64_trunc,
    f64_nearest,
    f64_sqrt,
    f64_add,
    f64_sub,
    f64_mul,
    f64_div,
    f64_min,
    f64_max,
    f64_copysign,

    i32_wrap_i64,
    i32_trunc_f32_s,
    i32_trunc_f32_u,
    i32_trunc_f64_s,
    i32_trunc_f64_u,
    i32_trunc_sat_f32_s,
    i32_trunc_sat_f32_u,
    i32_trunc_sat_f64_s,
    i32_trunc_sat_f64_u,
    i64_extend_i32_s,
    i64_extend_i32_u,
    i64_trunc_f32_s,
    i64_trunc_f32_u,
    i64_trunc_f64_s,
    i64_trunc_f64_u,
    i64_trunc_sat_f32_s,
    i64_trunc_sat_f32_u,
    i64_trunc_sat_f64_s,
    i64_trunc_sat_f64_u,
    f32_convert_i32_s,
    f32_convert_i32_u,
    f32_convert_i64_s,
    f32_convert_i64_u,
    f32_demote_f64,
    f64_convert_i32_s,
    f64_convert_i32_u,
    f64_convert_i64_s,
    f64_convert_i64_u,
    f64_promote_f32,

    i32_reinterpret_f32,
    i64_reinterpret_f64,
    f32_reinterpret_i32,
    f64_reinterpret_i64,

    i32_extend8_s,
    i32_extend16_s,
    i64_extend8_s,
    i64_extend16_s,
    i64_extend32_s,
}

pub type Expr = Value<Vec<Value<Instr>>>;

#[derive(Debug, Clone)]
pub enum Section {
    /// (Size, Section)
    Memory((Value<u32>, Vec<Memory>)),
    Data((Value<u32>, Arc<Mutex<Vec<DataSegment>>>)),
    Code((Value<u32>, MutableValue<Vec<Code>>)),
    Type((Value<u32>, Arc<Mutex<Vec<Type>>>)),
    Func((Value<u32>, Arc<Mutex<Vec<u32>>>)),
    Import((Value<u32>, Arc<Mutex<Vec<Import>>>)),
    Table((Value<u32>, Arc<Mutex<Vec<Table>>>)),
    Export((Value<u32>, Arc<Mutex<Vec<Export>>>)),
    Element((Value<u32>, Arc<Mutex<Vec<Element>>>)),
    Custom((Value<u32>, Arc<Mutex<CustomSection>>)),
    Global((Value<u32>, Arc<Mutex<Vec<Global>>>)),
    /// (Id, Size, Section)
    Unknown((u8, u32, Vec<u8>)),
}

impl Value<Section> {
    pub fn pos(&self) -> usize {
        use Section::*;

        match self.value {
            Type(_) => 1,
            Import(_) => 2,
            Func(_) => 3,
            Table(_) => 4,
            Memory(_) => 5,
            Global(_) => 6,
            Export(_) => 7,
            // Start(_) => 8,
            Element(_) => 9,
            Code(_) => 10,
            Data(_) => 11,
            Custom(_) | Unknown(_) => 99,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CustomSection {
    Unknown(String, Vec<u8>),
    Name(DebugNames),
    CoredumpCore(wasm_coredump_types::ProcessInfo),
    CoredumpCoreStack(wasm_coredump_types::CoreStack),
    BuildId(Vec<u8>),
}

#[derive(Debug)]
pub struct Module {
    pub sections: Arc<Mutex<Vec<Value<Section>>>>,
}

#[derive(Debug, Clone)]
pub struct Type {
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub import_type: ImportType,
}

#[derive(Debug, Clone)]
pub enum ImportType {
    Func(u32),
    Table(Table),
    Memory(Memory),
    Global(GlobalType),
}

#[derive(Debug, Clone)]
pub struct Export {
    pub name: String,
    pub descr: ExportDescr,
}

#[derive(Debug, Clone)]
pub enum ExportDescr {
    Func(Arc<Mutex<u32>>),
    Table(Arc<Mutex<u32>>),
    Mem(Arc<Mutex<u32>>),
    Global(Arc<Mutex<u32>>),
}

#[derive(Debug, Clone)]
pub enum Element {
    FuncActive(Expr, Arc<Mutex<Vec<u32>>>),
}

#[derive(Debug, Clone)]
pub struct Global {
    pub global_type: GlobalType,
    pub expr: Expr,
}

impl Global {
    pub fn compute_value(&self) -> i64 {
        let expr = &self.expr.value;
        if !self.global_type.mutable {
            for instr in expr {
                if let Instr::i32_const(v) = instr.value {
                    return v;
                }
            }
        }

        unreachable!("unsupported global expression: {:?}", expr)
    }
}

#[derive(Debug, Clone)]
pub struct GlobalType {
    pub valtype: ValueType,
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub struct DebugNames {
    pub module: Option<String>,
    pub func_names: Option<Arc<Mutex<HashMap<u32, String>>>>,
    pub func_local_names: Option<HashMap<u32, HashMap<u32, String>>>,
}
