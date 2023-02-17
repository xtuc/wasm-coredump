use core_wasm_ast as ast;
use log::{debug, warn};
use nom::bytes::complete::take;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

mod coredump;

type BoxError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub(crate) struct ParserError(String);

impl<I> nom::error::ParseError<I> for ParserError {
    fn from_error_kind(_input: I, kind: nom::error::ErrorKind) -> Self {
        ParserError(format!("error {:?}", kind))
    }
    fn append(_input: I, _kind: nom::error::ErrorKind, _other: Self) -> Self {
        todo!()
    }
    fn from_char(_input: I, _: char) -> Self {
        todo!()
    }
    fn or(self, _other: Self) -> Self {
        todo!()
    }
}

pub(crate) type IResult<I, O, E = ParserError> = Result<(I, O), nom::Err<E>>;

#[derive(Debug, Clone)]
pub(crate) struct InputContext<'a> {
    input: &'a [u8],
    offset: usize,
}

impl<'a> InputContext<'a> {
    fn read_u32(self) -> IResult<InputContext<'a>, u32> {
        let (input, bytes) = self.read_bytes(4usize)?;
        let value = u32::from_le_bytes(bytes.try_into().unwrap());
        Ok((input, value))
    }

    fn read_i32(self) -> IResult<InputContext<'a>, i32> {
        let (input, bytes) = self.read_bytes(4usize)?;
        let value = i32::from_le_bytes(bytes.try_into().unwrap());
        Ok((input, value))
    }

    fn read_i64(self) -> IResult<InputContext<'a>, i64> {
        let (input, bytes) = self.read_bytes(8usize)?;
        let value = i64::from_le_bytes(bytes.try_into().unwrap());
        Ok((input, value))
    }

    fn read_u8(self) -> IResult<InputContext<'a>, u8> {
        let (input, bytes) = self.read_bytes(1usize)?;
        let value = u8::from_le_bytes(bytes.try_into().unwrap());
        Ok((input, value))
    }

    fn read_leb128(self) -> IResult<InputContext<'a>, u32> {
        let mut input = self.input;
        let v = leb128::read::unsigned(&mut input).unwrap();

        // FIXME: find a better way to known how many bytes we just read
        let size = {
            let mut buffer = Vec::new();
            leb128::write::unsigned(&mut buffer, v).unwrap();
            buffer.len()
        };

        Ok((
            Self {
                input,
                offset: self.offset + size,
            },
            v as u32,
        ))
    }

    fn read_f32(self) -> IResult<InputContext<'a>, f32> {
        let (ctx, bytes) = self.read_bytes(4)?;
        let v = f32::from_le_bytes(bytes[0..4].try_into().unwrap());
        Ok((ctx, v))
    }

    fn read_f64(self) -> IResult<InputContext<'a>, f64> {
        let (ctx, bytes) = self.read_bytes(8)?;
        let v = f64::from_le_bytes(bytes[0..8].try_into().unwrap());
        Ok((ctx, v))
    }

    fn read_leb128_signed(self) -> IResult<InputContext<'a>, i64> {
        let mut input = self.input;
        let v = leb128::read::signed(&mut input).unwrap();

        // FIXME: find a better way to known how many bytes we just read
        let size = {
            let mut buffer = Vec::new();
            leb128::write::signed(&mut buffer, v).unwrap();
            buffer.len()
        };

        Ok((
            Self {
                input,
                offset: self.offset + size,
            },
            v,
        ))
    }

    fn read_bytes(self, n: usize) -> IResult<InputContext<'a>, &'a [u8]> {
        let (input, bytes) = take(n)(self.input)?;

        Ok((
            Self {
                input,
                offset: self.offset + n,
            },
            bytes,
        ))
    }

    fn peak_u8(self) -> IResult<InputContext<'a>, u8> {
        let (_input, byte) = take(1usize)(self.input)?;
        Ok((self, byte[0]))
    }
}

pub fn parse<'a>(input: &'a [u8]) -> Result<ast::Module, BoxError> {
    let input = InputContext { input, offset: 0 };
    match decode_module(input) {
        Ok((_, module)) => Ok(module),
        Err(err) => Err(format!("failed to decode: {}", err).into()),
    }
}

fn decode_module<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Module> {
    let (ctx, magic) = ctx.read_bytes(4)?;
    if magic != b"\0asm" {
        panic!("unsupported header: {:?}", magic)
    }
    assert_eq!(ctx.offset, 4);
    let (ctx, version) = ctx.read_u32()?;
    if version != 1 {
        panic!("unsupported version: {:?}", version)
    }
    assert_eq!(ctx.offset, 8);

    let mut ctx = ctx;

    let mut sections = vec![];
    loop {
        let start_offset = ctx.offset;
        let res = decode_section(ctx)?;
        ctx = res.0;
        let end_offset = ctx.offset;

        let value = ast::Value {
            start_offset,
            value: res.1,
            end_offset,
        };

        sections.push(value);

        if ctx.input.is_empty() {
            break;
        }
    }

    let module = ast::Module {
        sections: Arc::new(Mutex::new(sections)),
    };
    Ok((ctx, module))
}

