use biome_js_syntax::{AnyTsIdentifierBinding, TsInterfaceDeclaration};
use type_info::{TsInterface, Type};

use crate::{TResult, TypeAnalyzer};

impl TypeAnalyzer {
    pub fn analyze_ts_interface_declaration(&self, node: &TsInterfaceDeclaration) -> TResult<Type> {
        let name = match node.id().unwrap() {
            AnyTsIdentifierBinding::TsIdentifierBinding(bind) => {
                bind.name_token().unwrap().text_trimmed().to_string()
            }
            _ => todo!(),
        };

        let mut type_params = vec![];
        if let Some(params) = node.type_parameters() {
            for param in params.items().into_iter().flatten() {
                if let Ok(param) = self.analyze_type_param(&param) {
                    type_params.push(param);
                }
            }
        }

        let members = node.members();
        let mut properties = vec![];
        for m in members {
            let ty = match self.analyze_any_ts_type_member(&m) {
                Ok(ty) => ty,
                Err(_) => continue,
            };
            properties.push(ty);
        }

        Ok(Type::Interface(TsInterface {
            name: name.to_string(),
            extends: vec![],
            type_params,
            properties,
        }))
    }
}
