use biome_js_syntax::{AnyJsLiteralExpression, AnyJsObjectMember, JsObjectExpression};
use type_info::{
    BoolLiteral, ObjectLiteral, ObjectPropertyType, TsKeywordTypeKind, TsLiteralTypeKind, TypeInfo,
};

use crate::{TResult, TypeAnalyzer};

impl TypeAnalyzer {
    pub fn analyze_js_literal_expression(
        &self,
        node: &AnyJsLiteralExpression,
    ) -> TResult<TypeInfo> {
        let ty = match node {
            AnyJsLiteralExpression::JsBooleanLiteralExpression(node) => {
                let value = node.value_token()?;
                match value.text() {
                    "true" => TypeInfo::Literal(TsLiteralTypeKind::Boolean(BoolLiteral::True)),
                    "false" => TypeInfo::Literal(TsLiteralTypeKind::Boolean(BoolLiteral::False)),
                    _ => unreachable!(),
                }
            }
            AnyJsLiteralExpression::JsNumberLiteralExpression(lit) => {
                let value = lit.value_token()?;
                TypeInfo::Literal(TsLiteralTypeKind::Number(value.text().parse().unwrap()))
            }
            AnyJsLiteralExpression::JsStringLiteralExpression(lit) => {
                let value = lit.value_token()?.text().to_string().replace("\'", "");
                TypeInfo::Literal(TsLiteralTypeKind::String(value))
            }

            AnyJsLiteralExpression::JsNullLiteralExpression(_) => {
                TypeInfo::KeywordType(TsKeywordTypeKind::Null)
            }
            _ => todo!("{:?}", node),
        };
        Ok(ty)
    }

    pub fn analyze_js_object_expression(&self, node: &JsObjectExpression) -> TResult<TypeInfo> {
        let mut properties = vec![];
        for prop in node.members() {
            let prop = prop?;
            match prop {
                AnyJsObjectMember::JsPropertyObjectMember(member) => {
                    let key = member.name()?.name().unwrap().to_string();
                    let value = member.value()?;
                    let value_ty = self.analyze_any_js_expression(&value)?;
                    properties.push(ObjectPropertyType {
                        name: key,
                        type_info: value_ty,
                    });
                }
                _ => todo!(),
            }
        }
        Ok(TypeInfo::Literal(TsLiteralTypeKind::Object(
            ObjectLiteral { properties },
        )))
    }
}