fn decode_section_memory<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, Vec<ast::Memory>> {
    decode_vec(ctx, decode_memory)
}

fn decode_section_global<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, Vec<ast::Global>> {
    decode_vec(ctx, decode_global)
}

fn decode_section_export<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, Vec<ast::Export>> {
    decode_vec(ctx, decode_export)
}

fn decode_section_element<'a>(
    ctx: InputContext<'a>,
) -> IResult<InputContext<'a>, Vec<ast::Element>> {
    decode_vec(ctx, decode_element)
}

fn decode_section_data<'a>(
    ctx: InputContext<'a>,
) -> IResult<InputContext<'a>, Vec<ast::DataSegment>> {
    decode_vec(ctx, decode_data)
}

fn decode_section_type<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, Vec<ast::Type>> {
    decode_vec(ctx, decode_type)
}

fn decode_section_custom<'a>(
    ctx: InputContext<'a>,
) -> IResult<InputContext<'a>, ast::CustomSection> {
    let (ctx, name) = decode_name(ctx)?;
    log::debug!("parse custom section: {:?}", name);
    Ok(match name.as_str() {
        "name" => {
            let (ctx, content) = decode_section_custom_name(ctx)?;
            (ctx, ast::CustomSection::Name(content))
        }
        "core" => {
            let (ctx, content) = coredump::decode_process_info(ctx)?;
            (ctx, ast::CustomSection::CoredumpCore(content))
        }
        "corestack" => {
            let (ctx, content) = coredump::decode_core_stack(ctx)?;
            (ctx, ast::CustomSection::CoredumpCoreStack(content))
        }
        _ => {
            debug!("unknown custom section: {}", name);
            (
                ctx.clone(),
                ast::CustomSection::Unknown(name, ctx.input.to_vec()),
            )
        }
    })
}

fn decode_section_custom_name<'a>(
    ctx: InputContext<'a>,
) -> IResult<InputContext<'a>, ast::DebugNames> {
    let module = None;
    let mut func_names = None;
    let func_local_names = None;

    let mut ctx = ctx;

    loop {
        if ctx.input.is_empty() {
            break;
        }
        let ret = ctx.read_u8()?;
        ctx = ret.0;
        let subsection = ret.1;

        let ret = ctx.read_leb128()?;
        ctx = ret.0;
        let size = ret.1;

        debug!("parsing name subsection: {}", subsection);

        match subsection {
            // 0 => {
            //     let ret = decode_name(ctx)?;
            //     ctx = ret.0;
            //     module = ret.1;
            // }
            1 => {
                let ret = decode_namemap(ctx)?;
                ctx = ret.0;
                func_names = Some(Arc::new(Mutex::new(ret.1)));
            }
            // 2 => {
            //     let ret = ctx.read_leb128()?;
            //     ctx = ret.0;
            //     let n = ret.1;

            //     for i in 0..n {
            //         let ret = decode_namemap(ctx)?;
            //         ctx = ret.0;
            //         func_local_names.insert(i, ret.1);
            //     }
            // }
            _ => {
                warn!(
                    "ignoring custom name subsection: {} ({} byte(s))",
                    subsection, size
                );
                let ret = ctx.read_bytes(size as usize)?;
                ctx = ret.0;
            }
        }
    }

    let debug_names = ast::DebugNames {
        module,
        func_names,
        func_local_names,
    };
    Ok((ctx, debug_names))
}

fn decode_namemap<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, HashMap<u32, String>> {
    let mut map = HashMap::new();

    let (ctx, n) = ctx.read_leb128()?;
    let mut ctx = ctx;
    for _ in 0..n {
        let ret = ctx.read_leb128()?;
        ctx = ret.0;
        let funcidx = ret.1;

        let ret = decode_name(ctx)?;
        ctx = ret.0;
        let name = ret.1;

        map.insert(funcidx, name);
    }

    Ok((ctx, map))
}

fn decode_section_func<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, Vec<u32>> {
    decode_vec(ctx, |ctx| ctx.read_leb128())
}

fn decode_section_table<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, Vec<ast::Table>> {
    decode_vec(ctx, decode_table)
}

fn decode_section_import<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, Vec<ast::Import>> {
    decode_vec(ctx, decode_import)
}

fn decode_section_code<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, Vec<ast::Code>> {
    decode_vec(ctx, decode_code)
}

