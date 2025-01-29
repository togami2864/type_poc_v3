use biome_js_syntax::AnyJsExpression;
use type_info::TypeInfo;

use crate::{TResult, TypeAnalyzer};

mod literal;

impl TypeAnalyzer {
    pub fn analyze_expression(&self, node: &AnyJsExpression) -> TypeInfo {
        let ty = self.analyze_any_js_expression(node);
        match ty {
            Ok(ty) => ty,
            Err(_) => TypeInfo::Unknown,
        }
    }

    pub fn analyze_any_js_expression(&self, node: &AnyJsExpression) -> TResult<TypeInfo> {
        let ty = match node {
            AnyJsExpression::AnyJsLiteralExpression(expr) => {
                self.analyze_js_literal_expression(expr)?
            }
            AnyJsExpression::JsObjectExpression(node) => self.analyze_js_object_expression(node)?,
            AnyJsExpression::JsArrowFunctionExpression(node) => {
                self.analyze_js_arrow_function_expression(node)?
            }
            AnyJsExpression::JsFunctionExpression(node) => TypeInfo::Unknown,
            _ => todo!("{:?}", node),
        };
        Ok(ty)
    }
}
