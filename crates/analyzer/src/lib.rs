use biome_js_syntax::*;
use visitor::Visitor;

#[derive(Debug)]
pub struct TypeAnalyzer {}

impl Visitor for TypeAnalyzer {
    fn visit(&mut self, node: &AnyJsRoot) {
        todo!()
    }

    fn visit_module(&mut self, node: &JsModule) {
        todo!()
    }

    fn visit_module_item(&mut self, node: &AnyJsModuleItem) {
        todo!()
    }

    fn visit_statement(&mut self, node: &AnyJsStatement) {
        todo!()
    }
}
