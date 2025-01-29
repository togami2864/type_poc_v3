use biome_js_syntax::{AnyTsReturnType, TsDeclareFunctionDeclaration, TsFunctionType};
use type_info::{TsFunctionSignature, Type};

use crate::{TResult, TypeAnalyzer};

impl TypeAnalyzer {
    pub fn analyze_ts_function_type(&self, node: &TsFunctionType) -> TResult<Type> {
        let mut type_params = vec![];
        if let Some(params) = node.type_parameters() {
            for p in params.items().into_iter().flatten() {
                let param = self.analyze_type_param(&p)?;
                type_params.push(param);
            }
        };

        let mut params = vec![];

        if let Ok(parameters) = node.parameters() {
            params = self.analyze_js_parameters(&parameters)?;
        }

        let return_type = if let Ok(ty) = node.return_type() {
            match ty {
                AnyTsReturnType::AnyTsType(ty) => {
                    let ty = self.analyze_any_ts_types(&ty)?;
                    Box::new(ty)
                }
                node => todo!("{:?}", node),
            }
        } else {
            Box::new(Type::Unknown)
        };

        Ok(Type::Function(TsFunctionSignature {
            type_params,
            this_param: None,
            params,
            return_type,
            is_async: false,
        }))
    }

    pub fn analyze_ts_declare_function_declaration(
        &self,
        node: &TsDeclareFunctionDeclaration,
    ) -> TResult<Type> {
        let mut type_params = vec![];
        if let Some(params) = node.type_parameters() {
            for p in params.items().into_iter().flatten() {
                let param = self.analyze_type_param(&p)?;
                type_params.push(param);
            }
        };

        let mut params = vec![];

        if let Ok(parameters) = node.parameters() {
            params = self.analyze_js_parameters(&parameters)?;
        }

        let return_type = if let Some(ty) = node.return_type_annotation() {
            match ty.ty()? {
                AnyTsReturnType::AnyTsType(ty) => {
                    let ty = self.analyze_any_ts_types(&ty)?;
                    Box::new(ty)
                }
                node => todo!("{:?}", node),
            }
        } else {
            Box::new(Type::Unknown)
        };

        Ok(Type::Function(TsFunctionSignature {
            type_params,
            this_param: None,
            params,
            return_type,
            is_async: false,
        }))
    }
}
