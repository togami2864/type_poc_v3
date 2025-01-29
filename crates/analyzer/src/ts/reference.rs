use biome_js_syntax::{AnyTsName, TsReferenceType};
use type_info::{TsTypeRef, Type};

use crate::{TResult, TypeAnalyzer};

impl TypeAnalyzer {
    pub fn analyze_ts_type_ref(&self, node: &TsReferenceType) -> TResult<Type> {
        let name = match node.name()? {
            AnyTsName::JsReferenceIdentifier(ident) => {
                let value = ident.value_token()?;
                value.text_trimmed().to_string()
            }
            AnyTsName::TsQualifiedName(qual) => {
                todo!("qualified name {:?}", qual)
            }
        };

        let mut type_params = vec![];

        if let Some(args) = node.type_arguments() {
            for arg in args.ts_type_argument_list().into_iter().flatten() {
                let ty = self.analyze_any_ts_types(&arg);
                match ty {
                    Ok(ty) => type_params.push(ty),
                    Err(_) => continue,
                }
            }
        }
        Ok(Type::TypeRef(TsTypeRef {
            name: name.to_owned(),
            type_params,
        }))
    }
}
