use std::path::PathBuf;

use analyzer::TypeAnalyzer;
use biome_js_parser::parse;
use biome_js_syntax::JsFileSource;
use visitor::Visitor;

#[derive(Debug)]
pub struct Server {
    analyzer: TypeAnalyzer,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            analyzer: TypeAnalyzer::default(),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn analyze(&mut self, paths: Vec<PathBuf>) {
        for p in paths {
            let src = std::fs::read_to_string(&p).unwrap();
            let src_type = JsFileSource::ts();
            let parsed = parse(&src, src_type, Default::default());
            if parsed.has_errors() {
                panic!("Failed to parse source code: {:?}", parsed.diagnostics());
            }

            self.analyzer.visit(&parsed.tree());
        }
    }

    pub fn get_resolved_type_info(&self, symbol_name: String) {}
}
