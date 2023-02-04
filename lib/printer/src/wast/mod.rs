use colored::Colorize;
pub mod coredump;
use core_wasm_ast as ast;
use log::warn;
use std::fmt::Write;

type BoxError = Box<dyn std::error::Error>;

pub struct WastPrinter<'a, W> {
    pub module: &'a ast::traverse::WasmModule,
    pub out: &'a mut W,
}

impl<'a, W: Write> WastPrinter<'a, W> {
    pub fn print_global(&mut self, node: &ast::Global) -> Result<(), BoxError> {
        write!(self.out, "({} ", "global".bright_red())?;
        self.print_globaltype(&node.global_type)?;
        write!(self.out, " ")?;
        self.print_expr(&node.expr)?;
        write!(self.out, ")")?;
        Ok(())
    }

    pub fn print_globaltype(&mut self, node: &ast::GlobalType) -> Result<(), BoxError> {
        if node.mutable {
            write!(self.out, "({} ", "mut".bright_red())?;
            self.print_valuetype(&node.valtype)?;
            write!(self.out, ")")?;
        } else {
            self.print_valuetype(&node.valtype)?;
        }
        Ok(())
    }

    pub fn print_valuetype(&mut self, node: &ast::ValueType) -> Result<(), BoxError> {
        let t = match node {
            ast::ValueType::NumType(ast::NumType::F32) => "f32",
            ast::ValueType::NumType(ast::NumType::F64) => "f64",
            ast::ValueType::NumType(ast::NumType::I32) => "i32",
            ast::ValueType::NumType(ast::NumType::I64) => "i64",
        };
        write!(self.out, "{}", t.bright_red())?;
        Ok(())
    }

    pub fn print_expr(&mut self, node: &ast::Expr) -> Result<(), BoxError> {
        write!(self.out, "(")?;
        for instr in &node.value {
            self.print_instr(&instr.value)?;
        }
        write!(self.out, ")")?;
        Ok(())
    }

    pub fn print_instr(&mut self, node: &ast::Instr) -> Result<(), BoxError> {
        use ast::Instr::*;

        let expr = match node {
            end => format!(""),
            i32_const(v) => format!("i32.const {}", v.to_string().blue()),
            e => {
                warn!("unsupported expr: {:?}", e);
                "unknown".to_owned()
            }
        };
        write!(self.out, "{}", expr)?;
        Ok(())
    }

    pub fn print_import(&mut self, node: &ast::Import) -> Result<(), BoxError> {
        write!(self.out, "({} ", "import".bright_red())?;
        self.print_name(&node.module)?;
        write!(self.out, " ")?;
        self.print_name(&node.name)?;
        write!(self.out, " ")?;

        if let Some(t) = self.module.get_type(node.typeidx) {
            self.print_type(&t)?;
        } else {
            write!(self.out, "(type {})", node.typeidx)?;
        }

        write!(self.out, ")")?;
        Ok(())
    }

    pub fn print_name(&mut self, node: &str) -> Result<(), BoxError> {
        write!(self.out, "{}", format!("\"{}\"", node).blue())?;
        Ok(())
    }

    pub fn print_type(&mut self, node: &ast::Type) -> Result<(), BoxError> {
        write!(self.out, "(")?;
        write!(self.out, "{}", "func".bright_red())?;
        let mut i = 0;
        while i < node.params.len() - 1 {
            write!(self.out, " ({} ", "param".bright_red())?;
            loop {
                self.print_valuetype(&node.params[i])?;
                if let Some(next) = node.params.get(i + 1) {
                    if node.params[i] != *next {
                        break;
                    }
                } else {
                    break;
                }
                write!(self.out, " ")?;
                i += 1
            }
            write!(self.out, ")")?;
        }
        for result in &node.results {
            write!(self.out, " ({} ", "result".bright_red())?;
            self.print_valuetype(result)?;
            write!(self.out, ")")?;
        }
        write!(self.out, ")")?;
        Ok(())
    }
}
