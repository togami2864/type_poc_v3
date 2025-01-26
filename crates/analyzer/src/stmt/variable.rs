use biome_js_syntax::{AnyTsVariableAnnotation, JsVariableDeclarator};
use type_info::TypeInfo;

use crate::{TResult, TypeAnalyzer};

impl TypeAnalyzer {
    pub fn analyze_js_variable_declarator(&self, node: &JsVariableDeclarator) -> TResult<TypeInfo> {
        let ann = node.variable_annotation();

        let ty = if let Some(ann) = ann {
            match ann {
                AnyTsVariableAnnotation::TsDefiniteVariableAnnotation(node) => {
                    todo!("definite assignment assertion {:?}", node)
                }
                AnyTsVariableAnnotation::TsTypeAnnotation(node) => {
                    self.analyze_type_annotation(node)
                }
            }
        } else if let Some(init) = node.initializer() {
            if let Ok(expr) = init.expression() {
                self.analyze_expression(&expr)
            } else {
                TypeInfo::Unknown
            }
        } else {
            TypeInfo::Unknown
        };
        Ok(ty)
    }
}
