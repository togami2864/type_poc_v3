use std::path::PathBuf;

use biome_js_parser::parse;
use biome_js_syntax::*;
use biome_rowan::SyntaxError;
use symbol::{GlobalSymbolTable, Symbol, SymbolTable};
use type_info::*;
use visitor::Visitor;

mod expr;
mod function;
mod resolver;
mod stmt;
mod ts;

type TResult<T> = Result<T, SyntaxError>;

#[derive(Debug, Default)]
pub struct TypeAnalyzer {
    current_path: PathBuf,
    symbol_table: SymbolTable,
    global_symbol_table: GlobalSymbolTable,
}

impl TypeAnalyzer {
    pub fn new(builtin_path: Vec<PathBuf>) -> Self {
        let mut analyzer = Self {
            current_path: PathBuf::new(),
            symbol_table: SymbolTable::new(),
            global_symbol_table: GlobalSymbolTable::new(),
        };

        analyzer.init_builtin_types(builtin_path);
        analyzer
    }

    pub fn print_symbol_table(&self) {
        for (path, symbol_table) in self.symbol_table.iter() {
            println!("Path: {:?}", path);
            for (name, symbol) in symbol_table.iter() {
                println!("  \x1b[32m{}\x1b[0m: {:?}\n", name, symbol);
            }
        }
    }

    pub fn print_global_symbol_table(&self) {
        for (name, symbol) in self.global_symbol_table.iter() {
            println!("  \x1b[32m{}\x1b[0m: {:?}\n", name, symbol);
        }
    }

    fn init_builtin_types(&mut self, path: Vec<PathBuf>) {
        let src_type = JsFileSource::d_ts();

        for p in path {
            let src = std::fs::read_to_string(&p).unwrap();
            let parsed = parse(&src, src_type, Default::default());
            if parsed.has_errors() {
                panic!("Failed to parse source code: {:?}", parsed.diagnostics());
            }
            let root = parsed.tree();
            self.visit(&root);
            for (_, symbol_table) in self.symbol_table.iter() {
                for (_, symbol) in symbol_table.iter() {
                    self.global_symbol_table.insert(symbol.clone());
                }
            }
        }
    }

    pub fn insert_new_symbol(&mut self, symbol: Symbol) {
        self.symbol_table.insert(self.current_path.clone(), symbol);
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbol_table.get(&self.current_path, name)
    }

    pub fn get_global_symbol(&self, name: &str) -> Option<&Symbol> {
        self.global_symbol_table.get(name)
    }
}

impl Visitor for TypeAnalyzer {
    fn visit(&mut self, node: &AnyJsRoot) {
        match node {
            AnyJsRoot::JsModule(node) => self.visit_module(node),
            AnyJsRoot::TsDeclarationModule(node) => self.visit_ts_declaration_module(node),
            node => todo!("{:?}", node),
        }
    }

    fn visit_module(&mut self, node: &JsModule) {
        for item in node.items() {
            self.visit_module_item(&item);
        }
    }

    fn visit_ts_declaration_module(&mut self, node: &TsDeclarationModule) {
        for item in node.items() {
            self.visit_module_item(&item);
        }
    }

    fn visit_module_item(&mut self, node: &AnyJsModuleItem) {
        match node {
            AnyJsModuleItem::AnyJsStatement(node) => self.visit_statement(node),
            node => todo!("{:?}", node),
        }
    }

    fn visit_statement(&mut self, node: &AnyJsStatement) {
        match node {
            AnyJsStatement::TsDeclareStatement(node) => {
                self.visit_ts_declare_statement(node);
            }
            AnyJsStatement::TsInterfaceDeclaration(node) => {
                self.visit_ts_interface_declaration(node);
            }
            AnyJsStatement::JsVariableStatement(node) => {
                self.visit_js_variable_statement(node);
            }
            AnyJsStatement::JsExpressionStatement(node) => {
                self.visit_js_expression_statement(node);
            }
            AnyJsStatement::JsFunctionDeclaration(node) => {
                self.visit_js_function_declaration(node);
            }
            node => todo!("{:?}", node),
        }
    }

    fn visit_js_expression_statement(&mut self, _node: &JsExpressionStatement) {}

    fn visit_js_function_declaration(&mut self, node: &JsFunctionDeclaration) {
        if let Ok(ty) = self.analyze_js_function_declaration(node) {
            let symbol = Symbol::new(node.id().unwrap().to_string(), ty);
            self.insert_new_symbol(symbol);
        }
    }

    fn visit_js_variable_statement(&mut self, node: &JsVariableStatement) {
        if let Ok(list) = node.declaration() {
            for decl in list.declarators().into_iter().flatten() {
                self.visit_js_variable_declarator(&decl);
            }
        }
    }

    fn visit_ts_declare_statement(&mut self, node: &TsDeclareStatement) {
        if let Ok(n) = node.declaration() {
            match n {
                AnyJsDeclarationClause::JsVariableDeclarationClause(node) => {
                    self.visit_js_variable_declaration_clause(&node);
                }
                _ => todo!("{:?}", n),
            }
        }
    }

    fn visit_ts_interface_declaration(&mut self, node: &TsInterfaceDeclaration) {
        let interface_name = match node.id().unwrap() {
            AnyTsIdentifierBinding::TsIdentifierBinding(bind) => {
                let name_token = bind.name_token().unwrap();
                name_token.text_trimmed().to_string()
            }
            _ => todo!(),
        };
        let ty = self.analyze_ts_interface_declaration(node).unwrap();
        let symbol = Symbol::new(interface_name.to_string(), ty);
        self.insert_new_symbol(symbol);
    }

    fn visit_js_variable_declaration_clause(&mut self, node: &JsVariableDeclarationClause) {
        if let Ok(decl) = node.declaration() {
            for d in decl.declarators().into_iter().flatten() {
                self.visit_js_variable_declarator(&d)
            }
        }
    }

    fn visit_js_variable_declarator(&mut self, node: &JsVariableDeclarator) {
        let id = match node.id() {
            Ok(node) => match node {
                AnyJsBindingPattern::AnyJsBinding(node) => match node {
                    AnyJsBinding::JsIdentifierBinding(bind) => {
                        bind.name_token().unwrap().text_trimmed().to_string()
                    }
                    _ => todo!(),
                },
                AnyJsBindingPattern::JsArrayBindingPattern(node) => {
                    todo!("array binding pattern {:?}", node)
                }
                AnyJsBindingPattern::JsObjectBindingPattern(node) => {
                    todo!("object binding pattern {:?}", node)
                }
            },
            Err(_) => todo!(),
        };
        let ty = self.analyze_js_variable_declarator(node).unwrap();
        let symbol = Symbol::new(id.to_string(), ty);
        self.insert_new_symbol(symbol);
    }
}
