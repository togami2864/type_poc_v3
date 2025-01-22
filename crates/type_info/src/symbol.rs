use std::path::PathBuf;

use crate::TypeInfo;
use biome_js_syntax::JsLanguage;
use biome_rowan::SyntaxNode;
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub node: SyntaxNode<JsLanguage>,
    pub ty: TypeInfo,
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    symbol_table: FxHashMap<PathBuf, FxHashMap<String, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbol_table: FxHashMap::default(),
        }
    }

    pub fn insert(&mut self, path: PathBuf, symbol: Symbol) {
        self.symbol_table
            .entry(path)
            .or_default()
            .insert(symbol.name.clone(), symbol);
    }

    pub fn get(&self, path: &PathBuf, name: &str) -> Option<&Symbol> {
        self.symbol_table.get(path)?.get(name)
    }
}
