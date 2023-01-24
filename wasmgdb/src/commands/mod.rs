//! Handles the parsing and execution of commands

use std::fmt;

use crate::{coredump, BoxError, Context};

mod examine;
mod find;
mod frames;
mod info;
pub(crate) mod parser;
mod print;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expr<'a> {
    Name(&'a str),
    Hex(u64),
    Int(i64),
    Deref(Box<Expr<'a>>),
    Cast(&'a str, Box<Expr<'a>>),
    MemberAccess(Box<Expr<'a>>, &'a str),
    Str(&'a str),
}

impl<'a> Expr<'a> {
    pub(crate) fn object(&'a self) -> Option<&'a str> {
        match self {
            Expr::Str(_) | Expr::Cast(_, _) | Expr::Hex(_) | Expr::Int(_) => None,
            Expr::Name(n) => Some(n),
            Expr::Deref(t) => t.object(),
            Expr::MemberAccess(o, _) => o.object(),
        }
    }
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Int(n) => write!(f, "{}", n),
            Expr::Hex(n) => write!(f, "0x{:x}", n),
            Expr::Name(v) => write!(f, "{}", v),
            Expr::Str(v) => write!(f, "\"{}\"", v),
            Expr::Cast(t, v) => write!(f, "({}) {}", t, v),
            Expr::Deref(t) => write!(f, "*{}", t),
            Expr::MemberAccess(expr, v) => write!(f, "{}.{}", expr, v),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Command<'a> {
    Unknown,
    Backtrace,
    SelectFrame(usize),
    Print(PrintFormat, Expr<'a>),
    Examine(Expr<'a>, (Option<u32>, Option<PrintFormat>)),
    Find(Option<Expr<'a>>, Option<Expr<'a>>, Expr<'a>),
    Info(&'a str, Vec<Expr<'a>>),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum PrintFormat {
    None,
    String,
}

pub(crate) fn run_command<R: gimli::Reader>(
    ctx: &mut Context<R>,
    stack_frames: &Vec<coredump::StackFrame>,
    cmd: Command,
) -> Result<(), BoxError> {
    match cmd {
        Command::Backtrace => {
            frames::backtrace(ctx, stack_frames)?;
        }

        Command::Examine(what, (number, format)) => {
            examine::examine(&ctx, what, number, format)?;
        }

        Command::Print(format, what) => {
            print::print(&ctx, format, what)?;
        }

        Command::Find(start, end, expr) => {
            find::find(&ctx, start, end, expr)?;
        }

        Command::Info(what, args) => {
            info::info(&ctx, what, args)?;
        }

        Command::SelectFrame(selected_frame) => {
            let stack_frame = &stack_frames[stack_frames.len() - 1 - selected_frame];

            frames::print_frame(ctx, &stack_frame)?;
            frames::select_frame(ctx, &stack_frame)?;

            ctx.selected_frame = Some(stack_frame.clone());
        }

        Command::Unknown => return Err("unknow command".into()),
    }

    Ok(())
}
