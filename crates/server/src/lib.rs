use std::path::PathBuf;

use analyzer::TypeAnalyzer;
use biome_js_parser::parse;
use biome_js_syntax::{JsFileSource, JsSyntaxKind, JsSyntaxNode};
use type_info::{symbol::Symbol, Type};
use visitor::Visitor;

#[derive(Debug)]
pub struct Server {
    analyzer: TypeAnalyzer,
}

impl Server {
    pub fn new(builtin_path: Vec<PathBuf>) -> Self {
        Self {
            analyzer: TypeAnalyzer::new(builtin_path),
        }
    }

    pub fn analyze(&mut self, paths: Vec<PathBuf>) {
        for p in paths {
            let src_type = JsFileSource::ts();
            let src = std::fs::read_to_string(&p).unwrap();
            let parsed = parse(&src, src_type, Default::default());
            if parsed.has_errors() {
                panic!("Failed to parse source code: {:?}", parsed.diagnostics());
            }

            self.analyzer.visit(&parsed.tree());
        }
    }

    pub fn print_symbol_table(&self) {
        self.analyzer.print_symbol_table();
    }

    pub fn get_resolved_type_info(&self, symbol_name: String) -> Option<&Symbol> {
        if let Some(local) = self.analyzer.get_symbol(&symbol_name) {
            Some(local)
        } else if let Some(global) = self.analyzer.get_builtin_symbol(&symbol_name) {
            Some(global)
        } else {
            None
        }
    }

    pub fn get_type_info_from_builtin(&self, node: &JsSyntaxNode) -> Type {
        if matches!(node.kind(), JsSyntaxKind::JS_REFERENCE_IDENTIFIER) {
            let symbol_name = node.text_trimmed().to_string();
            if let Some(ty) = self.analyzer.get_builtin_symbol(&symbol_name) {
                ty.ty.clone()
            } else {
                Type::Unknown
            }
        } else {
            Type::Unknown
        }
    }
}