fn decode_reftype<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Reftype> {
    let (ctx, t) = ctx.read_u8()?;
    Ok((
        ctx,
        match t {
            0x70 => ast::Reftype::Func,
            0x6F => ast::Reftype::Extern,
            _ => unimplemented!("unsupported reftype: {}", t),
        },
    ))
}

fn decode_limits<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Limits> {
    let (ctx, t) = ctx.read_u8()?;
    Ok(match t {
        0x00 => {
            let (ctx, min) = ctx.read_leb128()?;
            (ctx, ast::Limits { min, max: None })
        }
        0x01 => {
            let (ctx, min) = ctx.read_leb128()?;
            let (ctx, max) = ctx.read_leb128()?;
            (
                ctx,
                ast::Limits {
                    min,
                    max: Some(max),
                },
            )
        }
        _ => unimplemented!("unsupported limit: {}", t),
    })
}

fn decode_table<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Table> {
    let (ctx, reftype) = decode_reftype(ctx)?;
    let (ctx, limits) = decode_limits(ctx)?;
    let table = ast::Table { reftype, limits };
    Ok((ctx, table))
}

fn decode_import<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Import> {
    let (ctx, module) = decode_name(ctx)?;
    let (ctx, name) = decode_name(ctx)?;

    let (ctx, descr_t) = ctx.read_u8()?;
    let (ctx, import_type) = match descr_t {
        0x00 => {
            let (ctx, typeidx) = ctx.read_leb128()?;
            (ctx, ast::ImportType::Func(typeidx))
        }
        0x03 => {
            let (ctx, globaltype) = decode_global_type(ctx)?;
            (ctx, ast::ImportType::Global(globaltype))
        }
        _ => unimplemented!("import description: {:x}", descr_t),
    };

    let import = ast::Import {
        module,
        name,
        import_type,
    };
    Ok((ctx, import))
}

pub(crate) fn decode_name<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, String> {
    let (ctx, bytes) = decode_vec(ctx, |ctx| ctx.read_u8())?;
    let v = String::from_utf8_lossy(&bytes).to_string();
    Ok((ctx, v))
}

fn decode_code<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Code> {
    let start_offset = ctx.offset;
    let (ctx, size) = ctx.read_leb128()?;
    let end_offset = ctx.offset;

    let (ctx, code_bytes) = ctx.read_bytes(size as usize)?;

    let code = {
        let ctx = InputContext {
            input: code_bytes,
            offset: ctx.offset,
        };

        let size = ast::Value {
            start_offset,
            value: size,
            end_offset,
        };
        let (ctx, locals) = decode_vec(ctx, decode_code_local)?;
        let (_ctx, body) = decode_expr(ctx, ast::Instr::end)?;
        let body = Arc::new(Mutex::new(body));

        // Bytes are split before, no need to propagate this context.
        ast::Code { size, locals, body }
    };

    Ok((ctx, code))
}

