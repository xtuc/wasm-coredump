use colored::Colorize;
use log::{error, warn};
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write as IoWrite;
use std::sync::Arc;
use std::{borrow, env};

use wasmgdb_ddbug_parser as ddbug_parser;

mod commands;
mod memory;

use commands::parser::parse_command;
use commands::run_command;

pub(crate) type BoxError = Box<dyn std::error::Error>;

pub(crate) fn print_value<R: gimli::Reader>(
    ctx: &Context<R>,
    addr: u32,
    type_: &ddbug_parser::Type,
    mut depth: usize,
) -> Result<String, BoxError> {
    let ident = "\t".repeat(depth);

    match &type_.kind() {
        ddbug_parser::TypeKind::Modifier(type_modifier)
            if type_modifier.kind() == ddbug_parser::TypeModifierKind::Pointer =>
        {
            let target_type = if let Some(ty) = type_modifier.ty(&ctx.ddbug) {
                format!("{}", ty)
            } else {
                "???".to_owned()
            };
            Ok(format!("{}*{} = 0x{:x}", ident, target_type.yellow(), addr))
        }
        ddbug_parser::TypeKind::Base(base_type) => {
            let size_of = base_type.byte_size().unwrap_or(4);
            let mut bytes = memory::read(&ctx.coredump.data, addr, size_of)?.to_vec();
            bytes.reverse();
            let value = match base_type.encoding() {
                ddbug_parser::BaseTypeEncoding::Boolean => {
                    assert_eq!(bytes.len(), 1);

                    if bytes[0] == 0x0 {
                        "false".to_owned()
                    } else {
                        "true".to_owned()
                    }
                }
                _ => format!("0x{}", hex::encode(&bytes)),
            };
            Ok(format!(
                "{}{} = {}",
                ident,
                base_type.name().unwrap().yellow(),
                value
            ))
        }
        ddbug_parser::TypeKind::Struct(struct_type) => {
            let mut out = "".to_owned();
            write!(
                out,
                "{}{} = {{",
                ident,
                struct_type.name().unwrap().yellow()
            )?;

            if depth < 1 {
                write!(out, "\n")?;

                depth += 1;
                for member in struct_type.members() {
                    if let Some(member_type) = member.ty(&ctx.ddbug) {
                        let addr = memory::get_member_addr(addr, member)?;
                        let value = print_value(ctx, addr, member_type.as_ref(), depth)?;

                        let ident = "\t".repeat(depth);
                        let member_name = member.name().unwrap_or_else(|| "<unknown>").green();

                        write!(out, "{}{} (0x{:x}): {}\n", ident, member_name, addr, value)?;
                    } else {
                        write!(
                            out,
                            "{}{} (0x{:x}): <type unknown>\n",
                            ident,
                            member.name().unwrap().green(),
                            addr
                        )?;
                    }
                }
            } else {
                write!(out, "…")?;
            }
            write!(out, "}}")?;

            Ok(out)
        }
        ddbug_parser::TypeKind::Enumeration(enum_type) => {
            let size_of = enum_type.byte_size(&ctx.ddbug).unwrap();
            let bytes = memory::read(&ctx.coredump.data, addr, size_of)?.to_vec();

            let value =
                get_enum_name(ctx, &enum_type, &bytes).unwrap_or_else(|| "<unknown>".to_owned());

            Ok(format!(
                "{}{} = {}",
                ident,
                enum_type.name().unwrap_or_default(),
                value
            ))
        }
        e => unimplemented!("{:?}", e),
    }
}

fn get_enum_name<'i, R: gimli::Reader>(
    ctx: &Context<'i, R>,
    ty: &ddbug_parser::EnumerationType<'i>,
    bytes: &[u8],
) -> Option<String> {
    for item in ty.enumerators(&ctx.ddbug) {
        let item_value = item.value().unwrap_or_default();
        let search = match ty.byte_size(&ctx.ddbug).unwrap() {
            1 => bytes[0] as i64,
            4 => i32::from_le_bytes(bytes.try_into().unwrap()) as i64,
            8 => i64::from_le_bytes(bytes.try_into().unwrap()),
            n => unimplemented!("size {:?}", n),
        };

        if item_value == search {
            return item.clone().name().map(|v| v.to_owned());
        }
    }

    None
}

