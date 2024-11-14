//! Handles the parsing and execution of commands

use crate::{BoxError, Context};
use std::fmt;

mod breakpoint;
mod examine;
mod find;
mod frames;
mod info;
pub(crate) mod parser;
mod print;
mod run;

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
    Run,
    BreakPoint(u32),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum PrintFormat {
    None,
    String,
}

pub(crate) fn run_command<'src, 'input>(
    ctx: &'src Context<'src>,
    cmd: Command<'input>,
) -> Result<(), BoxError> {
    match cmd {
        Command::Run => {
            run::run(ctx)?;
        }

        Command::BreakPoint(pos) => {
            breakpoint::set_breakpoint(ctx, pos)?;
        }

        Command::Backtrace => {
            let thread = ctx.thread()?;
            frames::backtrace(ctx, &thread)?;
        }

        Command::Examine(what, (number, format)) => {
            let coredump = ctx.coredump()?;
            examine::examine(&coredump, what, number, format)?;
        }

        Command::Print(format, what) => {
            print::print(ctx, format, what)?;
        }

        Command::Find(start, end, expr) => {
            find::find(ctx, start, end, expr)?;
        }

        Command::Info(what, args) => {
            info::info(ctx, what, args)?;
        }

        Command::SelectFrame(selected_frame) => {
            let thread = ctx.thread()?;
            let stack_frame = &thread.frames[thread.frames.len() - 1 - selected_frame];

            frames::print_frame(ctx, &stack_frame)?;
            frames::select_frame(ctx, &stack_frame)?;
            *ctx.selected_frame.borrow_mut() = Some(stack_frame.clone());
        }

        Command::Unknown => return Err("unknown command".into()),
    }

    Ok(())
}
