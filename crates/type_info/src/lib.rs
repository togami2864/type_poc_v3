pub mod symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeInfo {
    KeywordType(TsKeywordTypeKind),
    Union(Vec<TypeInfo>),
    Intersection(Vec<TypeInfo>),
    Function(TsFunctionSignature),
    Alias(TsTypeAlias),
    Interface(TsInterface),
    Literal(TsLiteralTypeKind),
    TypeRef(TsTypeRef),
    TypeInstantiation(TsTypeInstantiation),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TsKeywordTypeKind {
    // primitive
    BigInt,
    Boolean,
    Null,
    Number,
    String,
    Symbol,
    Undefined,
    // special
    Any,
    Unknown,
    Never,
    Void,
    Object,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TsLiteralTypeKind {
    Number(i64),
    String(String),
    Boolean(BoolLiteral),
    Object(ObjectLiteral),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoolLiteral {
    True,
    False,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLiteral {
    pub properties: Vec<ObjectPropertyType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectPropertyType {
    pub name: String,
    pub type_info: TypeInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsFunctionSignature {
    pub type_params: Vec<TypeParam>,
    pub this_param: Option<Box<TypeInfo>>,
    pub params: Vec<FunctionParam>,
    pub return_type: Box<TypeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionParam {
    pub name: String,
    pub param_type: TypeInfo,
    pub is_optional: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsInterface {
    pub name: String,
    pub extends: Vec<TypeInfo>,
    pub properties: Vec<TsInterfaceProperty>,
    pub type_params: Vec<TypeParam>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsInterfaceProperty {
    pub name: String,
    pub type_info: TypeInfo,
    pub is_optional: bool,
    pub is_readonly: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsTypeAlias {
    pub name: String,
    pub type_params: Vec<TypeParam>,
    pub aliased_type: Box<TypeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsTypeRef {
    pub name: String,
    pub type_params: Vec<TypeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParam {
    pub name: String,
    pub constraint: Option<TypeInfo>,
    pub default: Option<TypeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TsTypeInstantiation {
    pub base_type: Box<TypeInfo>,
    pub type_args: Vec<TypeInfo>,
}
