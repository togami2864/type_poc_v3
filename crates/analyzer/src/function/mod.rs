use biome_js_syntax::{
    AnyJsFormalParameter, AnyJsParameter, AnyTsReturnType, JsFunctionDeclaration, JsParameters,
};
use type_info::{FunctionParam, TsFunctionSignature, TypeInfo};

use crate::{TResult, TypeAnalyzer};

mod arrow;

impl TypeAnalyzer {
    pub fn analyze_js_function_declaration(
        &self,
        node: &JsFunctionDeclaration,
    ) -> TResult<TypeInfo> {
        let is_async = node.async_token().is_some();

        let mut params = vec![];

        if let Ok(param) = node.parameters() {
            params = self.analyze_js_parameters(&param)?;
        }

        let return_type = if let Some(ret_ty) = node.return_type_annotation() {
            let ty = ret_ty.ty()?;
            match ty {
                AnyTsReturnType::AnyTsType(ty) => self.analyze_any_ts_types(&ty)?,
                _ => TypeInfo::Unknown,
            }
        } else {
            TypeInfo::Unknown
        };

        Ok(TypeInfo::Function(TsFunctionSignature {
            //todo
            type_params: vec![],
            this_param: None,
            params,
            return_type: Box::new(return_type),
            is_async,
        }))
    }

    pub fn analyze_js_parameters(&self, params: &JsParameters) -> TResult<Vec<FunctionParam>> {
        let mut result = vec![];
        for p in params.items().into_iter().flatten() {
            match p {
                AnyJsParameter::AnyJsFormalParameter(p) => {
                    match p {
                        AnyJsFormalParameter::JsFormalParameter(p) => {
                            let name = p.binding()?;
                            let is_optional = p.question_mark_token().is_some();
                            let param_type = if let Some(ann) = p.type_annotation() {
                                self.analyze_type_annotation(ann)
                            } else {
                                TypeInfo::Unknown
                            };

                            result.push(FunctionParam {
                                name: name.to_string(),
                                is_optional,
                                param_type,
                            });
                        }
                        AnyJsFormalParameter::JsBogusParameter(_)
                        | AnyJsFormalParameter::JsMetavariable(_) => {
                            unreachable!()
                        }
                    };
                }
                AnyJsParameter::JsRestParameter(_) => todo!(),
                AnyJsParameter::TsThisParameter(_) => todo!(),
            }
        }
        Ok(result)
    }
}
