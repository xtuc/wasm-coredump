use byteorder::{ByteOrder, LittleEndian};
use core_wasm_ast as ast;
use log::warn;
use std::io::Write;

type BoxError = Box<dyn std::error::Error + Sync + Send>;

pub fn print(module: &ast::Module) -> Result<Vec<u8>, BoxError> {
    let mut buffer = vec![];

    buffer.write(b"\0asm")?;
    buffer.write(&1u32.to_le_bytes())?;

    for section in module.sections.lock().unwrap().iter() {
        write_section(&mut buffer, &section.value)?;
    }

    Ok(buffer)
}

macro_rules! write_section {
    ($b:expr, $o:expr, $id:expr, $write_fn:expr) => {
        $b.push($id); // section id

        let before_offset = $b.len();
        $b.push(0x0); // section bytes

        $write_fn($b, &$o)?;

        let after_offset = $b.len();

        // section fixup
        let section_len = after_offset - before_offset - 1;

        // section length - fixup
        write_unsigned_leb128_at_offset($b, before_offset, section_len);
    };
}

fn write_section(buffer: &mut Vec<u8>, section: &ast::Section) -> Result<(), BoxError> {
    match section {
        ast::Section::Import((_size, content)) => {
            write_section!(buffer, content.lock().unwrap(), 2, write_section_import);
            Ok(())
        }
        ast::Section::Table((_size, content)) => {
            write_section!(buffer, content.lock().unwrap(), 4, write_section_table);
            Ok(())
        }
        ast::Section::Memory((_size, content)) => {
            write_section!(buffer, content, 5, write_section_memory);
            Ok(())
        }
        ast::Section::Code((_size, content)) => {
            write_section!(buffer, content.lock().unwrap(), 10, write_section_code);
            Ok(())
        }
        ast::Section::Type((_size, content)) => {
            write_section!(buffer, content.lock().unwrap(), 1, write_section_type);
            Ok(())
        }
        ast::Section::Element((_size, content)) => {
            write_section!(buffer, content.lock().unwrap(), 9, write_section_element);
            Ok(())
        }
        ast::Section::Custom((_size, content)) => {
            write_section!(buffer, content.lock().unwrap(), 0, write_section_custom);
            Ok(())
        }
        ast::Section::Unknown((id, size, content)) => {
            buffer.push(*id);
            write_unsigned_leb128(buffer, *size as u64);
            buffer.write(content)?;
            Ok(())
        }
        ast::Section::Data((_size, content)) => {
            write_section!(buffer, content.lock().unwrap(), 11, write_section_data);
            Ok(())
        }
        ast::Section::Func((_size, content)) => {
            write_section!(buffer, content.lock().unwrap(), 3, write_section_func);
            Ok(())
        }
        ast::Section::Export((_size, content)) => {
            write_section!(buffer, content.lock().unwrap(), 7, write_section_export);
            Ok(())
        }
        ast::Section::Global((_size, content)) => {
            write_section!(buffer, content.lock().unwrap(), 6, write_section_global);
            Ok(())
        }
    }
}

fn write_section_import(buffer: &mut Vec<u8>, content: &Vec<ast::Import>) -> Result<(), BoxError> {
    write_vec_len(buffer, content); // vec length

    for import in content {
        write_utf8(buffer, &import.module);
        write_utf8(buffer, &import.name);
        match &import.import_type {
            ast::ImportType::Func(funcidx) => {
                buffer.push(0x0);
                write_unsigned_leb128(buffer, *funcidx as u64);
            }

            ast::ImportType::Table(table) => {
                buffer.push(0x1);
                write_table(buffer, table)?;
            }

            ast::ImportType::Memory(mem) => {
                buffer.push(0x2);
                write_memory(buffer, mem)?;
            }

            ast::ImportType::Global(globaltype) => {
                buffer.push(0x3);

                write_value_type(buffer, &globaltype.valtype);
                if globaltype.mutable {
                    buffer.push(0x01);
                } else {
                    buffer.push(0x00);
                }
            }
        }
    }

    Ok(())
}

