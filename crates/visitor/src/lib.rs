use biome_js_syntax::*;

pub trait Visitor {
    fn visit(&mut self, node: &AnyJsRoot);

    fn visit_module(&mut self, node: &JsModule);

    fn visit_ts_declaration_module(&mut self, node: &TsDeclarationModule);

    fn visit_module_item(&mut self, node: &AnyJsModuleItem);

    fn visit_statement(&mut self, node: &AnyJsStatement);

    fn visit_js_expression_statement(&mut self, node: &JsExpressionStatement);

    fn visit_js_variable_statement(&mut self, node: &JsVariableStatement);

    fn visit_ts_declare_statement(&mut self, node: &TsDeclareStatement);

    fn visit_js_variable_declaration_clause(&mut self, node: &JsVariableDeclarationClause);

    fn visit_js_variable_declarator(&mut self, node: &JsVariableDeclarator);

    fn visit_ts_interface_declaration(&mut self, node: &TsInterfaceDeclaration);
}
