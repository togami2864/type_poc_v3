use biome_js_syntax::{AnyJsModuleItem, AnyJsRoot, AnyJsStatement, JsModule};

pub trait Visitor {
    fn visit(&mut self, node: &AnyJsRoot);

    fn visit_module(&mut self, node: &JsModule);

    fn visit_module_item(&mut self, node: &AnyJsModuleItem);

    fn visit_statement(&mut self, node: &AnyJsStatement);
}