fn write_section_table(buffer: &mut Vec<u8>, content: &Vec<ast::Table>) -> Result<(), BoxError> {
    write_vec_len(buffer, content); // vec length

    for table in content {
        write_table(buffer, table)?;
    }

    Ok(())
}

fn write_table(buffer: &mut Vec<u8>, table: &ast::Table) -> Result<(), BoxError> {
    write_reftype(buffer, &table.reftype);
    write_limits(buffer, &table.limits);
    Ok(())
}

fn write_limits(buffer: &mut Vec<u8>, limits: &ast::Limits) {
    if let Some(max) = limits.max {
        buffer.push(0x01);
        write_unsigned_leb128(buffer, limits.min as u64);
        write_unsigned_leb128(buffer, max as u64);
    } else {
        buffer.push(0x00);
        write_unsigned_leb128(buffer, limits.min as u64);
    }
}

fn write_reftype(buffer: &mut Vec<u8>, typeref: &ast::Reftype) {
    let b = match typeref {
        ast::Reftype::Func => 0x70,
        ast::Reftype::Extern => 0x6F,
    };
    buffer.push(b);
}

pub(crate) fn write_utf8(buffer: &mut Vec<u8>, v: &str) {
    let bytes = v.as_bytes().to_vec();
    write_vec_len(buffer, &bytes);
    buffer.write_all(&bytes).unwrap();
}

fn write_section_memory(buffer: &mut Vec<u8>, content: &Vec<ast::Memory>) -> Result<(), BoxError> {
    write_vec_len(buffer, content); // vec length

    for mem in content {
        write_memory(buffer, mem)?;
    }

    Ok(())
}

fn write_memory(buffer: &mut Vec<u8>, mem: &ast::Memory) -> Result<(), BoxError> {
    if let Some(max) = mem.max {
        buffer.push(0x1);
        write_unsigned_leb128(buffer, mem.min.value as u64);
        write_unsigned_leb128(buffer, max as u64);
    } else {
        buffer.push(0x0);
        write_unsigned_leb128(buffer, mem.min.value as u64);
    }

    Ok(())
}

fn write_section_global(buffer: &mut Vec<u8>, content: &Vec<ast::Global>) -> Result<(), BoxError> {
    write_vec_len(buffer, &content); // vec length

    for global in content {
        write_value_type(buffer, &global.global_type.valtype);
        if global.global_type.mutable {
            buffer.push(0x01);
        } else {
            buffer.push(0x00);
        }

        write_code_expr(buffer, &global.expr.value);
    }

    Ok(())
}

fn write_section_export(buffer: &mut Vec<u8>, content: &Vec<ast::Export>) -> Result<(), BoxError> {
    write_vec_len(buffer, &content); // vec length

    for export in content {
        write_utf8(buffer, &export.name);
        match &export.descr {
            ast::ExportDescr::Func(idx) => {
                buffer.push(0x00);
                write_unsigned_leb128(buffer, *idx.lock().unwrap() as u64);
            }
            ast::ExportDescr::Table(idx) => {
                buffer.push(0x01);
                write_unsigned_leb128(buffer, *idx.lock().unwrap() as u64);
            }
            ast::ExportDescr::Mem(idx) => {
                buffer.push(0x02);
                write_unsigned_leb128(buffer, *idx.lock().unwrap() as u64);
            }
            ast::ExportDescr::Global(idx) => {
                buffer.push(0x03);
                write_unsigned_leb128(buffer, *idx.lock().unwrap() as u64);
            }
        };
    }

    Ok(())
}

fn write_section_func(buffer: &mut Vec<u8>, content: &Vec<u32>) -> Result<(), BoxError> {
    write_vec_len(buffer, &content); // vec length

    for idx in content {
        write_unsigned_leb128(buffer, *idx as u64);
    }

    Ok(())
}

fn write_section_data(
    buffer: &mut Vec<u8>,
    content: &Vec<ast::DataSegment>,
) -> Result<(), BoxError> {
    write_vec_len(buffer, &content); // vec length

    for data_segment in content {
        if data_segment.mode == ast::DataSegmentMode::Active {
            write_unsigned_leb128(buffer, 0);
            if let Some(offset) = &data_segment.offset {
                write_code_expr(buffer, &offset.value);
            }
        } else {
            write_unsigned_leb128(buffer, 1);
        }

        write_vec_len(buffer, &data_segment.bytes); // vec length
        buffer.write(&data_segment.bytes)?;
    }

    Ok(())
}

