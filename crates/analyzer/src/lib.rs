use std::path::PathBuf;

use biome_js_syntax::*;
use type_info::{
    symbol::{Symbol, SymbolTable},
    TsKeywordTypeKind, TsLiteralTypeKind, TypeInfo,
};
use visitor::Visitor;

#[derive(Debug, Default)]
pub struct TypeAnalyzer {
    current_path: PathBuf,
    symbol_table: SymbolTable,
}

impl TypeAnalyzer {
    pub fn new() -> Self {
        Self {
            current_path: PathBuf::new(),
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn insert_new_symbol(&mut self, symbol: Symbol) {
        self.symbol_table.insert(self.current_path.clone(), symbol);
    }

    pub fn analyze_type_annotation(&self, node: TsTypeAnnotation) -> TypeInfo {
        let ty = node.ty().unwrap();
        let ty = self.analyze_ts_types(&ty);
        ty
    }

    pub fn analyze_ts_types(&self, node: &AnyTsType) -> TypeInfo {
        match node {
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
                let value = lit.literal();
                match value {
                    Ok(val) => {
                        TypeInfo::Literal(TsLiteralTypeKind::Boolean(val.text().to_string()))
                    }
                    Err(_) => TypeInfo::Unknown,
                }
            }
            AnyTsType::TsNumberLiteralType(lit) => {
                let value = lit.literal_token();
                match value {
                    Ok(val) => {
                        TypeInfo::Literal(TsLiteralTypeKind::Number(val.text().parse().unwrap()))
                    }
                    Err(_) => TypeInfo::Unknown,
                }
            }

            AnyTsType::TsStringLiteralType(lit) => {
                let value = lit.literal_token();
                match value {
                    Ok(val) => TypeInfo::Literal(TsLiteralTypeKind::String(val.text().to_string())),
                    Err(_) => TypeInfo::Unknown,
                }
            }

            AnyTsType::TsNullLiteralType(_) => {
                TypeInfo::Literal(TsLiteralTypeKind::String("null".to_string()))
            }

            AnyTsType::TsBigintLiteralType(ts_bigint_literal_type) => todo!(),
            AnyTsType::JsMetavariable(js_metavariable) => todo!(),
            AnyTsType::TsAnyType(ts_any_type) => todo!(),
            AnyTsType::TsArrayType(ts_array_type) => todo!(),
            AnyTsType::TsBogusType(ts_bogus_type) => todo!(),
            AnyTsType::TsConditionalType(ts_conditional_type) => todo!(),
            AnyTsType::TsConstructorType(ts_constructor_type) => todo!(),
            AnyTsType::TsFunctionType(ts_function_type) => todo!(),
            AnyTsType::TsImportType(ts_import_type) => todo!(),
            AnyTsType::TsIndexedAccessType(ts_indexed_access_type) => todo!(),
            AnyTsType::TsInferType(ts_infer_type) => todo!(),
            AnyTsType::TsIntersectionType(ts_intersection_type) => todo!(),
            AnyTsType::TsMappedType(ts_mapped_type) => todo!(),
            AnyTsType::TsNonPrimitiveType(ts_non_primitive_type) => todo!(),
            AnyTsType::TsObjectType(ts_object_type) => todo!(),
            AnyTsType::TsParenthesizedType(ts_parenthesized_type) => todo!(),
            AnyTsType::TsReferenceType(ts_reference_type) => todo!(),
            AnyTsType::TsTemplateLiteralType(ts_template_literal_type) => todo!(),
            AnyTsType::TsThisType(ts_this_type) => todo!(),
            AnyTsType::TsTupleType(ts_tuple_type) => todo!(),
            AnyTsType::TsTypeOperatorType(ts_type_operator_type) => todo!(),
            AnyTsType::TsTypeofType(ts_typeof_type) => todo!(),
            AnyTsType::TsUnionType(ts_union_type) => todo!(),
        }
    }

    pub fn analyze_ts_expression(&self, node: &AnyJsExpression) -> TypeInfo {
        match node {
            AnyJsExpression::AnyJsLiteralExpression(expr) => {
                self.analyze_js_literal_expression(expr)
            }
            AnyJsExpression::JsArrayExpression(js_array_expression) => todo!(),
            AnyJsExpression::JsArrowFunctionExpression(js_arrow_function_expression) => todo!(),
            AnyJsExpression::JsAssignmentExpression(js_assignment_expression) => todo!(),
            AnyJsExpression::JsAwaitExpression(js_await_expression) => todo!(),
            AnyJsExpression::JsBinaryExpression(js_binary_expression) => todo!(),
            AnyJsExpression::JsBogusExpression(js_bogus_expression) => todo!(),
            AnyJsExpression::JsCallExpression(js_call_expression) => todo!(),
            AnyJsExpression::JsClassExpression(js_class_expression) => todo!(),
            AnyJsExpression::JsComputedMemberExpression(js_computed_member_expression) => todo!(),
            AnyJsExpression::JsConditionalExpression(js_conditional_expression) => todo!(),
            AnyJsExpression::JsFunctionExpression(js_function_expression) => todo!(),
            AnyJsExpression::JsIdentifierExpression(js_identifier_expression) => todo!(),
            AnyJsExpression::JsImportCallExpression(js_import_call_expression) => todo!(),
            AnyJsExpression::JsImportMetaExpression(js_import_meta_expression) => todo!(),
            AnyJsExpression::JsInExpression(js_in_expression) => todo!(),
            AnyJsExpression::JsInstanceofExpression(js_instanceof_expression) => todo!(),
            AnyJsExpression::JsLogicalExpression(js_logical_expression) => todo!(),
            AnyJsExpression::JsMetavariable(js_metavariable) => todo!(),
            AnyJsExpression::JsNewExpression(js_new_expression) => todo!(),
            AnyJsExpression::JsNewTargetExpression(js_new_target_expression) => todo!(),
            AnyJsExpression::JsObjectExpression(js_object_expression) => todo!(),
            AnyJsExpression::JsParenthesizedExpression(js_parenthesized_expression) => todo!(),
            AnyJsExpression::JsPostUpdateExpression(js_post_update_expression) => todo!(),
            AnyJsExpression::JsPreUpdateExpression(js_pre_update_expression) => todo!(),
            AnyJsExpression::JsSequenceExpression(js_sequence_expression) => todo!(),
            AnyJsExpression::JsStaticMemberExpression(js_static_member_expression) => todo!(),
            AnyJsExpression::JsSuperExpression(js_super_expression) => todo!(),
            AnyJsExpression::JsTemplateExpression(js_template_expression) => todo!(),
            AnyJsExpression::JsThisExpression(js_this_expression) => todo!(),
            AnyJsExpression::JsUnaryExpression(js_unary_expression) => todo!(),
            AnyJsExpression::JsYieldExpression(js_yield_expression) => todo!(),
            AnyJsExpression::JsxTagExpression(jsx_tag_expression) => todo!(),
            AnyJsExpression::TsAsExpression(ts_as_expression) => todo!(),
            AnyJsExpression::TsInstantiationExpression(ts_instantiation_expression) => todo!(),
            AnyJsExpression::TsNonNullAssertionExpression(ts_non_null_assertion_expression) => {
                todo!()
            }
            AnyJsExpression::TsSatisfiesExpression(ts_satisfies_expression) => todo!(),
            AnyJsExpression::TsTypeAssertionExpression(ts_type_assertion_expression) => todo!(),
        }
    }

    pub fn analyze_js_literal_expression(&self, node: &AnyJsLiteralExpression) -> TypeInfo {
        match node {
            AnyJsLiteralExpression::JsBooleanLiteralExpression(node) => {
                let val = node.value_token();
                match val {
                    Ok(val) => {
                        TypeInfo::Literal(TsLiteralTypeKind::Boolean(val.text().to_string()))
                    }
                    Err(_) => TypeInfo::Unknown,
                }
            }
            AnyJsLiteralExpression::JsNumberLiteralExpression(lit) => {
                let val = lit.value_token();
                match val {
                    Ok(val) => {
                        TypeInfo::Literal(TsLiteralTypeKind::Number(val.text().parse().unwrap()))
                    }
                    Err(_) => TypeInfo::Unknown,
                }
            }
            AnyJsLiteralExpression::JsStringLiteralExpression(lit) => {
                let val = lit.value_token();
                match val {
                    Ok(val) => TypeInfo::Literal(TsLiteralTypeKind::String(val.text().to_string())),
                    Err(_) => TypeInfo::Unknown,
                }
            }

            AnyJsLiteralExpression::JsNullLiteralExpression(_) => {
                TypeInfo::KeywordType(TsKeywordTypeKind::Null)
            }

            AnyJsLiteralExpression::JsBigintLiteralExpression(js_bigint_literal_expression) => {
                todo!()
            }

            AnyJsLiteralExpression::JsRegexLiteralExpression(js_regex_literal_expression) => {
                todo!()
            }
        }
    }
}

impl Visitor for TypeAnalyzer {
    fn visit(&mut self, node: &AnyJsRoot) {
        match node {
            AnyJsRoot::JsModule(node) => self.visit_module(node),
            node => todo!("{:?}", node),
        }
    }

