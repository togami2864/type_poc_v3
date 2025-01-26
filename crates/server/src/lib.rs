use std::path::PathBuf;

use analyzer::TypeAnalyzer;
use biome_js_parser::parse;
use biome_js_syntax::JsFileSource;
use type_info::symbol::Symbol;
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

    pub fn get_resolved_type_info(&self, symbol_name: String) -> Option<&Symbol> {
        if let Some(local) = self.analyzer.get_symbol(&symbol_name) {
            Some(local)
        } else if let Some(global) = self.analyzer.get_global_symbol(&symbol_name) {
            Some(global)
        } else {
            None
        }
    }
}