fn write_section_type(buffer: &mut Vec<u8>, content: &Vec<ast::Type>) -> Result<(), BoxError> {
    write_vec_len(buffer, &content); // vec length

    for t in content {
        buffer.push(0x60);

        write_vec_len(buffer, &t.params); // vec length
        for param in &t.params {
            write_value_type(buffer, param);
        }

        write_vec_len(buffer, &t.results); // vec length
        for result in &t.results {
            write_value_type(buffer, result);
        }
    }
    Ok(())
}

fn write_section_custom(
    buffer: &mut Vec<u8>,
    content: &ast::CustomSection,
) -> Result<(), BoxError> {
    match content {
        ast::CustomSection::Unknown(name, bytes) => {
            write_utf8(buffer, &name);
            buffer.extend_from_slice(&bytes);
        }
        ast::CustomSection::Name(content) => {
            write_utf8(buffer, "name");
            write_section_custom_name(buffer, &content)?
        }

        ast::CustomSection::BuildId(id) => {
            write_utf8(buffer, "build_id");
            write_section_custom_build_id(buffer, id)?
        }

        ast::CustomSection::CoredumpCore(content) => {
            write_utf8(buffer, "core");
            wasm_coredump_encoder::encode_coredump_process(buffer, content)?;
        }

        ast::CustomSection::CoredumpCoreStack(content) => {
            write_utf8(buffer, "corestack");
            wasm_coredump_encoder::encode_coredump_stack(buffer, content)?;
        }
    }

    Ok(())
}

fn write_section_custom_build_id(buffer: &mut Vec<u8>, id: &[u8]) -> Result<(), BoxError> {
    write_unsigned_leb128(buffer, id.len() as u64);
    buffer.extend_from_slice(id);
    Ok(())
}

pub fn write_section_custom_name(
    buffer: &mut Vec<u8>,
    content: &ast::DebugNames,
) -> Result<(), BoxError> {
    if let Some(_module_name) = &content.module {
        warn!("Module Name not implemented yet")
    }

    if let Some(func_names) = &content.func_names {
        buffer.push(1);

        let mut subsection = vec![];
        {
            let func_names = func_names.lock().unwrap();

            write_unsigned_leb128(&mut subsection, func_names.len() as u64);

            for funcidx in 0..func_names.len() {
                if let Some(name) = func_names.get(&(funcidx as u32)) {
                    write_unsigned_leb128(&mut subsection, funcidx as u64);
                    write_utf8(&mut subsection, name);
                }
            }
        }

        write_unsigned_leb128(buffer, subsection.len() as u64);
        buffer.extend_from_slice(&subsection)
    }

    if let Some(_func_local_names) = &content.func_local_names {
        warn!("Local Names not implemented yet")
    }

    Ok(())
}

fn write_section_element(
    buffer: &mut Vec<u8>,
    content: &Vec<ast::Element>,
) -> Result<(), BoxError> {
    write_vec_len(buffer, &content); // vec length

    for t in content {
        match t {
            ast::Element::FuncActive(expr, vec) => {
                buffer.push(0);
                write_code_expr(buffer, &expr.value);
                write_vec_len(buffer, &vec.lock().unwrap());
                for funcidx in vec.lock().unwrap().iter() {
                    write_unsigned_leb128(buffer, *funcidx as u64);
                }
            }
        }
    }
    Ok(())
}

fn write_section_code(
    buffer: &mut Vec<u8>,
    content: &ast::Value<Vec<ast::Code>>,
) -> Result<(), BoxError> {
    write_vec_len(buffer, &content.value); // vec length

    for func in &content.value {
        let before_offset = buffer.len();
        buffer.push(0x0); // func size, going to be fixed.

        write_code_local(buffer, &func.locals);
        write_code_expr(buffer, &func.body.lock().unwrap().value);

        // func size fixup
        {
            let after_offset = buffer.len();
            let func_len = after_offset - before_offset - 1;

            write_unsigned_leb128_at_offset(buffer, before_offset, func_len);
        }
    }
    Ok(())
}