fn decode_instr<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Value<ast::Instr>> {
    let start_offset = ctx.offset;
    let (ctx, id) = ctx.read_u8()?;

    macro_rules! decode_instr {
        ($byte:expr, $instr:ident) => {
            if id == $byte {
                let end_offset = ctx.offset;
                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::$instr,
                    end_offset,
                };
                return Ok((ctx, value));
            }
        };
        ($byte:expr, $instr:ident(u8)) => {
            if id == $byte {
                let (ctx, arg0) = ctx.read_u8()?;
                let end_offset = ctx.offset;

                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::$instr(arg0),
                    end_offset,
                };
                return Ok((ctx, value));
            }
        };

        ($byte:expr, $instr:ident(f32)) => {
            if id == $byte {
                let (ctx, arg0) = ctx.read_f32()?;
                let end_offset = ctx.offset;

                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::$instr(arg0),
                    end_offset,
                };
                return Ok((ctx, value));
            }
        };

        ($byte:expr, $instr:ident(f64)) => {
            if id == $byte {
                let (ctx, arg0) = ctx.read_f64()?;
                let end_offset = ctx.offset;

                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::$instr(arg0),
                    end_offset,
                };
                return Ok((ctx, value));
            }
        };

        ($byte:expr, $instr:ident(i32)) => {
            if id == $byte {
                let (ctx, arg0) = ctx.read_leb128_signed()?;
                let end_offset = ctx.offset;

                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::$instr(arg0),
                    end_offset,
                };
                return Ok((ctx, value));
            }
        };

        ($byte:expr, $instr:ident(i64)) => {
            if id == $byte {
                let (ctx, arg0) = ctx.read_leb128_signed()?;
                let end_offset = ctx.offset;

                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::$instr(arg0),
                    end_offset,
                };
                return Ok((ctx, value));
            }
        };

        ($byte:expr, $instr:ident(u32)) => {
            if id == $byte {
                let (ctx, arg0) = ctx.read_leb128()?;
                let end_offset = ctx.offset;

                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::$instr(arg0),
                    end_offset,
                };
                return Ok((ctx, value));
            }
        };

        ($byte:expr, $instr:ident(MutableValue<u32>)) => {
            if id == $byte {
                let arg_start_offset = ctx.offset;
                let (ctx, arg0) = ctx.read_leb128()?;
                let end_offset = ctx.offset;

                let arg0 = ast::Value {
                    start_offset: arg_start_offset,
                    value: arg0,
                    end_offset,
                };

                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::$instr(Arc::new(Mutex::new(arg0))),
                    end_offset,
                };
                return Ok((ctx, value));
            }
        };

        ($byte:expr, $instr:ident(u32, u32)) => {
            if id == $byte {
                let (ctx, arg0) = ctx.read_leb128()?;
                let (ctx, arg1) = ctx.read_leb128()?;
                let end_offset = ctx.offset;

                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::$instr(arg0, arg1),
                    end_offset,
                };
                return Ok((ctx, value));
            }
        };

        ($byte:expr, $instr:ident(MutableValue<u32>, u32)) => {
            if id == $byte {
                let start_offset = ctx.offset;
                let (ctx, arg0) = ctx.read_leb128()?;
                let end_offset = ctx.offset;
                let arg0 = Arc::new(Mutex::new(ast::Value {
                    start_offset,
                    value: arg0,
                    end_offset,
                }));

                let (ctx, arg1) = ctx.read_leb128()?;
                let end_offset = ctx.offset;

                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::$instr(arg0, arg1),
                    end_offset,
                };
                return Ok((ctx, value));
            }
        };

        ($byte:expr, $instr:ident(Vec<u32>, u32)) => {
            if id == $byte {
                let (ctx, arg0) = decode_vec(ctx, |ctx| ctx.read_leb128())?;
                let (ctx, arg1) = ctx.read_leb128()?;
                let end_offset = ctx.offset;

                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::$instr(arg0, arg1),
                    end_offset,
                };
                return Ok((ctx, value));
            }
        };
    }

    decode_instr!(0x00, unreachable);
    decode_instr!(0x01, nop);

    if id == 0x02 {
        let (ctx, block_type) = decode_blocktype(ctx)?;
        let (ctx, body) = decode_expr(ctx, ast::Instr::end)?;
        let end_offset = ctx.offset;

        let value = ast::Value {
            start_offset,
            value: ast::Instr::Block(block_type, Arc::new(Mutex::new(body))),
            end_offset,
        };
        return Ok((ctx, value));
    }
    if id == 0x03 {
        let (ctx, block_type) = decode_blocktype(ctx)?;
        let (ctx, body) = decode_expr(ctx, ast::Instr::end)?;
        let end_offset = ctx.offset;

        let value = ast::Value {
            start_offset,
            value: ast::Instr::Loop(block_type, Arc::new(Mutex::new(body))),
            end_offset,
        };
        return Ok((ctx, value));
    }
    if id == 0x04 {
        let (ctx, block_type) = decode_blocktype(ctx)?;
        // let (ctx, consequent) = decode_expr(ctx, ast::Instr::else_end)?;
        // FIXME: support IfElse, If will contain both
        let (ctx, body) = decode_expr(ctx, ast::Instr::end)?;
        let end_offset = ctx.offset;

        let value = ast::Value {
            start_offset,
            value: ast::Instr::If(block_type, Arc::new(Mutex::new(body))),
            end_offset,
        };
        return Ok((ctx, value));
    }

    decode_instr!(0xc, br(u32));
    decode_instr!(0xd, br_if(u32));
    decode_instr!(0x0b, end);
    decode_instr!(0x05, else_end);
    decode_instr!(0x0e, br_table(Vec<u32>, u32));
    decode_instr!(0x0f, Return);
    decode_instr!(0x10, call(MutableValue<u32>));
    decode_instr!(0x11, call_indirect(u32, u32));

    decode_instr!(0x1a, drop);
    decode_instr!(0x1b, select);
    // decode_instr!(0x1c, select);

    decode_instr!(0x20, local_get(u32));
    decode_instr!(0x21, local_set(u32));
    decode_instr!(0x22, local_tee(u32));
    decode_instr!(0x23, global_get(u32));
    decode_instr!(0x24, global_set(u32));
    decode_instr!(0x25, table_get(u32));
    decode_instr!(0x26, table_set(u32));

    decode_instr!(0x28, i32_load(MutableValue<u32>, u32));
    decode_instr!(0x29, i64_load(MutableValue<u32>, u32));
    decode_instr!(0x2a, f32_load(MutableValue<u32>, u32));
    decode_instr!(0x2b, f64_load(MutableValue<u32>, u32));
    decode_instr!(0x2c, i32_load8_s(MutableValue<u32>, u32));
    decode_instr!(0x2d, i32_load8_u(MutableValue<u32>, u32));
    decode_instr!(0x2e, i32_load16_s(MutableValue<u32>, u32));
    decode_instr!(0x2f, i32_load16_u(MutableValue<u32>, u32));
    decode_instr!(0x30, i64_load8_s(MutableValue<u32>, u32));
    decode_instr!(0x31, i64_load8_u(MutableValue<u32>, u32));
    decode_instr!(0x32, i64_load16_s(MutableValue<u32>, u32));
    decode_instr!(0x33, i64_load16_u(MutableValue<u32>, u32));
    decode_instr!(0x34, i64_load32_s(MutableValue<u32>, u32));
    decode_instr!(0x35, i64_load32_u(MutableValue<u32>, u32));

    decode_instr!(0x36, i32_store(MutableValue<u32>, u32));
    decode_instr!(0x37, i64_store(MutableValue<u32>, u32));
    decode_instr!(0x38, f32_store(MutableValue<u32>, u32));
    decode_instr!(0x39, f64_store(MutableValue<u32>, u32));
    decode_instr!(0x3a, i32_store8(MutableValue<u32>, u32));
    decode_instr!(0x3b, i32_store16(MutableValue<u32>, u32));
    decode_instr!(0x3c, i64_store8(MutableValue<u32>, u32));
    decode_instr!(0x3d, i64_store16(MutableValue<u32>, u32));
    decode_instr!(0x3e, i64_store32(MutableValue<u32>, u32));

    decode_instr!(0x3f, memory_size(u8));
    decode_instr!(0x40, memory_grow(u8));

    decode_instr!(0x41, i32_const(i32));
    decode_instr!(0x42, i64_const(i64));
    decode_instr!(0x43, f32_const(f32));
    decode_instr!(0x44, f64_const(f64));

    decode_instr!(0x45, i32_eqz);
    decode_instr!(0x46, i32_eq);
    decode_instr!(0x47, i32_ne);
    decode_instr!(0x48, i32_lt_s);
    decode_instr!(0x49, i32_lt_u);
    decode_instr!(0x4a, i32_gt_s);
    decode_instr!(0x4b, i32_gt_u);
    decode_instr!(0x4c, i32_le_s);
    decode_instr!(0x4d, i32_le_u);
    decode_instr!(0x4e, i32_ge_s);
    decode_instr!(0x4f, i32_ge_u);

    decode_instr!(0x50, i64_eqz);
    decode_instr!(0x51, i64_eq);
    decode_instr!(0x52, i64_ne);
    decode_instr!(0x53, i64_lt_s);
    decode_instr!(0x54, i64_lt_u);
    decode_instr!(0x55, i64_gt_s);
    decode_instr!(0x56, i64_gt_u);
    decode_instr!(0x57, i64_le_s);
    decode_instr!(0x58, i64_le_u);
    decode_instr!(0x59, i64_ge_s);
    decode_instr!(0x5a, i64_ge_u);

    decode_instr!(0x5b, f32_eq);
    decode_instr!(0x5c, f32_ne);
    decode_instr!(0x5d, f32_lt);
    decode_instr!(0x5e, f32_gt);
    decode_instr!(0x5f, f32_le);
    decode_instr!(0x60, f32_ge);

    decode_instr!(0x61, f64_eq);
    decode_instr!(0x62, f64_ne);
    decode_instr!(0x63, f64_lt);
    decode_instr!(0x64, f64_gt);
    decode_instr!(0x65, f64_le);
    decode_instr!(0x66, f64_ge);

    decode_instr!(0x67, i32_clz);
    decode_instr!(0x68, i32_ctz);
    decode_instr!(0x69, i32_popcnt);
    decode_instr!(0x6a, i32_add);
    decode_instr!(0x6b, i32_sub);
    decode_instr!(0x6c, i32_mul);
    decode_instr!(0x6d, i32_div_s);
    decode_instr!(0x6e, i32_div_u);
    decode_instr!(0x6f, i32_rem_s);
    decode_instr!(0x70, i32_rem_u);
    decode_instr!(0x71, i32_and);
    decode_instr!(0x72, i32_or);
    decode_instr!(0x73, i32_xor);
    decode_instr!(0x74, i32_shl);
    decode_instr!(0x75, i32_shr_s);
    decode_instr!(0x76, i32_shr_u);
    decode_instr!(0x77, i32_rotl);
    decode_instr!(0x78, i32_rotr);

    decode_instr!(0x79, i64_clz);
    decode_instr!(0x7a, i64_ctz);
    decode_instr!(0x7b, i64_popcnt);
    decode_instr!(0x7c, i64_add);
    decode_instr!(0x7d, i64_sub);
    decode_instr!(0x7e, i64_mul);
    decode_instr!(0x7f, i64_div_s);
    decode_instr!(0x80, i64_div_u);
    decode_instr!(0x81, i64_rem_s);
    decode_instr!(0x82, i64_rem_u);
    decode_instr!(0x83, i64_and);
    decode_instr!(0x84, i64_or);
    decode_instr!(0x85, i64_xor);
    decode_instr!(0x86, i64_shl);
    decode_instr!(0x87, i64_shr_s);
    decode_instr!(0x88, i64_shr_u);
    decode_instr!(0x89, i64_rotl);
    decode_instr!(0x8a, i64_rotr);

    decode_instr!(0x8b, f32_abs);
    decode_instr!(0x8c, f32_neg);
    decode_instr!(0x8d, f32_ceil);
    decode_instr!(0x8e, f32_floor);
    decode_instr!(0x8f, f32_trunc);
    decode_instr!(0x90, f32_nearest);
    decode_instr!(0x91, f32_sqrt);
    decode_instr!(0x92, f32_add);
    decode_instr!(0x93, f32_sub);
    decode_instr!(0x94, f32_mul);
    decode_instr!(0x95, f32_div);
    decode_instr!(0x96, f32_min);
    decode_instr!(0x97, f32_max);
    decode_instr!(0x98, f32_copysign);

    decode_instr!(0x99, f64_abs);
    decode_instr!(0x9a, f64_neg);
    decode_instr!(0x9b, f64_ceil);
    decode_instr!(0x9c, f64_floor);
    decode_instr!(0x9d, f64_trunc);
    decode_instr!(0x9e, f64_nearest);
    decode_instr!(0x9f, f64_sqrt);
    decode_instr!(0xa0, f64_add);
    decode_instr!(0xa1, f64_sub);
    decode_instr!(0xa2, f64_mul);
    decode_instr!(0xa3, f64_div);
    decode_instr!(0xa4, f64_min);
    decode_instr!(0xa5, f64_max);
    decode_instr!(0xa6, f64_copysign);

    decode_instr!(0xa7, i32_wrap_i64);
    decode_instr!(0xa8, i32_trunc_f32_s);
    decode_instr!(0xa9, i32_trunc_f32_u);
    decode_instr!(0xaa, i32_trunc_f64_s);
    decode_instr!(0xab, i32_trunc_f64_u);
    decode_instr!(0xac, i64_extend_i32_s);
    decode_instr!(0xad, i64_extend_i32_u);
    decode_instr!(0xae, i64_trunc_f32_s);
    decode_instr!(0xaf, i64_trunc_f32_u);
    decode_instr!(0xb0, i64_trunc_f64_s);
    decode_instr!(0xb1, i64_trunc_f64_u);
    decode_instr!(0xb2, f32_convert_i32_s);
    decode_instr!(0xb3, f32_convert_i32_u);
    decode_instr!(0xb4, f32_convert_i64_s);
    decode_instr!(0xb5, f32_convert_i64_u);
    decode_instr!(0xb6, f32_demote_f64);
    decode_instr!(0xb7, f64_convert_i32_s);
    decode_instr!(0xb8, f64_convert_i32_u);
    decode_instr!(0xb9, f64_convert_i64_s);
    decode_instr!(0xba, f64_convert_i64_u);
    decode_instr!(0xbb, f64_promote_f32);

    decode_instr!(0xbc, i32_reinterpret_f32);
    decode_instr!(0xbd, i64_reinterpret_f64);
    decode_instr!(0xbe, f32_reinterpret_i32);
    decode_instr!(0xbf, f64_reinterpret_i64);

    decode_instr!(0xc0, i32_extend8_s);
    decode_instr!(0xc1, i32_extend16_s);
    decode_instr!(0xc2, i64_extend8_s);
    decode_instr!(0xc3, i64_extend16_s);
    decode_instr!(0xc4, i64_extend32_s);

    if id == 0xfc {
        let (ctx, b) = ctx.read_u8()?;
        let end_offset = ctx.offset;

        match b {
            10 => {
                let (ctx, imm0) = ctx.read_u8()?;
                let (ctx, imm1) = ctx.read_u8()?;
                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::memory_copy(imm0, imm1),
                    end_offset,
                };
                return Ok((ctx, value));
            }
            11 => {
                let (ctx, imm0) = ctx.read_u8()?;
                let value = ast::Value {
                    start_offset,
                    value: ast::Instr::memory_fill(imm0),
                    end_offset,
                };
                return Ok((ctx, value));
            }
            b => {
                unimplemented!("unknown 0xfc operation {}", b)
            }
        }
    }

    unimplemented!("unknown instruction: {:#x}", id);
}

