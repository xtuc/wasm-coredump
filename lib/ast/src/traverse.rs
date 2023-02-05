use crate::{ast, coredump};
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use threadpool::ThreadPool;

type BoxError = Box<dyn std::error::Error>;

pub struct WasmModule {
    pub inner: Arc<ast::Module>,
    types: Mutex<HashMap<u32, ast::Type>>,
    /// Mapping between funcidx and count of func locals
    func_locals: HashMap<u32, Vec<ast::CodeLocal>>,
    func_to_typeidx: Mutex<Vec<u32>>,
    func_starts: HashMap<u32, usize>,
    func_code: HashMap<u32, ast::Code>,
    pub func_names: HashMap<u32, String>,
    imports: Vec<ast::Import>,
    globals: Vec<ast::Global>,
    exports: Vec<ast::Export>,
}
impl WasmModule {
    pub fn new(inner: Arc<ast::Module>) -> Self {
        let mut types = HashMap::new();
        let mut func_locals = HashMap::new();
        let mut func_to_typeidx = Vec::new();
        let mut imports = Vec::new();
        let mut globals = Vec::new();
        let mut exports = Vec::new();
        let mut func_starts = HashMap::new();
        let mut func_code = HashMap::new();
        let mut func_names = HashMap::new();

        for section in inner.sections.lock().unwrap().iter() {
            match &section.value {
                ast::Section::Type((_size, content)) => {
                    let mut typeidx = 0;
                    for t in &*content.lock().unwrap() {
                        types.insert(typeidx, t.to_owned());
                        typeidx += 1;
                    }
                }

                ast::Section::Import((_size, content)) => {
                    imports = content.lock().unwrap().clone();
                }

                ast::Section::Global((_size, content)) => {
                    globals = content.lock().unwrap().clone();
                }

                ast::Section::Func((_size, content)) => {
                    func_to_typeidx = content.lock().unwrap().clone();
                }

                ast::Section::Export((_section_size, content)) => {
                    exports = content.lock().unwrap().clone();
                }

                ast::Section::Code((_section_size, content)) => {
                    let mut funcidx = imports.len() as u32;

                    for c in &content.lock().unwrap().value {
                        func_code.insert(funcidx, c.clone());

                        func_starts.insert(funcidx as u32, c.body.lock().unwrap().start_offset);
                        func_locals.insert(funcidx, c.locals.clone());
                        funcidx += 1;
                    }
                }

                ast::Section::Custom((_size, section)) => match &*section.lock().unwrap() {
                    ast::CustomSection::Name(names) => {
                        func_names = names.func_names.clone();
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Self {
            inner,
            imports,
            globals,
            exports,
            func_locals,
            func_starts,
            func_code,
            func_names,
            types: Mutex::new(types),
            func_to_typeidx: Mutex::new(func_to_typeidx),
        }
    }

    pub fn get_coredump(&self) -> Result<coredump::Coredump, BoxError> {
        let mut data = vec![];
        let mut stacks = vec![];
        let mut process_info = None;
        let mut memory = vec![];
        let mut globals = vec![];

        for section in self.inner.sections.lock().unwrap().iter() {
            match &section.value {
                ast::Section::Data((_section_size, content)) => {
                    let content = content.lock().unwrap();
                    let segment = content.first().unwrap();
                    let offset = segment.compute_offset();
                    debug!("data offset: {}", offset);
                    let padding = vec![0u8; offset as usize];
                    data = [padding, segment.bytes.clone()].concat();
                }

                ast::Section::Memory((_section_size, content)) => {
                    memory = content.clone();
                }

                ast::Section::Global((_section_size, content)) => {
                    globals = content.lock().unwrap().clone();
                }

                ast::Section::Custom((_size, section)) => match &*section.lock().unwrap() {
                    ast::CustomSection::CoredumpCore(info) => process_info = Some(info.clone()),
                    ast::CustomSection::CoredumpCoreStack(stack) => stacks.push(stack.clone()),

                    _ => {}
                },

                _ => {}
            }

            debug!("data size: {:?}", data.len());
        }

        let process_info = process_info.ok_or("Wasm module is not a coredump")?;

        Ok(coredump::Coredump {
            data,
            stacks,
            process_info,
            memory,
            globals,
        })
    }

    pub fn add_data(&self, offset: u32, bytes: &[u8]) -> (u32, u32) {
        for section in self.inner.sections.lock().unwrap().iter() {
            match &section.value {
                ast::Section::Data((_section_size, content)) => {
                    let segment = ast::DataSegment {
                        offset: Some(ast::Value::new(vec![
                            ast::Value::new(ast::Instr::i32_const(offset as i64)),
                            ast::Value::new(ast::Instr::end),
                        ])),
                        bytes: bytes.to_vec(),
                        mode: ast::DataSegmentMode::Active,
                    };
                    content.lock().unwrap().push(segment);
                }
                _ => {}
            }
        }

        (offset, offset + bytes.len() as u32)
    }

    pub fn is_func_imported(&self, funcidx: u32) -> bool {
        (funcidx as usize) < self.imports.len()
    }

    pub fn imports(&self) -> &Vec<ast::Import> {
        &self.imports
    }

    pub fn globals(&self) -> &Vec<ast::Global> {
        &self.globals
    }

    pub fn func_locals_count(&self, funcidx: u32) -> u32 {
        let locals = self.func_locals(funcidx);
        let mut count = 0;
        for local in locals {
            count += local.count;
        }

        count
    }

    pub fn func_locals(&self, funcidx: u32) -> Vec<ast::CodeLocal> {
        let locals = self
            .func_locals
            .get(&funcidx)
            .expect(&format!("locals for funcidx {}", funcidx));
        locals.to_owned()
    }

    pub fn is_func_exported(&self, funcidx: u32) -> bool {
        for export in &self.exports {
            match &export.descr {
                ast::ExportDescr::Func(f) => {
                    if *f.lock().unwrap() == funcidx {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }

    pub fn get_export_func(&self, name: &str) -> Result<&ast::Code, BoxError> {
        for export in &self.exports {
            if export.name == name {
                match &export.descr {
                    ast::ExportDescr::Func(f) => {
                        let funcidx = &*f.lock().unwrap();
                        return self
                            .func_code
                            .get(funcidx)
                            .ok_or("exported function not found".into());
                    }
                    _ => return Err("export is not a function".into()),
                }
            }
        }

        Err("export not found".into())
    }

    /// Retrieve the type of a function,
    /// note that this doesn't work for imported functions as they
    /// have their type expressed differently.
    pub fn get_func_type(&self, funcidx: u32) -> ast::Type {
        let typeidx = self.get_func_typeidx(funcidx);
        let types = self.types.lock().unwrap();
        types.get(&typeidx).expect("type not found").clone()
    }

    pub fn get_type(&self, typeidx: u32) -> Option<ast::Type> {
        let types = self.types.lock().unwrap();
        types.get(&typeidx).cloned()
    }

    pub fn get_func_typeidx(&self, funcidx: u32) -> u32 {
        let func_to_typeidx = self.func_to_typeidx.lock().unwrap();

        if (funcidx as usize) < self.imports.len() {
            // Func is an imported function
            let import = &self.imports[funcidx as usize];
            import.typeidx
        } else {
            // Func is an implemented function
            let funcidx = funcidx as usize - self.imports.len();

            *func_to_typeidx
                .get(funcidx as usize)
                .expect(&format!("type not found for funcidx: {}", funcidx))
        }
    }

    pub fn get_code_section_start_offset(&self) -> Option<usize> {
        for section in self.inner.sections.lock().unwrap().iter() {
            match &section.value {
                ast::Section::Code(_) => return Some(section.start_offset),
                _ => {}
            }
        }
        None
    }

    pub fn get_start_of_func(&self, funcidx: u32) -> Option<usize> {
        if let Some(start) = self.func_starts.get(&funcidx) {
            Some(*start)
        } else {
            None
        }
    }

    pub fn get_custom_section(&self, name: &str) -> Option<Vec<u8>> {
        for section in self.inner.sections.lock().unwrap().iter() {
            match &section.value {
                ast::Section::Custom((_size, section)) => match &*section.lock().unwrap() {
                    ast::CustomSection::Unknown(section_name, bytes) => {
                        if section_name == name {
                            return Some(bytes.to_owned());
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        None
    }

    pub fn get_func_name(&self, funcidx: u32) -> Option<String> {
        for section in self.inner.sections.lock().unwrap().iter() {
            match &section.value {
                ast::Section::Custom((_size, section)) => match &*section.lock().unwrap() {
                    ast::CustomSection::Name(names) => {
                        if let Some(name) = names.func_names.get(&funcidx) {
                            return Some(name.clone());
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        None
    }

    pub fn find_import(&self, name: &str) -> u32 {
        let mut funcidx = 0;
        for section in self.inner.sections.lock().unwrap().iter() {
            match &section.value {
                ast::Section::Import((_section_size, content)) => {
                    for import in &*content.lock().unwrap() {
                        if import.name == name {
                            return funcidx;
                        }
                        funcidx += 1;
                    }
                }
                _ => {}
            }
        }

        0
    }

    pub fn add_import(&self, _import: &ast::Import) -> u32 {
        unimplemented!("Adding an import requires to shifts all references to functions by one, which is unsafe (func tables) or inconvenient (name section).");
    }

    pub fn add_global(&self, global: &ast::Global) -> Option<u32> {
        for section in self.inner.sections.lock().unwrap().iter() {
            match &section.value {
                ast::Section::Global((_section_size, content)) => {
                    let globalidx = content.lock().unwrap().len() as u32;
                    content.lock().unwrap().push(global.to_owned());
                    return Some(globalidx);
                }
                _ => {}
            }
        }

        let globals = vec![global.to_owned()];
        let global_section = ast::Section::Global((
            ast::Value::new(0), // section size will be set during encoding
            Arc::new(Mutex::new(globals)),
        ));

        self.add_section(global_section);
        return Some(0);
    }

    pub fn add_function(&self, func: &ast::Code, typeidx: u32) -> u32 {
        let mut funcidx = 0;

        for section in self.inner.sections.lock().unwrap().iter() {
            match &section.value {
                ast::Section::Import((_section_size, content)) => {
                    // TODO: why count Import and not Func section?
                    funcidx += content.lock().unwrap().len() as u32;
                }
                ast::Section::Code((_section_size, content)) => {
                    funcidx += content.lock().unwrap().value.len() as u32;
                    content.lock().unwrap().value.push(func.to_owned());
                }
                ast::Section::Func((_section_size, content)) => {
                    content.lock().unwrap().push(typeidx);
                }
                _ => {}
            }
        }

        self.func_to_typeidx.lock().unwrap().push(typeidx);
        funcidx
    }

    pub fn add_type(&self, t: &ast::Type) -> u32 {
        let mut typeidx = 0;

        for section in self.inner.sections.lock().unwrap().iter() {
            match &section.value {
                ast::Section::Type((_section_size, content)) => {
                    typeidx = content.lock().unwrap().len() as u32;
                    content.lock().unwrap().push(t.clone());

                    self.types.lock().unwrap().insert(typeidx, t.to_owned());
                }
                _ => {}
            }
        }

        typeidx
    }

    pub fn add_section(&self, s: ast::Section) {
        let mut sections = self.inner.sections.lock().unwrap();
        sections.push(ast::Value::new(s));
        sections.sort_by(|a, b| a.pos().cmp(&b.pos()));
    }
}

pub struct VisitorContext<'a, T> {
    pub module: Arc<WasmModule>,
    insert_nodes_after: Vec<T>,
    insert_nodes_before: Vec<T>,
    replace_node: Option<T>,
    pub curr_funcidx: Option<u32>,
    pub node: &'a T,
    traverse_stop: bool,
}
impl<'a, T> VisitorContext<'a, T> {
    pub fn new(module: Arc<WasmModule>, node: &'a T) -> Self {
        Self {
            node,
            module,
            insert_nodes_after: vec![],
            insert_nodes_before: vec![],
            replace_node: None,
            curr_funcidx: None,
            traverse_stop: false,
        }
    }
}

impl<'a, T> VisitorContext<'a, Vec<T>> {
    pub fn insert_node_after(&mut self, new_node: T) {
        self.insert_nodes_after.push(vec![new_node]);
    }

    pub fn insert_node_before(&mut self, new_node: T) {
        self.insert_nodes_before.push(vec![new_node]);
    }
}

impl<'a> VisitorContext<'a, ast::Value<ast::Instr>> {
    pub fn stop_traversal(&mut self) {
        self.traverse_stop = true;
    }

    pub fn insert_node_after(&mut self, new_node: ast::Instr) {
        self.insert_nodes_after.push(ast::Value::new(new_node));
    }

    pub fn insert_node_before(&mut self, new_node: ast::Instr) {
        self.insert_nodes_before.push(ast::Value::new(new_node));
    }

    pub fn replace_node(&mut self, new_node: ast::Instr) {
        self.replace_node = Some(ast::Value::new(new_node));
    }
}

pub trait Visitor {
    fn visit_instr<'a>(&self, _ctx: &'_ mut VisitorContext<'a, ast::Value<ast::Instr>>) {}
    fn visit_type<'a>(&self, _ctx: &'_ mut VisitorContext<'a, ast::Type>, _typeidx: u32) {}
    fn visit_code_section<'a>(&self, _ctx: &'_ mut VisitorContext<'a, Vec<ast::Code>>) {}
    fn visit_import_section<'a>(&self, _ctx: &'_ mut VisitorContext<'a, Vec<ast::Import>>) {}
    fn visit_func_section<'a>(&self, _ctx: &'_ mut VisitorContext<'a, Vec<u32>>) {}
    fn visit_data_section<'a>(&self, _ctx: &'_ mut VisitorContext<'a, Vec<ast::DataSegment>>) {}
    fn visit_table<'a>(&self, _ctx: &'_ mut VisitorContext<'a, ast::Table>) {}
    fn visit_export<'a>(&self, _ctx: &'_ mut VisitorContext<'a, ast::Export>) {}
    fn visit_element<'a>(&self, _ctx: &'_ mut VisitorContext<'a, ast::Element>) {}
    fn visit_code<'a>(&self, _ctx: &'_ mut VisitorContext<'a, ast::Code>, _funcidx: u32) {}
}

pub fn traverse(module: Arc<ast::Module>, visitor: Arc<dyn Visitor + Send + Sync>) {
    let pool = ThreadPool::new(num_cpus::get());

    let mut curr_funcidx = 0;

    let module_ast = Arc::new(WasmModule::new(Arc::clone(&module)));

    for section in module.sections.lock().unwrap().iter() {
        match &section.value {
            ast::Section::Func((_section_size, funcs)) => {
                let nodes = funcs.lock().unwrap().clone();
                let mut ctx = VisitorContext::new(Arc::clone(&module_ast), &nodes);
                Arc::clone(&visitor).visit_func_section(&mut ctx);
                assert!(ctx.insert_nodes_before.is_empty());

                {
                    let mut new_nodes = ctx.insert_nodes_after;
                    new_nodes.reverse();

                    for new_node in new_nodes {
                        debug!("inject new func: {:?}", new_node);
                        funcs.lock().unwrap().extend_from_slice(&new_node);
                    }
                }
            }
            ast::Section::Export((_section_size, exports)) => {
                for export in exports.lock().unwrap().iter() {
                    let mut ctx = VisitorContext::new(Arc::clone(&module_ast), export);
                    visitor.visit_export(&mut ctx);
                    assert!(ctx.insert_nodes_before.is_empty());
                    assert!(ctx.insert_nodes_after.is_empty());
                }
            }
            ast::Section::Element((_section_size, elements)) => {
                for element in elements.lock().unwrap().iter() {
                    let mut ctx = VisitorContext::new(Arc::clone(&module_ast), element);
                    visitor.visit_element(&mut ctx);
                    assert!(ctx.insert_nodes_before.is_empty());
                    assert!(ctx.insert_nodes_after.is_empty());
                }
            }
            ast::Section::Table((_section_size, tables)) => {
                let module_ast = Arc::clone(&module_ast);
                for table in tables.lock().unwrap().iter() {
                    let mut ctx = VisitorContext::new(Arc::clone(&module_ast), table);
                    visitor.visit_table(&mut ctx);
                    assert!(ctx.insert_nodes_before.is_empty());
                    assert!(ctx.insert_nodes_after.is_empty());
                }
            }
            ast::Section::Type((_section_size, types)) => {
                let mut typeidx = 0;
                let types_copy = types.lock().unwrap().clone();
                for t in types_copy {
                    let mut ctx = VisitorContext::new(Arc::clone(&module_ast), &t);
                    visitor.visit_type(&mut ctx, typeidx);
                    typeidx += 1;

                    assert!(ctx.insert_nodes_before.is_empty());
                    assert!(ctx.insert_nodes_after.is_empty());
                }
            }
            ast::Section::Import((_section_size, content)) => {
                let nodes = content.lock().unwrap().clone();
                let mut ctx = VisitorContext::new(Arc::clone(&module_ast), &nodes);
                Arc::clone(&visitor).visit_import_section(&mut ctx);
                assert!(ctx.insert_nodes_before.is_empty());

                {
                    for new_node in ctx.insert_nodes_after {
                        debug!("inject new import: {:?}", new_node);
                        content.lock().unwrap().extend_from_slice(&new_node);
                    }
                }

                curr_funcidx += content.lock().unwrap().len() as u32;
            }
            ast::Section::Code((_section_size, codes)) => {
                {
                    let nodes = codes.lock().unwrap().clone().value;
                    let mut ctx = VisitorContext::new(Arc::clone(&module_ast), &nodes);
                    Arc::clone(&visitor).visit_code_section(&mut ctx);
                    assert!(ctx.insert_nodes_before.is_empty());

                    let mut new_nodes = ctx.insert_nodes_after;
                    new_nodes.reverse();

                    for new_node in new_nodes {
                        debug!("inject new code: {:?}", new_node);
                        codes.lock().unwrap().value.extend_from_slice(&new_node);
                    }
                }

                let codes = codes.lock().unwrap().clone();
                for code in codes.value {
                    {
                        let mut ctx = VisitorContext::new(Arc::clone(&module_ast), &code);
                        Arc::clone(&visitor).visit_code(&mut ctx, curr_funcidx);
                    }

                    {
                        let visitor = Arc::clone(&visitor);
                        let module_ast = Arc::clone(&module_ast);
                        pool.execute(move || {
                            visit_expr(
                                Arc::clone(&module_ast),
                                Arc::clone(&code.body),
                                Arc::clone(&visitor),
                                curr_funcidx,
                            );
                        });
                    }

                    curr_funcidx += 1;
                }
            }
            ast::Section::Data((_section_size, segments)) => {
                let nodes = segments.lock().unwrap().clone();
                let mut ctx = VisitorContext::new(Arc::clone(&module_ast), &nodes);
                Arc::clone(&visitor).visit_data_section(&mut ctx);
                assert!(ctx.insert_nodes_before.is_empty());

                let mut new_nodes = ctx.insert_nodes_after;
                new_nodes.reverse();

                for new_node in new_nodes {
                    debug!("inject new data: {:?}", new_node);
                    segments.lock().unwrap().extend_from_slice(&new_node);
                }
            }
            _ => {}
        }
    }

    // TODO: add barrier
    pool.join();
}

fn visit_expr(
    module_ast: Arc<WasmModule>,
    expr: ast::MutableValue<Vec<ast::Value<ast::Instr>>>,
    visitor: Arc<dyn Visitor + Send + Sync>,
    curr_funcidx: u32,
) {
    let expr_copy = expr.lock().unwrap().clone();

    // Keep track of many nodes we injected since we started iterating, so that
    // subsequent inserts are at the right place.
    // The iterator is a copy of the array of nodes.
    let mut added = 0;

    for i in 0..expr_copy.value.len() {
        let instr = expr_copy.value[i].clone();
        if let ast::Instr::Block(_, body) = instr.value {
            visit_expr(Arc::clone(&module_ast), body, visitor.clone(), curr_funcidx);
        } else if let ast::Instr::If(_, body) = instr.value {
            visit_expr(Arc::clone(&module_ast), body, visitor.clone(), curr_funcidx);
        } else if let ast::Instr::Loop(_, body) = instr.value {
            visit_expr(Arc::clone(&module_ast), body, visitor.clone(), curr_funcidx);
        } else {
            let mut ctx = VisitorContext::new(Arc::clone(&module_ast), &instr);
            ctx.curr_funcidx = Some(curr_funcidx);
            visitor.visit_instr(&mut ctx);

            if let Some(replace_node) = ctx.replace_node {
                debug!("replace instr: {:?}", replace_node);
                expr.lock().unwrap().value[i + added] = replace_node;
            }

            if ctx.insert_nodes_after.len() > 0 {
                debug!("insert instr(s): {:?}", ctx.insert_nodes_after);
                expr.lock().unwrap().value.splice(
                    (i + added + 1)..(i + added + 1),
                    ctx.insert_nodes_after.clone(),
                );
                added += ctx.insert_nodes_after.len();
            }

            if ctx.insert_nodes_before.len() > 0 {
                debug!("insert instr(s): {:?}", ctx.insert_nodes_before);

                expr.lock()
                    .unwrap()
                    .value
                    .splice((i + added)..(i + added), ctx.insert_nodes_before.clone());
                added += ctx.insert_nodes_before.len();
            }

            if ctx.traverse_stop {
                break;
            }
        }
    }
}
