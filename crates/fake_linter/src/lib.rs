use biome_js_syntax::*;
use biome_rowan::AstNode;
use biome_rowan::SyntaxNodeCast;
use server::Server;
use std::path::PathBuf;
use type_info::TypeInfo;

pub struct NoFloatingPromisesLinter {
    server: Server,
    current_path: PathBuf,
    diagnostics: Vec<String>,
}

const KNOWN_BUILTIN_PROMISE: &str = "Promise";

impl NoFloatingPromisesLinter {
    pub fn new(server: Server) -> Self {
        Self {
            server,
            current_path: PathBuf::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn visit(&mut self, ast: &AnyJsRoot) {
        if let AnyJsRoot::JsModule(module) = ast {
            for item in module.items() {
                if let Some(stmt) = item.as_any_js_statement() {
                    if let Some(expr) = stmt.as_js_expression_statement() {
                        self.check_expression(&expr.expression().unwrap());
                    }
                }
            }
        }
    }

    pub fn set_current_path(&mut self, path: PathBuf) {
        self.current_path = path;
    }

    pub fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }

    fn is_promise_type(&self, type_info: &TypeInfo) -> bool {
        match type_info {
            TypeInfo::TypeRef(type_ref) if type_ref.name == KNOWN_BUILTIN_PROMISE => true,
            TypeInfo::Function(func) => self.is_promise_type(&func.return_type),
            _ => false,
        }
    }

    fn is_handled_promise(&self, node: &JsSyntaxNode) -> bool {
        match node.kind() {
            JsSyntaxKind::JS_AWAIT_EXPRESSION => true,
            JsSyntaxKind::JS_CALL_EXPRESSION => {
                if let Some(member_expr) = node.first_child() {
                    let obj = node.clone().cast::<JsCallExpression>().unwrap();
                    let callee = obj.callee().unwrap();
                    let object_type = self.infer_expression_type(&callee);
                    if self.is_promise_type(&object_type) {
                        if let Some(prop_name) = member_expr.last_child() {
                            let method_name = prop_name.text();
                            return method_name == "then" || method_name == "catch";
                        }
                    }
                }
                false
            }
            JsSyntaxKind::JS_UNARY_EXPRESSION => node.first_child().is_some(),
            _ => false,
        }
    }

    fn infer_expression_type(&self, node: &AnyJsExpression) -> TypeInfo {
        match node {
            AnyJsExpression::JsIdentifierExpression(expr) => {
                let id = expr.name().unwrap().text();
                if let Some(symbol) = self.server.get_resolved_type_info(id) {
                    symbol.ty.clone()
                } else {
                    TypeInfo::Unknown
                }
            }
            node => todo!("{}", node),
        }
    }

    fn check_expression(&mut self, expr: &AnyJsExpression) {
        if let AnyJsExpression::JsCallExpression(call_expr) = expr {
            self.check_call_expression(call_expr)
        }
    }

    fn check_call_expression(&mut self, call_expr: &JsCallExpression) {
        let type_info = self.infer_expression_type(&call_expr.callee().unwrap());
        if self.is_promise_type(&type_info) && !self.is_handled_promise(call_expr.syntax()) {
            let diagnostic = format!("Unhandled Promise at {:?}", call_expr.syntax().text_range());
            self.diagnostics.push(diagnostic);
        }
    }
}