fn decode_expr<'a>(
    ctx: InputContext<'a>,
    _end_instr: ast::Instr,
) -> IResult<InputContext<'a>, ast::Expr> {
    let start_offset = ctx.offset;

    let mut ctx = ctx;
    let mut vec = Vec::new();
    loop {
        let ret = decode_instr(ctx)?;
        ctx = ret.0;
        vec.push(ret.1.clone());
        if matches!(ret.1.value, ast::Instr::end) {
            break;
        }
    }

    let end_offset = ctx.offset;
    let value = ast::Value {
        start_offset,
        value: vec,
        end_offset,
    };
    Ok((ctx, value))
}

fn decode_code_local<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::CodeLocal> {
    let (ctx, count) = ctx.read_leb128()?;
    let (ctx, value_type) = decode_valtype(ctx)?;
    let code_local = ast::CodeLocal { count, value_type };
    Ok((ctx, code_local))
}

fn decode_valtype<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::ValueType> {
    let (ctx, id) = ctx.read_u8()?;
    Ok((
        ctx,
        match id {
            0x7F => ast::ValueType::NumType(ast::NumType::I32),
            0x7E => ast::ValueType::NumType(ast::NumType::I64),
            0x7D => ast::ValueType::NumType(ast::NumType::F32),
            0x7C => ast::ValueType::NumType(ast::NumType::F64),
            e => unimplemented!("unsupported type: {:x}", e),
        },
    ))
}

