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
    Boolean(bool),
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
    pub properties: Vec<InterfaceProperty>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceProperty {
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
    pub type_args: Vec<TypeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParam {
    pub name: String,
    pub constraint: Option<TypeInfo>,
}
