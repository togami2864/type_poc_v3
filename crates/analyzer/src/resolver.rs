use std::path::PathBuf;

use rustc_hash::FxHashMap;
use type_info::{symbol::Symbol, *};

use crate::TypeAnalyzer;

impl TypeAnalyzer {
    pub fn resolve_type_info(&self, symbol: &Symbol, path: &PathBuf) -> TypeInfo {
        match &symbol.ty {
            TypeInfo::Interface(interface) => {
                let mut resolved_interface = interface.clone();

                for type_param in &mut resolved_interface.type_params {
                    if let Some(constraint) = &type_param.constraint {
                        type_param.constraint =
                            Some(self.resolve_type_info_inner(constraint, path));
                    }
                    if let Some(default) = &type_param.default {
                        type_param.default = Some(self.resolve_type_info_inner(default, path));
                    }
                }

                for property in &mut resolved_interface.properties {
                    property.type_info = self.resolve_type_info_inner(&property.type_info, path);
                }

                resolved_interface.extends = resolved_interface
                    .extends
                    .iter()
                    .map(|ext| self.resolve_type_info_inner(ext, path))
                    .collect();

                TypeInfo::Interface(resolved_interface)
            }
            TypeInfo::Function(func) => {
                let mut resolved_func = func.clone();
                resolved_func.params = resolved_func
                    .params
                    .into_iter()
                    .map(|param| FunctionParam {
                        name: param.name,
                        param_type: self.resolve_type_info_inner(&param.param_type, path),
                        is_optional: param.is_optional,
                    })
                    .collect();

                resolved_func.return_type =
                    Box::new(self.resolve_type_info_inner(&func.return_type, path));

                TypeInfo::Function(resolved_func)
            }
            _ => symbol.ty.clone(),
        }
    }

    fn resolve_type_info_inner(&self, ty: &TypeInfo, path: &PathBuf) -> TypeInfo {
        match ty {
            TypeInfo::TypeRef(type_ref) => {
                if let Some(symbol) = self.symbol_table.get(path, &type_ref.name) {
                    let ty = self.apply_type_arguments(&symbol.ty, &type_ref.type_params);
                    return ty;
                }
                if let Some(referred_symbol) = self.global_symbol_table.get(&type_ref.name) {
                    let mut resolved_type = self.resolve_type_info(referred_symbol, path);

                    if !type_ref.type_params.is_empty() {
                        resolved_type =
                            self.apply_type_arguments(&resolved_type, &type_ref.type_params);
                    }

                    resolved_type
                } else {
                    TypeInfo::Unknown
                }
            }
            TypeInfo::Union(types) => TypeInfo::Union(
                types
                    .iter()
                    .map(|t| self.resolve_type_info_inner(t, path))
                    .collect(),
            ),
            TypeInfo::Intersection(types) => TypeInfo::Intersection(
                types
                    .iter()
                    .map(|t| self.resolve_type_info_inner(t, path))
                    .collect(),
            ),
            TypeInfo::Function(func) => {
                let mut resolved_func = func.clone();
                resolved_func.params = resolved_func
                    .params
                    .into_iter()
                    .map(|param| FunctionParam {
                        name: param.name,
                        param_type: self.resolve_type_info_inner(&param.param_type, path),
                        is_optional: param.is_optional,
                    })
                    .collect();
                resolved_func.return_type =
                    Box::new(self.resolve_type_info_inner(&func.return_type, path));
                TypeInfo::Function(resolved_func)
            }
            _ => ty.clone(),
        }
    }

    fn apply_type_arguments(&self, base_type: &TypeInfo, type_args: &[TypeInfo]) -> TypeInfo {
        match base_type {
            TypeInfo::Interface(interface) => {
                if interface.type_params.len() != type_args.len() {
                    return TypeInfo::Unknown;
                }

                let mut type_map = FxHashMap::default();
                for (param, arg) in interface.type_params.iter().zip(type_args.iter()) {
                    type_map.insert(param.name.clone(), arg.clone());
                }
                let resolved_properties = interface
                    .properties
                    .iter()
                    .map(|prop| {
                        let resolved_type = substitute_type(&prop.type_info, &type_map);
                        TsInterfaceProperty {
                            name: prop.name.clone(),
                            type_info: resolved_type,
                            is_optional: prop.is_optional,
                            is_readonly: prop.is_readonly,
                        }
                    })
                    .collect();
                TypeInfo::Interface(TsInterface {
                    name: interface.name.clone(),
                    extends: interface.extends.clone(),
                    properties: resolved_properties,
                    type_params: interface.type_params.clone(),
                })
            }
            _ => base_type.clone(),
        }
    }
}

pub fn substitute_type(ty: &TypeInfo, type_map: &FxHashMap<String, TypeInfo>) -> TypeInfo {
    match ty {
        TypeInfo::TypeRef(ref_type) => {
            if let Some(ty) = type_map.get(&ref_type.name) {
                return ty.clone();
            }
            if !ref_type.type_params.is_empty() {
                let mut resolved_type = vec![];
                for r in ref_type.type_params.iter() {
                    let ty = substitute_type(r, type_map);
                    resolved_type.push(ty);
                }
                TypeInfo::TypeRef(TsTypeRef {
                    name: ref_type.name.clone(),
                    type_params: resolved_type,
                })
            } else {
                ty.clone()
            }
        }
        TypeInfo::Union(types) => {
            let resolved_types = types.iter().map(|t| substitute_type(t, type_map)).collect();
            TypeInfo::Union(resolved_types)
        }
        TypeInfo::Function(func) => {
            let mut resolved_func = func.clone();
            resolved_func.params = resolved_func
                .params
                .iter()
                .map(|param| FunctionParam {
                    name: param.name.clone(),
                    param_type: substitute_type(&param.param_type, type_map),
                    is_optional: param.is_optional,
                })
                .collect();
            resolved_func.return_type = Box::new(substitute_type(&func.return_type, type_map));
            TypeInfo::Function(resolved_func)
        }
        _ => ty.clone(),
    }
}