fn decode_type<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Type> {
    let (ctx, b) = ctx.read_u8()?;
    if b != 0x60 {
        panic!("unexpected type");
    }
    let (ctx, params) = decode_vec(ctx, decode_valtype)?;
    let (ctx, results) = decode_vec(ctx, decode_valtype)?;

    let t = ast::Type { params, results };
    Ok((ctx, t))
}

fn decode_data<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::DataSegment> {
    let (ctx, t) = ctx.read_leb128()?;

    match t {
        0 => {
            let (ctx, expr) = decode_expr(ctx, ast::Instr::end)?;
            let (ctx, bytes) = decode_vec(ctx, |ctx| ctx.read_u8())?;
            let data_segment = ast::DataSegment {
                mode: ast::DataSegmentMode::Active,
                offset: Some(expr),
                bytes,
            };
            Ok((ctx, data_segment))
        }
        1 => {
            let (ctx, bytes) = decode_vec(ctx, |ctx| ctx.read_u8())?;
            let data_segment = ast::DataSegment {
                mode: ast::DataSegmentMode::Passive,
                offset: None,
                bytes,
            };
            Ok((ctx, data_segment))
        }
        _ => unimplemented!("data segment of type: {}", t),
    }
}

fn decode_element<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Element> {
    let (ctx, t) = ctx.read_leb128()?;
    Ok(match t {
        0 => {
            let (ctx, expr) = decode_expr(ctx, ast::Instr::end)?;
            let (ctx, elements) = decode_vec(ctx, |ctx| ctx.read_leb128())?;
            (
                ctx,
                ast::Element::FuncActive(expr, Arc::new(Mutex::new(elements))),
            )
        }
        _ => unimplemented!("element segment of type: {}", t),
    })
}