fn repl(
    coredump: &core_wasm_ast::coredump::Coredump,
    source: &core_wasm_ast::traverse::WasmModule,
    ddbug: ddbug_parser::FileHash<'_>,
) -> Result<(), BoxError> {
    // Load a section and return as `Cow<[u8]>`.
    let load_section = |id: gimli::SectionId| -> Result<borrow::Cow<[u8]>, gimli::Error> {
        if let Some(bytes) = source.get_custom_section(id.name()) {
            Ok(borrow::Cow::from(bytes))
        } else {
            warn!("DWARF section {} not found", id.name());
            Ok(borrow::Cow::Borrowed(&[][..]))
        }
    };

    let endian = gimli::RunTimeEndian::Little;

    // Load all of the sections.
    let dwarf_cow = gimli::Dwarf::load(&load_section)?;

    // Borrow a `Cow<[u8]>` to create an `EndianSlice`.
    let borrow_section: &dyn for<'a> Fn(
        &'a borrow::Cow<[u8]>,
    ) -> gimli::EndianSlice<'a, gimli::RunTimeEndian> =
        &|section| gimli::EndianSlice::new(&*section, endian);

    // Create `EndianSlice`s for all of the sections.
    let dwarf = Arc::new(dwarf_cow.borrow(&borrow_section));

    let thread = coredump.stacks.first().ok_or("coredump has no threads")?;

    let mut ctx = Context {
        ddbug,
        coredump,
        source,
        dwarf: Arc::clone(&dwarf),
        selected_frame: None,
        variables: HashMap::new(),
    };

    let stdin = io::stdin();
    loop {
        print!("wasmgdb> ");
        io::stdout().flush().unwrap();

        let line = stdin.lock().lines().next().unwrap()?;

        match parse_command(&line) {
            Ok((_, cmd)) => {
                if let Err(err) = run_command(&mut ctx, thread, cmd) {
                    error!("failed to run command ({}): {}", line, err);
                }
            }
            Err(err) => {
                error!("error while parsing ({}): {}", line, err);
            }
        }
    }
}

pub(crate) struct Context<'a, R: gimli::Reader> {
    selected_frame: Option<core_wasm_ast::coredump::StackFrame>,
    /// Variables present in the selected scope
    variables: HashMap<String, ddbug_parser::Parameter<'a>>,

    coredump: &'a core_wasm_ast::coredump::Coredump,

    /// DWARF types
    dwarf: Arc<gimli::Dwarf<R>>,

    /// DWARF informations
    ddbug: ddbug_parser::FileHash<'a>,

    /// Source Wasm module
    source: &'a core_wasm_ast::traverse::WasmModule,
}

pub fn main() -> Result<(), BoxError> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let coredump_filename = args[1].clone();
    let source_filename = args[2].clone();

    let mut coredump = Vec::new();
    {
        let mut file = File::open(coredump_filename).expect("File not found");
        file.read_to_end(&mut coredump)
            .expect("Error while reading file");
    }

    let ctx = ddbug_parser::File::parse(source_filename.clone()).unwrap();
    let ddbug = ddbug_parser::FileHash::new(ctx.file());

    let mut source = Vec::new();
    {
        let mut file = File::open(source_filename).expect("File not found");
        file.read_to_end(&mut source)
            .expect("Error while reading file");
    }

    let source = wasm_parser::parse(&source)
        .map_err(|err| format!("failed to parse Wasm module: {}", err))?;
    let source = core_wasm_ast::traverse::WasmModule::new(Arc::new(source));

    let coredump_wasm = wasm_parser::parse(&coredump)
        .map_err(|err| format!("failed to parse Wasm module: {}", err))?;
    let coredump_wasm = core_wasm_ast::traverse::WasmModule::new(Arc::new(coredump_wasm));
    let coredump = coredump_wasm.get_coredump()?;

    repl(&coredump, &source, ddbug)
}
