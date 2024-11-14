use crate::commands::Expr;
use crate::BoxError;
use crate::Context;
use colored::Colorize;

pub(crate) fn find<'src, 'input>(
    ctx: &'src Context<'src>,
    start: Option<Expr<'input>>,
    end: Option<Expr<'input>>,
    expr: Expr<'input>,
) -> Result<(), BoxError> {
    let coredump = ctx.coredump()?;

    let start = if let Some(Expr::Hex(v)) = start {
        v as usize
    } else {
        0
    };
    let end = if let Some(Expr::Hex(v)) = end {
        v as usize
    } else {
        coredump.data.len()
    };

    let search_bytes = expr_to_bytes(&expr)?;
    let mem = &coredump.data[start..end];

    let mut offset = 0;
    let mut found = 0;
    let mut last_offset = 0;

    for window in mem.windows(search_bytes.len()) {
        if window == search_bytes {
            let v = format!("0x{:x}", offset);
            let distance_from_last = offset - last_offset;
            println!("{} after {} byte(s)", v.blue(), distance_from_last);

            found += 1;
            last_offset = offset;
        }

        offset += 1;
    }

    println!("{} pattern(s) found.", found);
    Ok(())
}

fn expr_to_bytes<'a>(expr: &Expr<'a>) -> Result<Vec<u8>, BoxError> {
    use Expr::*;

    match expr {
        Hex(n) => Ok(n.to_le_bytes().to_vec()),
        Str(s) => Ok(s.as_bytes().to_vec()),
        _ => Err(format!("cannot turn {} into bytes", expr).into()),
    }
}