fn decode_export<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Export> {
    let (ctx, name) = decode_name(ctx)?;
    let (ctx, descr) = decode_export_desc(ctx)?;
    let export = ast::Export { name, descr };
    Ok((ctx, export))
}

fn decode_export_desc<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::ExportDescr> {
    let (ctx, t) = ctx.read_u8()?;
    Ok(match t {
        0x00 => {
            let (ctx, idx) = ctx.read_leb128()?;
            (ctx, ast::ExportDescr::Func(Arc::new(Mutex::new(idx))))
        }
        0x01 => {
            let (ctx, idx) = ctx.read_leb128()?;
            (ctx, ast::ExportDescr::Table(Arc::new(Mutex::new(idx))))
        }
        0x02 => {
            let (ctx, idx) = ctx.read_leb128()?;
            (ctx, ast::ExportDescr::Mem(Arc::new(Mutex::new(idx))))
        }
        0x03 => {
            let (ctx, idx) = ctx.read_leb128()?;
            (ctx, ast::ExportDescr::Global(Arc::new(Mutex::new(idx))))
        }
        _ => unimplemented!("unsupported export descr"),
    })
}

fn decode_global<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Global> {
    let (ctx, global_type) = decode_global_type(ctx)?;
    let (ctx, expr) = decode_expr(ctx, ast::Instr::end)?;
    let global = ast::Global { global_type, expr };
    Ok((ctx, global))
}

fn decode_global_type<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::GlobalType> {
    let (ctx, valtype) = decode_valtype(ctx)?;
    let (ctx, mutable) = ctx.read_u8()?;

    let global_type = ast::GlobalType {
        valtype,
        mutable: mutable == 1,
    };

    Ok((ctx, global_type))
}

