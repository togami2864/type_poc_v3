use std::path::PathBuf;

use crate::TypeInfo;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub ty: TypeInfo,
}

impl Symbol {
    pub fn new(name: String, ty: TypeInfo) -> Self {
        Self { name, ty }
    }
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

#[derive(Debug, Default)]
pub struct GlobalSymbolTable {
    symbol_table: FxHashMap<String, Symbol>,
}

impl GlobalSymbolTable {
    pub fn new() -> Self {
        Self {
            symbol_table: FxHashMap::default(),
        }
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.symbol_table.insert(symbol.name.clone(), symbol);
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbol_table.get(name)
    }
}
