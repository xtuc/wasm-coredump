use crate::commands::parser::parse_command;
use crate::commands::run_command;
use crate::memory;
use colored::Colorize;
use log::error;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;
use std::io;
use std::io::BufRead;
use std::io::Write as IoWrite;
use std::sync::Mutex;
use wasmgdb_ddbug_parser as ddbug_parser;

pub(crate) type BoxError = Box<dyn std::error::Error>;

pub(crate) struct Context<'a> {
    pub(crate) selected_frame: Option<wasm_coredump_types::StackFrame>,
    pub(crate) selected_thread: Option<usize>,

    /// Variables present in the selected scope
    pub(crate) variables: HashMap<String, ddbug_parser::Parameter<'a>>,

    pub(crate) coredump: Option<wasm_coredump_types::Coredump>,

    /// DWARF informations
    pub(crate) ddbug: ddbug_parser::FileHash<'a>,

    /// Source Wasm module
    pub(crate) source: &'a core_wasm_ast::traverse::WasmModule,

    pub(crate) break_points: Mutex<HashSet<u32>>,
}

impl<'a> Context<'a> {
    pub(crate) fn coredump(&mut self) -> Result<wasm_coredump_types::Coredump, BoxError> {
        self.coredump
            .as_ref()
            .map(|c| c.clone())
            .ok_or("No coredump present".into())
    }

    pub(crate) fn thread(&mut self) -> Result<wasm_coredump_types::CoreStack, BoxError> {
        let coredump = self.coredump()?;

        self.selected_thread
            .map(|idx| coredump.stacks[idx].clone())
            .ok_or("No frame selected".into())
    }
}

pub(crate) fn print_value<'a>(
    ctx: &'a Context<'a>,
    addr: u32,
    type_: &'a ddbug_parser::Type,
    mut depth: usize,
) -> Result<String, BoxError> {
    let ident = "\t".repeat(depth);
    let coredump = ctx.coredump.as_ref().ok_or("no coredump present")?;

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
            let mut bytes = memory::read(&coredump.data, addr, size_of)?.to_vec();
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
            let bytes = memory::read(&coredump.data, addr, size_of)?.to_vec();

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

fn get_enum_name<'i>(
    ctx: &Context<'i>,
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

pub(crate) fn repl(
    coredump: Option<wasm_coredump_types::Coredump>,
    source: &core_wasm_ast::traverse::WasmModule,
    ddbug: ddbug_parser::FileHash<'_>,
) -> Result<(), BoxError> {
    let mut ctx = Context {
        ddbug,
        coredump,
        source,
        variables: HashMap::new(),

        selected_frame: None,
        selected_thread: Some(0), // auto select the first thread

        break_points: Mutex::new(HashSet::new()),
    };

    let stdin = io::stdin();
    loop {
        print!("wasmgdb> ");
        io::stdout().flush().unwrap();

        if let Some(line) = stdin.lock().lines().next() {
            let line = line?;
            match parse_command(&line) {
                Ok((_, cmd)) => {
                    if let Err(err) = run_command(&mut ctx, cmd) {
                        error!("failed to run command ({}): {}", line, err);
                    }
                }
                Err(err) => {
                    error!("error while parsing ({}): {}", line, err);
                }
            }
        } else {
            return Ok(());
        }
    }
}
