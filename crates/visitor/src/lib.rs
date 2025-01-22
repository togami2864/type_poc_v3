use biome_js_syntax::{AnyJsModuleItem, AnyJsRoot, AnyJsStatement, JsModule};

pub trait Visitor {
    type Output;
    type Error;

    fn visit(&mut self, node: &AnyJsRoot) -> Result<Self::Output, Self::Error>;

    fn visit_module(&mut self, node: &JsModule) -> Result<Self::Output, Self::Error>;

    fn visit_module_item(&mut self, node: &AnyJsModuleItem) -> Result<Self::Output, Self::Error>;

    fn visit_statement(&mut self, node: &AnyJsStatement) -> Result<Self::Output, Self::Error>;
}
