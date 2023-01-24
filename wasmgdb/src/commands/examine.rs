use crate::commands::{Expr, PrintFormat};
use crate::{BoxError, Context};
use std::fmt::Write;

pub(crate) fn examine<'a, R: gimli::Reader>(
    ctx: &Context<R>,
    what: Expr<'a>,
    number: Option<u32>,
    format: Option<PrintFormat>,
) -> Result<(), BoxError> {
    let addr = if let Expr::Hex(addr) = what {
        addr
    } else {
        unreachable!();
    };

    let mut out = "".to_owned();
    let number = number.unwrap_or_else(|| 8);

    for offset in 0..number {
        let v = ctx.coredump[addr as usize + offset as usize];
        match format {
            Some(PrintFormat::String) => write!(out, "{}", v as char)?,
            _ => write!(out, "0x{} ", v)?,
        };
    }

    match format {
        Some(PrintFormat::String) => println!("{} ({} char(s)) = {:?}", what, number, out),
        _ => println!("{} ({} byte(s)) = {}", what, number, out),
    }
    Ok(())
}
