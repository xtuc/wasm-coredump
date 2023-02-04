pub mod coredump;
use core_wasm_ast as ast;
use log::warn;
use std::fmt::Write;

type BoxError = Box<dyn std::error::Error>;

pub fn print_global<W: Write>(out: &mut W, node: &ast::Global) -> Result<(), BoxError> {
    write!(out, "(global ")?;
    print_globaltype(out, &node.global_type)?;
    write!(out, " ")?;
    print_expr(out, &node.expr)?;
    write!(out, ")")?;
    Ok(())
}

pub fn print_globaltype<W: Write>(out: &mut W, node: &ast::GlobalType) -> Result<(), BoxError> {
    if node.mutable {
        write!(out, "(mut ")?;
        print_valuetype(out, &node.valtype)?;
        write!(out, ")")?;
    } else {
        print_valuetype(out, &node.valtype)?;
    }
    Ok(())
}

pub fn print_valuetype<W: Write>(out: &mut W, node: &ast::ValueType) -> Result<(), BoxError> {
    let t = match node {
        ast::ValueType::NumType(ast::NumType::F32) => "f32",
        ast::ValueType::NumType(ast::NumType::F64) => "f64",
        ast::ValueType::NumType(ast::NumType::I32) => "i32",
        ast::ValueType::NumType(ast::NumType::I64) => "i64",
    };
    write!(out, "{}", t)?;
    Ok(())
}

pub fn print_expr<W: Write>(out: &mut W, node: &ast::Expr) -> Result<(), BoxError> {
    write!(out, "(")?;
    for instr in &node.value {
        print_instr(out, &instr.value)?;
    }
    write!(out, ")")?;
    Ok(())
}

pub fn print_instr<W: Write>(out: &mut W, node: &ast::Instr) -> Result<(), BoxError> {
    use ast::Instr::*;

    let expr = match node {
        end => format!(""),
        i32_const(v) => format!("i32.const {}", v),
        e => {
            warn!("unsupported expr: {:?}", e);
            "unknown".to_owned()
        }
    };
    write!(out, "{}", expr)?;
    Ok(())
}
