use std::path::PathBuf;

use biome_js_syntax::*;
use biome_rowan::SyntaxError;
use symbol::{Symbol, SymbolTable};
use type_info::*;
use visitor::Visitor;

type TResult<T> = Result<T, SyntaxError>;

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

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbol_table.get(&self.current_path, name)
    }

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

    pub fn analyze_expression(&self, node: &AnyJsExpression) -> TypeInfo {
        let ty = self.analyze_any_js_expression(node);
        match ty {
            Ok(ty) => ty,
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
                let value = lit.literal_token()?.text().replace("\'", "");
                TypeInfo::Literal(TsLiteralTypeKind::Number(value.parse().unwrap()))
            }
            AnyTsType::TsStringLiteralType(lit) => {
                let value = lit.literal_token()?.text().replace("\'", "");
                TypeInfo::Literal(TsLiteralTypeKind::String(value))
            }
            AnyTsType::TsNullLiteralType(_) => TypeInfo::KeywordType(TsKeywordTypeKind::Null),
            _ => todo!("{:?}", node),
        };
        Ok(ty)
    }

    pub fn analyze_any_ts_type_member(
        &self,
        node: &AnyTsTypeMember,
    ) -> TResult<TsInterfaceProperty> {
        let ty = match node {
            AnyTsTypeMember::TsPropertySignatureTypeMember(m) => {
                let name = match m.name().unwrap() {
                    AnyJsObjectMemberName::JsLiteralMemberName(member) => {
                        member.value().unwrap().text_trimmed().to_string()
                    }
                    _ => todo!("member name {:?}", m.name()),
                };
                let is_optional = m.optional_token().is_some();
                let is_readonly = m.readonly_token().is_some();
                if let Some(ann) = m.type_annotation() {
                    let type_info = self.analyze_type_annotation(ann);
                    TsInterfaceProperty {
                        name: name.to_string(),
                        type_info,
                        is_optional,
                        is_readonly,
                    }
                } else {
                    todo!()
                }
            }
            _ => todo!("{:?}", node),
        };
        Ok(ty)
    }

    pub fn analyze_any_js_expression(&self, node: &AnyJsExpression) -> TResult<TypeInfo> {
        let ty = match node {
            AnyJsExpression::AnyJsLiteralExpression(expr) => {
                self.analyze_js_literal_expression(expr)?
            }
            AnyJsExpression::JsObjectExpression(node) => self.analyze_js_object_expression(node)?,
            _ => todo!("{:?}", node),
        };
        Ok(ty)
    }

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
            AnyJsStatement::TsInterfaceDeclaration(node) => {
                self.visit_ts_interface_declaration(node);
            }
            AnyJsStatement::JsVariableStatement(node) => {
                self.visit_js_variable_statement(node);
            }
            node => todo!("{:?}", node),
        }
    }

    fn visit_js_variable_statement(&mut self, node: &JsVariableStatement) {
        if let Ok(list) = node.declaration() {
            for decl in list.declarators().into_iter().flatten() {
                self.visit_js_variable_declarator(&decl);
            }
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

    fn visit_ts_interface_declaration(&mut self, node: &TsInterfaceDeclaration) {
        let interface_name = match node.id().unwrap() {
            AnyTsIdentifierBinding::TsIdentifierBinding(bind) => {
                bind.name_token().unwrap().text_trimmed().to_string()
            }
            _ => todo!(),
        };

        let members = node.members();
        let mut properties = vec![];
        for m in members {
            let ty = match self.analyze_any_ts_type_member(&m) {
                Ok(ty) => ty,
                Err(_) => continue,
            };
            properties.push(ty);
        }

        let ty = TsInterface {
            name: interface_name.to_string(),
            extends: vec![],
            properties,
        };
        let symbol = Symbol::new(
            interface_name.to_string(),
            type_info::TypeInfo::Interface(ty),
        );
        self.insert_new_symbol(symbol);
    }

    fn visit_js_variable_declaration_clause(&mut self, node: &JsVariableDeclarationClause) {
        if let Ok(decl) = node.declaration() {
            for d in decl.declarators().into_iter().flatten() {
                self.visit_js_variable_declarator(&d)
            }
        }
    }

    fn visit_js_variable_declarator(&mut self, node: &JsVariableDeclarator) {
        let id = match node.id() {
            Ok(node) => match node {
                AnyJsBindingPattern::AnyJsBinding(node) => match node {
                    AnyJsBinding::JsIdentifierBinding(bind) => {
                        bind.name_token().unwrap().text_trimmed().to_string()
                    }
                    _ => todo!(),
                },
                AnyJsBindingPattern::JsArrayBindingPattern(node) => {
                    todo!("array binding pattern {:?}", node)
                }
                AnyJsBindingPattern::JsObjectBindingPattern(node) => {
                    todo!("object binding pattern {:?}", node)
                }
            },
            Err(_) => todo!(),
        };
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
            if let Ok(expr) = init.expression() {
                let ty = self.analyze_expression(&expr);
                let symbol = Symbol::new(id.to_string(), ty);
                self.insert_new_symbol(symbol);
            }
        }
    }
}
