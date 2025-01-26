use biome_js_syntax::{AnyJsObjectMemberName, AnyTsReturnType, AnyTsTypeMember};
use type_info::{TsFunctionSignature, TsInterfaceProperty, TypeInfo};

use crate::{TResult, TypeAnalyzer};

impl TypeAnalyzer {
    pub fn analyze_any_ts_type_member(
        &self,
        node: &AnyTsTypeMember,
    ) -> TResult<TsInterfaceProperty> {
        let ty = match node {
            AnyTsTypeMember::TsPropertySignatureTypeMember(m) => {
                let name = match m.name().unwrap() {
                    AnyJsObjectMemberName::JsLiteralMemberName(member) => {
                        member.value().unwrap().text_trimmed().to_string()
                    }
                    _ => todo!("member name {:?}", m.name()),
                };
                let is_optional = m.optional_token().is_some();
                let is_readonly = m.readonly_token().is_some();
                if let Some(ann) = m.type_annotation() {
                    let type_info = self.analyze_type_annotation(ann);
                    TsInterfaceProperty {
                        name: name.to_string(),
                        type_info,
                        is_optional,
                        is_readonly,
                    }
                } else {
                    todo!()
                }
            }
            AnyTsTypeMember::TsMethodSignatureTypeMember(member) => {
                let name = match member.name()? {
                    AnyJsObjectMemberName::JsLiteralMemberName(literal) => {
                        literal.value().unwrap().text_trimmed().to_string()
                    }
                    node => todo!("{:?}", node),
                };

                let is_optional = member.optional_token().is_some();

                let mut type_params = vec![];
                if let Some(ty_params) = member.type_parameters() {
                    for param in ty_params.items().into_iter().flatten() {
                        let param = self.analyze_type_param(&param)?;
                        type_params.push(param);
                    }
                };

                let mut params = vec![];
                if let Ok(parameter) = member.parameters() {
                    params = self.analyze_js_parameters(&parameter)?;
                }

                let return_type = if let Some(ty) = member.return_type_annotation() {
                    let ret_ty = ty.ty()?;
                    match ret_ty {
                        AnyTsReturnType::AnyTsType(ty) => {
                            let ty = self.analyze_any_ts_types(&ty)?;
                            Box::new(ty)
                        }
                        node => todo!("{:?}", node),
                    }
                } else {
                    Box::new(TypeInfo::Unknown)
                };

                TsInterfaceProperty {
                    name: name.to_string(),
                    type_info: TypeInfo::Function(TsFunctionSignature {
                        type_params,
                        this_param: None,
                        params,
                        return_type,
                        is_async: false,
                    }),
                    is_optional,
                    is_readonly: false,
                }
            }
            _ => todo!("{:?}", node),
        };
        Ok(ty)
    }
}
