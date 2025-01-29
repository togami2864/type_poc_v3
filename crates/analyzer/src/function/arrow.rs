use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyTsReturnType, JsArrowFunctionExpression,
};
use type_info::{FunctionParam, TsFunctionSignature, Type};

use crate::{TResult, TypeAnalyzer};

impl TypeAnalyzer {
    pub fn analyze_js_arrow_function_expression(
        &self,
        node: &JsArrowFunctionExpression,
    ) -> TResult<Type> {
        let is_async = node.async_token().is_some();
        let mut type_params = vec![];
        if let Some(params) = node.type_parameters() {
            for p in params.items().into_iter().flatten() {
                let param = self.analyze_type_param(&p)?;
                type_params.push(param);
            }
        };

        let mut params = vec![];

        if let Ok(parameters) = node.parameters() {
            match parameters {
                AnyJsArrowFunctionParameters::AnyJsBinding(node) => match node {
                    AnyJsBinding::JsIdentifierBinding(bind) => {
                        let name = bind.name_token().unwrap().text_trimmed().to_string();
                        params.push(FunctionParam {
                            name,
                            is_optional: false,
                            param_type: Type::Unknown,
                        });
                    }
                    _ => todo!("{:?}", node),
                },
                AnyJsArrowFunctionParameters::JsParameters(param) => {
                    params = self.analyze_js_parameters(&param)?;
                }
            }
        }

        let return_type = if let Some(ty) = node.return_type_annotation() {
            let ty = ty.ty()?;
            match ty {
                AnyTsReturnType::AnyTsType(any_ts_type) => {
                    let ty = self.analyze_any_ts_types(&any_ts_type)?;
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
            is_async,
        }))
    }
}