fn decode_memory<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Memory> {
    let (ctx, t) = ctx.read_u8()?;

    let start_offset = ctx.offset;
    let (ctx, min) = ctx.read_leb128()?;
    let end_offset = ctx.offset;
    let min = ast::Value {
        start_offset,
        value: min,
        end_offset,
    };

    let (ctx, mem) = match t {
        0 => (ctx, ast::Memory { min, max: None }),
        1 => {
            let (ctx, max) = ctx.read_leb128()?;
            (
                ctx,
                ast::Memory {
                    min,
                    max: Some(max),
                },
            )
        }
        e => unimplemented!("unsupported memory type: {}", e),
    };

    Ok((ctx, mem))
}

fn decode_section<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::Section> {
    let (ctx, id) = ctx.read_u8()?;

    let start_offset = ctx.offset;
    let (ctx, size) = ctx.read_leb128()?;
    let end_offset = ctx.offset;

    let section_size = ast::Value {
        start_offset,
        value: size,
        end_offset,
    };

    let offset = ctx.offset;
    debug!("decoding section {} ({} byte(s)) @ {}", id, size, offset);
    let (ctx, section_bytes) = ctx.read_bytes(size as usize)?;

    let section_bytes = InputContext {
        input: section_bytes,
        offset,
    };

    let section = match id {
        0 => match decode_section_custom(section_bytes.clone()) {
            Ok((_, res)) => ast::Section::Custom((section_size, Arc::new(Mutex::new(res)))),
            Err(err) => {
                warn!("failed to parse custom section: {}. Ignoring.", err);
                ast::Section::Unknown((id, size, section_bytes.input.to_vec()))
            }
        },
        1 => {
            let (_, res) = decode_section_type(section_bytes)?;
            ast::Section::Type((section_size, Arc::new(Mutex::new(res))))
        }
        2 => {
            let (_, res) = decode_section_import(section_bytes)?;
            ast::Section::Import((section_size, Arc::new(Mutex::new(res))))
        }
        3 => {
            let (_, res) = decode_section_func(section_bytes)?;
            ast::Section::Func((section_size, Arc::new(Mutex::new(res))))
        }
        4 => {
            let (_, res) = decode_section_table(section_bytes)?;
            ast::Section::Table((section_size, Arc::new(Mutex::new(res))))
        }
        5 => {
            let (_, res) = decode_section_memory(section_bytes)?;
            ast::Section::Memory((section_size, res))
        }
        6 => {
            let (_, res) = decode_section_global(section_bytes)?;
            ast::Section::Global((section_size, Arc::new(Mutex::new(res))))
        }
        7 => {
            let (_, res) = decode_section_export(section_bytes)?;
            ast::Section::Export((section_size, Arc::new(Mutex::new(res))))
        }
        9 => {
            let (_, res) = decode_section_element(section_bytes)?;
            ast::Section::Element((section_size, Arc::new(Mutex::new(res))))
        }
        10 => {
            let (_, res) = decode_section_code(section_bytes)?;
            let end_offset = ctx.offset;

            let value = ast::Value {
                start_offset,
                value: res,
                end_offset,
            };
            ast::Section::Code((section_size, Arc::new(Mutex::new(value))))
        }
        11 => {
            let (_, res) = decode_section_data(section_bytes)?;
            ast::Section::Data((section_size, Arc::new(Mutex::new(res))))
        }
        id => {
            warn!("unknown section with id {}", id);
            ast::Section::Unknown((id, size, section_bytes.input.to_vec()))
        }
    };
    Ok((ctx, section))
}

type DecodeItem<'a, T> = fn(InputContext<'a>) -> IResult<InputContext<'a>, T>;

pub(crate) fn decode_vec<'a, T>(
    ctx: InputContext<'a>,
    decode_item_fn: DecodeItem<'a, T>,
) -> IResult<InputContext<'a>, Vec<T>> {
    let (ctx, n) = ctx.read_leb128()?;

    let mut items: Vec<T> = Vec::new();
    let mut ctx = ctx;
    for _ in 0..n {
        let res = decode_item_fn(ctx)?;
        ctx = res.0;
        items.push(res.1);
    }

    Ok((ctx, items))
}

fn decode_blocktype<'a>(ctx: InputContext<'a>) -> IResult<InputContext<'a>, ast::BlockType> {
    let (ctx, next) = ctx.peak_u8()?;
    if next == 0x40 {
        // actually read what we peaked
        let (ctx, _) = ctx.read_u8()?;
        Ok((ctx, ast::BlockType::Empty))
    } else {
        if let Ok((ctx, valtype)) = decode_valtype(ctx.clone()) {
            Ok((ctx, ast::BlockType::ValueType(valtype)))
        } else {
            let (ctx, typeidx) = ctx.read_leb128()?;
            Ok((ctx, ast::BlockType::Typeidx(typeidx)))
        }
    }
}
