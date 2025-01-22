use biome_js_syntax::*;
use visitor::Visitor;

#[derive(Debug)]
pub struct TypeAnalyzer {}

// impl Visitor for TypeAnalyzer {
//     type Output;

//     type Error;

//     fn visit(&mut self, node: &AnyJsRoot) -> Result<Self::Output, Self::Error> {
//         todo!()
//     }

//     fn visit_module(&mut self, node: &JsModule) -> Result<Self::Output, Self::Error> {
//         todo!()
//     }

//     fn visit_module_item(&mut self, node: &AnyJsModuleItem) -> Result<Self::Output, Self::Error> {
//         todo!()
//     }

//     fn visit_statement(&mut self, node: &AnyJsStatement) -> Result<Self::Output, Self::Error> {
//         todo!()
//     }
// }
