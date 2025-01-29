use biome_js_syntax::{AnyTsType, TsTypeAnnotation};
use type_info::{BoolLiteral, TsKeywordTypeKind, TsLiteralTypeKind, Type};

use crate::{TResult, TypeAnalyzer};

impl TypeAnalyzer {
    pub fn analyze_type_annotation(&self, node: TsTypeAnnotation) -> Type {
        match node.ty() {
            Ok(ty) => {
                let ty = self.analyze_any_ts_types(&ty);
                match ty {
                    Ok(ty) => ty,
                    Err(_) => Type::Unknown,
                }
            }
            Err(_) => Type::Unknown,
        }
    }

    pub fn analyze_any_ts_types(&self, node: &AnyTsType) -> TResult<Type> {
        let ty = match node {
            AnyTsType::TsAnyType(_) => Type::KeywordType(TsKeywordTypeKind::Any),
            AnyTsType::TsBigintType(_) => Type::KeywordType(TsKeywordTypeKind::BigInt),
            AnyTsType::TsBooleanType(_) => Type::KeywordType(TsKeywordTypeKind::Boolean),
            AnyTsType::TsNeverType(_) => Type::KeywordType(TsKeywordTypeKind::Never),
            AnyTsType::TsNumberType(_) => Type::KeywordType(TsKeywordTypeKind::Number),
            AnyTsType::TsStringType(_) => Type::KeywordType(TsKeywordTypeKind::String),
            AnyTsType::TsSymbolType(_) => Type::KeywordType(TsKeywordTypeKind::Symbol),
            AnyTsType::TsUndefinedType(_) => Type::KeywordType(TsKeywordTypeKind::Undefined),
            AnyTsType::TsVoidType(_) => Type::KeywordType(TsKeywordTypeKind::Void),
            AnyTsType::TsUnknownType(_) => Type::KeywordType(TsKeywordTypeKind::Unknown),

            AnyTsType::TsBooleanLiteralType(lit) => {
                let literal = lit.literal()?;
                let value = literal.text_trimmed();
                match value {
                    "true" => Type::Literal(TsLiteralTypeKind::Boolean(BoolLiteral::True)),
                    "false" => Type::Literal(TsLiteralTypeKind::Boolean(BoolLiteral::False)),
                    _ => unreachable!(),
                }
            }
            AnyTsType::TsNumberLiteralType(lit) => {
                let value = lit.literal_token()?.text_trimmed().to_string();
                dbg!(&value);
                Type::Literal(TsLiteralTypeKind::Number(value.parse().unwrap()))
            }
            AnyTsType::TsStringLiteralType(lit) => {
                let value = lit.literal_token()?.text_trimmed().to_string();
                Type::Literal(TsLiteralTypeKind::String(value))
            }
            AnyTsType::TsNullLiteralType(_) => Type::KeywordType(TsKeywordTypeKind::Null),

            AnyTsType::TsReferenceType(ref_type) => self.analyze_ts_type_ref(ref_type)?,
            AnyTsType::TsUnionType(union) => {
                let mut types = vec![];
                for ty in union.types().into_iter().flatten() {
                    let t = self.analyze_any_ts_types(&ty)?;
                    types.push(t);
                }
                Type::Union(types)
            }
            AnyTsType::TsParenthesizedType(ty) => {
                let inner = ty.ty()?;
                self.analyze_any_ts_types(&inner)?
            }
            AnyTsType::TsFunctionType(func) => self.analyze_ts_function_type(func)?,
            _ => todo!("{:?}", node),
        };
        Ok(ty)
    }
}