fn write_vec_len<T>(buffer: &mut Vec<u8>, vec: &Vec<T>) {
    write_unsigned_leb128(buffer, vec.len() as u64);
}

fn write_code_local(buffer: &mut Vec<u8>, locals: &Vec<ast::CodeLocal>) {
    write_vec_len(buffer, locals); // vec length

    for local in locals {
        write_unsigned_leb128(buffer, local.count as u64);
        write_value_type(buffer, &local.value_type);
    }
}

fn write_value_type(buffer: &mut Vec<u8>, value_type: &ast::ValueType) {
    use ast::NumType::*;
    use ast::ValueType::*;
    let b: u8 = match value_type {
        NumType(I32) => 0x7F,
        NumType(I64) => 0x7E,
        NumType(F32) => 0x7D,
        NumType(F64) => 0x7C,
    };
    buffer.push(b);
}

fn write_code_expr(buffer: &mut Vec<u8>, expr: &Vec<ast::Value<ast::Instr>>) {
    for instr in expr {
        let id = instr.value.clone();

        macro_rules! write_instr {
            ($byte:expr, $instr:ident) => {
                if matches!(id, ast::Instr::$instr) {
                    buffer.push($byte);
                    continue;
                }
            };
            ($byte:expr, $instr:ident(u8)) => {
                if let ast::Instr::$instr(imm0) = id {
                    buffer.push($byte);
                    buffer.push(imm0);
                    continue;
                }
            };

            ($byte:expr, $instr:ident(f32)) => {
                if let ast::Instr::$instr(imm0) = id {
                    buffer.push($byte);
                    write_float_f32(buffer, imm0);
                    continue;
                }
            };

            ($byte:expr, $instr:ident(f64)) => {
                if let ast::Instr::$instr(imm0) = id {
                    buffer.push($byte);
                    write_float_f64(buffer, imm0);
                    continue;
                }
            };

            ($byte:expr, $instr:ident(i32)) => {
                if let ast::Instr::$instr(imm0) = id {
                    buffer.push($byte);
                    write_signed_leb128(buffer, imm0 as i64);
                    continue;
                }
            };

            ($byte:expr, $instr:ident(i64)) => {
                if let ast::Instr::$instr(imm0) = id {
                    buffer.push($byte);
                    write_signed_leb128(buffer, imm0);
                    continue;
                }
            };

            ($byte:expr, $instr:ident(u32)) => {
                if let ast::Instr::$instr(imm0) = id {
                    buffer.push($byte);
                    write_unsigned_leb128(buffer, imm0 as u64);
                    continue;
                }
            };

            ($byte:expr, $instr:ident(MutableValue<u32>)) => {
                if let ast::Instr::$instr(imm0) = id {
                    buffer.push($byte);
                    write_unsigned_leb128(buffer, imm0.lock().unwrap().value as u64);
                    continue;
                }
            };

            ($byte:expr, $instr:ident(u32, u32)) => {
                if let ast::Instr::$instr(imm0, imm1) = id {
                    buffer.push($byte);
                    write_unsigned_leb128(buffer, imm0 as u64);
                    write_unsigned_leb128(buffer, imm1 as u64);
                    continue;
                }
            };

            ($byte:expr, $instr:ident(MutableValue<u32>, u32)) => {
                if let ast::Instr::$instr(imm0, imm1) = id {
                    buffer.push($byte);
                    let imm0 = imm0.lock().unwrap().value;
                    write_unsigned_leb128(buffer, imm0 as u64);
                    write_unsigned_leb128(buffer, imm1 as u64);
                    continue;
                }
            };

            ($byte:expr, $instr:ident(Vec<u32>, u32)) => {
                if let ast::Instr::$instr(imm0, imm1) = id.clone() {
                    buffer.push($byte);
                    write_vec_len(buffer, &imm0); // vec lengh
                    for imm in imm0 {
                        write_unsigned_leb128(buffer, imm as u64);
                    }
                    write_unsigned_leb128(buffer, imm1 as u64);
                    continue;
                }
            };
        }

        write_instr!(0x00, unreachable);
        write_instr!(0x01, nop);

        if let ast::Instr::Block(ref block_type, body) = id {
            buffer.push(0x02);
            write_blocktype(buffer, block_type);
            write_code_expr(buffer, &body.lock().unwrap().value);
            continue;
        }

        if let ast::Instr::Loop(ref block_type, body) = id {
            buffer.push(0x03);
            write_blocktype(buffer, block_type);
            write_code_expr(buffer, &body.lock().unwrap().value);
            continue;
        }

        if let ast::Instr::If(ref block_type, body) = id {
            // FIXME: support IfElse, If will contain both
            buffer.push(0x04);
            write_blocktype(buffer, block_type);
            write_code_expr(buffer, &body.lock().unwrap().value);
            continue;
        }

        write_instr!(0xc, br(u32));
        write_instr!(0xd, br_if(u32));
        write_instr!(0x0b, end);
        write_instr!(0x05, else_end);
        write_instr!(0x0e, br_table(Vec<u32>, u32));
        write_instr!(0x0f, Return);
        write_instr!(0x10, call(MutableValue<u32>));
        write_instr!(0x11, call_indirect(u32, u32));

        write_instr!(0x1a, drop);
        write_instr!(0x1b, select);
        // write_instr!(0x1c, select);

        write_instr!(0x20, local_get(u32));
        write_instr!(0x21, local_set(u32));
        write_instr!(0x22, local_tee(u32));
        write_instr!(0x23, global_get(u32));
        write_instr!(0x24, global_set(u32));
        write_instr!(0x25, table_get(u32));
        write_instr!(0x26, table_set(u32));

        write_instr!(0x28, i32_load(MutableValue<u32>, u32));
        write_instr!(0x29, i64_load(MutableValue<u32>, u32));
        write_instr!(0x2a, f32_load(MutableValue<u32>, u32));
        write_instr!(0x2b, f64_load(MutableValue<u32>, u32));
        write_instr!(0x2c, i32_load8_s(MutableValue<u32>, u32));
        write_instr!(0x2d, i32_load8_u(MutableValue<u32>, u32));
        write_instr!(0x2e, i32_load16_s(MutableValue<u32>, u32));
        write_instr!(0x2f, i32_load16_u(MutableValue<u32>, u32));
        write_instr!(0x30, i64_load8_s(MutableValue<u32>, u32));
        write_instr!(0x31, i64_load8_u(MutableValue<u32>, u32));
        write_instr!(0x32, i64_load16_s(MutableValue<u32>, u32));
        write_instr!(0x33, i64_load16_u(MutableValue<u32>, u32));
        write_instr!(0x34, i64_load32_s(MutableValue<u32>, u32));
        write_instr!(0x35, i64_load32_u(MutableValue<u32>, u32));

        write_instr!(0x36, i32_store(MutableValue<u32>, u32));
        write_instr!(0x37, i64_store(MutableValue<u32>, u32));
        write_instr!(0x38, f32_store(MutableValue<u32>, u32));
        write_instr!(0x39, f64_store(MutableValue<u32>, u32));
        write_instr!(0x3a, i32_store8(MutableValue<u32>, u32));
        write_instr!(0x3b, i32_store16(MutableValue<u32>, u32));
        write_instr!(0x3c, i64_store8(MutableValue<u32>, u32));
        write_instr!(0x3d, i64_store16(MutableValue<u32>, u32));
        write_instr!(0x3e, i64_store32(MutableValue<u32>, u32));

        write_instr!(0x3f, memory_size(u8));
        write_instr!(0x40, memory_grow(u8));

        write_instr!(0x41, i32_const(i32));
        write_instr!(0x42, i64_const(i64));
        write_instr!(0x43, f32_const(f32));
        write_instr!(0x44, f64_const(f64));

        write_instr!(0x45, i32_eqz);
        write_instr!(0x46, i32_eq);
        write_instr!(0x47, i32_ne);
        write_instr!(0x48, i32_lt_s);
        write_instr!(0x49, i32_lt_u);
        write_instr!(0x4a, i32_gt_s);
        write_instr!(0x4b, i32_gt_u);
        write_instr!(0x4c, i32_le_s);
        write_instr!(0x4d, i32_le_u);
        write_instr!(0x4e, i32_ge_s);
        write_instr!(0x4f, i32_ge_u);

        write_instr!(0x50, i64_eqz);
        write_instr!(0x51, i64_eq);
        write_instr!(0x52, i64_ne);
        write_instr!(0x53, i64_lt_s);
        write_instr!(0x54, i64_lt_u);
        write_instr!(0x55, i64_gt_s);
        write_instr!(0x56, i64_gt_u);
        write_instr!(0x57, i64_le_s);
        write_instr!(0x58, i64_le_u);
        write_instr!(0x59, i64_ge_s);
        write_instr!(0x5a, i64_ge_u);

        write_instr!(0x5b, f32_eq);
        write_instr!(0x5c, f32_ne);
        write_instr!(0x5d, f32_lt);
        write_instr!(0x5e, f32_gt);
        write_instr!(0x5f, f32_le);
        write_instr!(0x60, f32_ge);

        write_instr!(0x61, f64_eq);
        write_instr!(0x62, f64_ne);
        write_instr!(0x63, f64_lt);
        write_instr!(0x64, f64_gt);
        write_instr!(0x65, f64_le);
        write_instr!(0x66, f64_ge);

        write_instr!(0x67, i32_clz);
        write_instr!(0x68, i32_ctz);
        write_instr!(0x69, i32_popcnt);
        write_instr!(0x6a, i32_add);
        write_instr!(0x6b, i32_sub);
        write_instr!(0x6c, i32_mul);
        write_instr!(0x6d, i32_div_s);
        write_instr!(0x6e, i32_div_u);
        write_instr!(0x6f, i32_rem_s);
        write_instr!(0x70, i32_rem_u);
        write_instr!(0x71, i32_and);
        write_instr!(0x72, i32_or);
        write_instr!(0x73, i32_xor);
        write_instr!(0x74, i32_shl);
        write_instr!(0x75, i32_shr_s);
        write_instr!(0x76, i32_shr_u);
        write_instr!(0x77, i32_rotl);
        write_instr!(0x78, i32_rotr);

        write_instr!(0x79, i64_clz);
        write_instr!(0x7a, i64_ctz);
        write_instr!(0x7b, i64_popcnt);
        write_instr!(0x7c, i64_add);
        write_instr!(0x7d, i64_sub);
        write_instr!(0x7e, i64_mul);
        write_instr!(0x7f, i64_div_s);
        write_instr!(0x80, i64_div_u);
        write_instr!(0x81, i64_rem_s);
        write_instr!(0x82, i64_rem_u);
        write_instr!(0x83, i64_and);
        write_instr!(0x84, i64_or);
        write_instr!(0x85, i64_xor);
        write_instr!(0x86, i64_shl);
        write_instr!(0x87, i64_shr_s);
        write_instr!(0x88, i64_shr_u);
        write_instr!(0x89, i64_rotl);
        write_instr!(0x8a, i64_rotr);

        write_instr!(0x8b, f32_abs);
        write_instr!(0x8c, f32_neg);
        write_instr!(0x8d, f32_ceil);
        write_instr!(0x8e, f32_floor);
        write_instr!(0x8f, f32_trunc);
        write_instr!(0x90, f32_nearest);
        write_instr!(0x91, f32_sqrt);
        write_instr!(0x92, f32_add);
        write_instr!(0x93, f32_sub);
        write_instr!(0x94, f32_mul);
        write_instr!(0x95, f32_div);
        write_instr!(0x96, f32_min);
        write_instr!(0x97, f32_max);
        write_instr!(0x98, f32_copysign);

        write_instr!(0x99, f64_abs);
        write_instr!(0x9a, f64_neg);
        write_instr!(0x9b, f64_ceil);
        write_instr!(0x9c, f64_floor);
        write_instr!(0x9d, f64_trunc);
        write_instr!(0x9e, f64_nearest);
        write_instr!(0x9f, f64_sqrt);
        write_instr!(0xa0, f64_add);
        write_instr!(0xa1, f64_sub);
        write_instr!(0xa2, f64_mul);
        write_instr!(0xa3, f64_div);
        write_instr!(0xa4, f64_min);
        write_instr!(0xa5, f64_max);
        write_instr!(0xa6, f64_copysign);

        write_instr!(0xa7, i32_wrap_i64);
        write_instr!(0xa8, i32_trunc_f32_s);
        write_instr!(0xa9, i32_trunc_f32_u);
        write_instr!(0xaa, i32_trunc_f64_s);
        write_instr!(0xab, i32_trunc_f64_u);
        write_instr!(0xac, i64_extend_i32_s);
        write_instr!(0xad, i64_extend_i32_u);
        write_instr!(0xae, i64_trunc_f32_s);
        write_instr!(0xaf, i64_trunc_f32_u);
        write_instr!(0xb0, i64_trunc_f64_s);
        write_instr!(0xb1, i64_trunc_f64_u);
        write_instr!(0xb2, f32_convert_i32_s);
        write_instr!(0xb3, f32_convert_i32_u);
        write_instr!(0xb4, f32_convert_i64_s);
        write_instr!(0xb5, f32_convert_i64_u);
        write_instr!(0xb6, f32_demote_f64);
        write_instr!(0xb7, f64_convert_i32_s);
        write_instr!(0xb8, f64_convert_i32_u);
        write_instr!(0xb9, f64_convert_i64_s);
        write_instr!(0xba, f64_convert_i64_u);
        write_instr!(0xbb, f64_promote_f32);

        write_instr!(0xbc, i32_reinterpret_f32);
        write_instr!(0xbd, i64_reinterpret_f64);
        write_instr!(0xbe, f32_reinterpret_i32);
        write_instr!(0xbf, f64_reinterpret_i64);

        write_instr!(0xc0, i32_extend8_s);
        write_instr!(0xc1, i32_extend16_s);
        write_instr!(0xc2, i64_extend8_s);
        write_instr!(0xc3, i64_extend16_s);
        write_instr!(0xc4, i64_extend32_s);

        if let ast::Instr::memory_copy(imm0, imm1) = id {
            buffer.push(0xfc);
            buffer.push(10);
            buffer.push(imm0);
            buffer.push(imm1);
            continue;
        }
        if let ast::Instr::memory_fill(imm0) = id {
            buffer.push(0xfc);
            buffer.push(11);
            buffer.push(imm0);
            continue;
        }

        unimplemented!("unknown instruction: {:#?}", id);
    }
}

