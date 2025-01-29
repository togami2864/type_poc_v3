use std::path::PathBuf;

use crate::Type;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub ty: Type,
}

impl Symbol {
    pub fn new(name: String, ty: Type) -> Self {
        Self { name, ty }
    }
}

#[derive(Debug, Default)]
pub struct SymbolTable(FxHashMap<PathBuf, FxHashMap<String, Symbol>>);

impl SymbolTable {
    pub fn new() -> Self {
        Self(FxHashMap::default())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&PathBuf, &FxHashMap<String, Symbol>)> {
        self.0.iter()
    }

    pub fn insert(&mut self, path: PathBuf, symbol: Symbol) {
        self.0
            .entry(path)
            .or_default()
            .insert(symbol.name.clone(), symbol);
    }

    pub fn get(&self, path: &PathBuf, name: &str) -> Option<&Symbol> {
        self.0.get(path)?.get(name)
    }
}

#[derive(Debug, Default)]
pub struct BuiltinTable(FxHashMap<String, Symbol>);

impl BuiltinTable {
    pub fn new() -> Self {
        Self(FxHashMap::default())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Symbol)> {
        self.0.iter()
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.0.insert(symbol.name.clone(), symbol);
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.0.get(name)
    }
}