    fn visit_module(&mut self, node: &JsModule) {
        for item in node.items() {
            self.visit_module_item(&item);
        }
    }

    fn visit_module_item(&mut self, node: &AnyJsModuleItem) {
        match node {
            AnyJsModuleItem::AnyJsStatement(node) => self.visit_statement(node),
            node => todo!("{:?}", node),
        }
    }

    fn visit_statement(&mut self, node: &AnyJsStatement) {
        match node {
            AnyJsStatement::TsDeclareStatement(node) => {
                self.visit_ts_declare_statement(node);
            }
            node => todo!("{:?}", node),
        }
    }

    fn visit_ts_declare_statement(&mut self, node: &TsDeclareStatement) {
        node.declaration().map(|decl| match decl {
            AnyJsDeclarationClause::JsVariableDeclarationClause(node) => {
                self.visit_js_variable_declaration_clause(&node);
            }
            node => todo!("{:?}", node),
        });
    }

    fn visit_js_variable_declaration_clause(&mut self, node: &JsVariableDeclarationClause) {
        for n in node.declaration() {
            for d in n.declarators() {
                let d = d.unwrap();
                self.visit_js_variable_declarator(&d);
            }
        }
    }

    fn visit_js_variable_declarator(&mut self, node: &JsVariableDeclarator) {
        let id = node.id().unwrap();
        let ann = node.variable_annotation();

        if let Some(ann) = ann {
            match ann {
                AnyTsVariableAnnotation::TsDefiniteVariableAnnotation(node) => {
                    todo!("definite assignment assertion {:?}", node)
                }
                AnyTsVariableAnnotation::TsTypeAnnotation(node) => {
                    let ty = self.analyze_type_annotation(node);
                    let symbol = Symbol::new(id.to_string(), ty);
                    self.insert_new_symbol(symbol);
                }
            }
        } else if let Some(init) = node.initializer() {
            let expr = init.expression().unwrap();
            let ty = self.analyze_ts_expression(&expr);
            let symbol = Symbol::new(id.to_string(), ty);
            self.insert_new_symbol(symbol);
        }
    }
}
