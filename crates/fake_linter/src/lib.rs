use biome_js_syntax::*;
use biome_rowan::AstNode;
use biome_rowan::SyntaxNodeCast;
use server::Server;
use std::path::PathBuf;
use type_info::Type;

pub struct NoFloatingPromisesLinter {
    server: Server,
    current_path: PathBuf,
    diagnostics: Vec<String>,
}

const BUILTIN_PROMISE: &str = "Promise";
impl NoFloatingPromisesLinter {
    pub fn new(server: Server) -> Self {
        Self {
            server,
            current_path: PathBuf::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn visit(&mut self, ast: &AnyJsRoot) {
        for node in ast.syntax().preorder() {
            match node {
                WalkEvent::Enter(node) => {
                    if let Some(expr_stmt) = node.cast::<JsExpressionStatement>() {
                        self.check_expression_statement(&expr_stmt);
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }
    }

    fn check_expression_statement(&mut self, expr_stmt: &JsExpressionStatement) {
        if let Ok(expr) = expr_stmt.expression() {
            let is_void = matches!(
                &expr,
                AnyJsExpression::JsUnaryExpression(unary_expr) if unary_expr.operator().map_or(false, |op| op == JsUnaryOperator::Void)
            );

            if !is_void {
                let (is_unhandled, non_function_handler) = self.is_unhandled_promise(&expr);
                if is_unhandled || non_function_handler {
                    let diagnostic = if non_function_handler {
                        format!(
                            "Unhandled Promise with non-function handler at {:?} => {:?}",
                            expr_stmt.syntax().text_range(),
                            expr_stmt.syntax().text_trimmed()
                        )
                    } else {
                        format!(
                            "Unhandled Promise at {:?} => {:?}",
                            expr_stmt.syntax().text_range(),
                            expr_stmt.syntax().text_trimmed()
                        )
                    };
                    self.diagnostics.push(diagnostic);
                }
            }
        }
    }

    fn is_unhandled_promise(&self, expr: &AnyJsExpression) -> (bool, bool) {
        match expr {
            AnyJsExpression::JsCallExpression(call_expr) => {
                if let Ok(AnyJsExpression::JsStaticMemberExpression(member_expr)) =
                    call_expr.callee()
                {
                    if let (Ok(object), Ok(prop)) = (member_expr.object(), member_expr.member()) {
                        let method_name = prop.text();
                        let arguments = call_expr.arguments().ok().unwrap();

                        if method_name == "catch" && arguments.args().into_iter().count() >= 1 {
                            if let AnyJsCallArgument::AnyJsExpression(catch_rejection_handler) =
                                arguments.args().into_iter().next().unwrap().unwrap()
                            {
                                return (
                                    !self.is_valid_rejection_handler(&catch_rejection_handler),
                                    true,
                                );
                            }
                        }

                        if method_name == "then" {
                            let mut args = arguments.args().into_iter();
                            let is_first_arg_valid = args
                                .next()
                                .map(|arg| {
                                    if let Ok(AnyJsCallArgument::AnyJsExpression(expr)) = arg {
                                        self.is_valid_rejection_handler(&expr)
                                    } else {
                                        false
                                    }
                                })
                                .unwrap_or(true);

                            let is_second_arg_valid = args
                                .next()
                                .map(|arg| {
                                    if let Ok(AnyJsCallArgument::AnyJsExpression(expr)) = arg {
                                        self.is_valid_rejection_handler(&expr)
                                    } else {
                                        false
                                    }
                                })
                                .unwrap_or(true);

                            if !is_first_arg_valid || !is_second_arg_valid {
                                return (true, true);
                            }
                        }

                        let object_type = self.infer_expression_type(&object);
                        if is_promise_type(&object_type) {
                            return (true, false);
                        }
                    }
                }
                let type_info = self.infer_expression_type(expr);
                (is_promise_type(&type_info), false)
            }
            AnyJsExpression::JsAwaitExpression(_) => (false, false),
            AnyJsExpression::JsUnaryExpression(unary_expr) => {
                if let Ok(op) = unary_expr.operator() {
                    if op == JsUnaryOperator::Void {
                        return (false, false);
                    }
                }
                let type_info = self.infer_expression_type(expr);
                (is_promise_type(&type_info), false)
            }
            _ => {
                let type_info = self.infer_expression_type(expr);
                (is_promise_type(&type_info), false)
            }
        }
    }

    fn is_valid_rejection_handler(&self, handler: &AnyJsExpression) -> bool {
        match handler {
            AnyJsExpression::JsArrowFunctionExpression(_)
            | AnyJsExpression::JsFunctionExpression(_) => true,
            AnyJsExpression::JsIdentifierExpression(ident) => {
                if let Ok(name) = ident.name() {
                    let symbol_name = name.text().to_string();
                    if symbol_name == "undefined" {
                        return true;
                    }
                    if let Some(symbol) = self.server.get_type_info(symbol_name) {
                        is_call_signature(&symbol.ty)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNullLiteralExpression(_),
            ) => true,
            _ => false,
        }
    }

    fn infer_expression_type(&self, expr: &AnyJsExpression) -> Type {
        match expr {
            AnyJsExpression::JsCallExpression(call_expr) => {
                if let Ok(callee) = call_expr.callee() {
                    self.infer_expression_type(&callee)
                } else {
                    Type::Unknown
                }
            }
            AnyJsExpression::JsStaticMemberExpression(member_expr) => {
                if let Ok(object) = member_expr.object() {
                    self.infer_expression_type(&object)
                } else {
                    Type::Unknown
                }
            }
            AnyJsExpression::JsIdentifierExpression(ident_expr) => {
                if let Ok(name) = ident_expr.name() {
                    let symbol_name = name.text().to_string();
                    dbg!(&symbol_name);
                    if let Some(symbol) = self.server.get_type_info(symbol_name) {
                        symbol.ty.clone()
                    } else {
                        Type::Unknown
                    }
                } else {
                    Type::Unknown
                }
            }
            AnyJsExpression::JsAwaitExpression(await_expr) => {
                if let Ok(argument) = await_expr.argument() {
                    let arg_type = self.infer_expression_type(&argument);
                    if let Type::TypeRef(type_ref) = arg_type {
                        if type_ref.name == BUILTIN_PROMISE && !type_ref.type_params.is_empty() {
                            return type_ref.type_params[0].clone();
                        }
                    }
                }
                Type::Unknown
            }
            _ => Type::Unknown,
        }
    }

    pub fn set_current_path(&mut self, path: PathBuf) {
        self.current_path = path;
    }

    pub fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }
}

fn is_call_signature(type_info: &Type) -> bool {
    match type_info {
        Type::Function(_) => true,
        Type::Union(types) => types.iter().all(is_call_signature),
        _ => false,
    }
}

fn is_promise_type(type_info: &Type) -> bool {
    match type_info {
        Type::Interface(interface) if interface.name == BUILTIN_PROMISE => true,
        Type::TypeRef(type_ref) if type_ref.name == BUILTIN_PROMISE => true,
        Type::Function(func) => is_promise_type(&func.return_type),
        Type::Union(types) => types.iter().any(is_promise_type),
        _ => false,
    }
}
