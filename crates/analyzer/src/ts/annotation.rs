use biome_js_syntax::{AnyTsType, TsTypeAnnotation};
use type_info::{BoolLiteral, TsKeywordTypeKind, TsLiteralTypeKind, TypeInfo};

use crate::{TResult, TypeAnalyzer};

impl TypeAnalyzer {
    pub fn analyze_type_annotation(&self, node: TsTypeAnnotation) -> TypeInfo {
        match node.ty() {
            Ok(ty) => {
                let ty = self.analyze_any_ts_types(&ty);
                match ty {
                    Ok(ty) => ty,
                    Err(_) => TypeInfo::Unknown,
                }
            }
            Err(_) => TypeInfo::Unknown,
        }
    }

    pub fn analyze_any_ts_types(&self, node: &AnyTsType) -> TResult<TypeInfo> {
        let ty = match node {
            AnyTsType::TsAnyType(_) => TypeInfo::KeywordType(TsKeywordTypeKind::Any),
            AnyTsType::TsBigintType(_) => TypeInfo::KeywordType(TsKeywordTypeKind::BigInt),
            AnyTsType::TsBooleanType(_) => TypeInfo::KeywordType(TsKeywordTypeKind::Boolean),
            AnyTsType::TsNeverType(_) => TypeInfo::KeywordType(TsKeywordTypeKind::Never),
            AnyTsType::TsNumberType(_) => TypeInfo::KeywordType(TsKeywordTypeKind::Number),
            AnyTsType::TsStringType(_) => TypeInfo::KeywordType(TsKeywordTypeKind::String),
            AnyTsType::TsSymbolType(_) => TypeInfo::KeywordType(TsKeywordTypeKind::Symbol),
            AnyTsType::TsUndefinedType(_) => TypeInfo::KeywordType(TsKeywordTypeKind::Undefined),
            AnyTsType::TsVoidType(_) => TypeInfo::KeywordType(TsKeywordTypeKind::Void),
            AnyTsType::TsUnknownType(_) => TypeInfo::KeywordType(TsKeywordTypeKind::Unknown),

            AnyTsType::TsBooleanLiteralType(lit) => {
                let literal = lit.literal()?;
                let value = literal.text_trimmed();
                match value {
                    "true" => TypeInfo::Literal(TsLiteralTypeKind::Boolean(BoolLiteral::True)),
                    "false" => TypeInfo::Literal(TsLiteralTypeKind::Boolean(BoolLiteral::False)),
                    _ => unreachable!(),
                }
            }
            AnyTsType::TsNumberLiteralType(lit) => {
                let value = lit.literal_token()?.text_trimmed().to_string();
                dbg!(&value);
                TypeInfo::Literal(TsLiteralTypeKind::Number(value.parse().unwrap()))
            }
            AnyTsType::TsStringLiteralType(lit) => {
                let value = lit.literal_token()?.text_trimmed().to_string();
                TypeInfo::Literal(TsLiteralTypeKind::String(value))
            }
            AnyTsType::TsNullLiteralType(_) => TypeInfo::KeywordType(TsKeywordTypeKind::Null),

            AnyTsType::TsReferenceType(ref_type) => self.analyze_ts_type_ref(ref_type)?,
            AnyTsType::TsUnionType(union) => {
                let mut types = vec![];
                for ty in union.types().into_iter().flatten() {
                    let t = self.analyze_any_ts_types(&ty)?;
                    types.push(t);
                }
                TypeInfo::Union(types)
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
