use biome_js_syntax::TsTypeParameter;
use type_info::TypeParam;

use crate::{TResult, TypeAnalyzer};

impl TypeAnalyzer {
    pub fn analyze_type_param(&self, param: &TsTypeParameter) -> TResult<TypeParam> {
        let name = param
            .name()
            .unwrap()
            .ident_token()?
            .text_trimmed()
            .to_string();

        let mut constraint = None;
        let mut default = None;

        if let Some(constraint_clause) = param.constraint() {
            if let Ok(ty) = constraint_clause.ty() {
                constraint = Some(self.analyze_any_ts_types(&ty)?);
            }
        }

        if let Some(default_clause) = param.default() {
            if let Ok(ty) = default_clause.ty() {
                default = Some(self.analyze_any_ts_types(&ty)?);
            }
        }

        Ok(TypeParam {
            name,
            constraint,
            default,
        })
    }
}