fn write_unsigned_leb128_at_offset(bytes: &mut Vec<u8>, offset: usize, n: usize) {
    // remove placeholder
    bytes.remove(offset);

    let mut buffer = vec![];

    leb128::write::unsigned(&mut buffer, n as u64).expect("could not write LEB128");

    let mut i = 0;
    for byte in buffer {
        bytes.insert(offset + i, byte);
        i += 1;
    }
}

pub(crate) fn write_unsigned_leb128(buffer: &mut Vec<u8>, n: u64) {
    leb128::write::unsigned(buffer, n).expect("could not write LEB128");
}

fn write_signed_leb128(buffer: &mut Vec<u8>, n: i64) {
    leb128::write::signed(buffer, n).expect("could not write LEB128");
}

fn write_float_f64(buffer: &mut Vec<u8>, n: f64) {
    let mut b = [0; 8];
    LittleEndian::write_f64(&mut b, n);
    buffer.extend(b.iter())
}

fn write_float_f32(buffer: &mut Vec<u8>, n: f32) {
    let mut b = [0; 4];
    LittleEndian::write_f32(&mut b, n);
    buffer.extend(b.iter())
}

fn write_blocktype(buffer: &mut Vec<u8>, block_type: &ast::BlockType) {
    match block_type {
        ast::BlockType::Empty => {
            buffer.push(0x40);
        }
        ast::BlockType::ValueType(valtype) => {
            write_value_type(buffer, valtype);
        }
        ast::BlockType::Typeidx(valtype) => {
            write_signed_leb128(buffer, *valtype as i64);
        }
    }
}
