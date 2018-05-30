use ast::untyped::{UntypedNodeVisitor, UntypedExpr};
use ast::typed::TypedExpr;
use ast::types::{ImpoType, AnyKind, PrimitiveKind};
use ast::nodes::{ExprNode, BinaryOpKind, UnaryOpKind};
use text::TextRange;
use errors::{ImpoError, ErrorStage};

pub struct InferenceEngine {
    pub errors: Vec<ImpoError>,
}

impl InferenceEngine {
    pub fn new() -> InferenceEngine {
        InferenceEngine { errors: Vec::new() }
    }

    fn add_error(&mut self, range: TextRange, description: &'static str) {
        self.errors.push(ImpoError::new(range, ErrorStage::Typechecking, description));
    }
}

impl UntypedNodeVisitor<TypedExpr> for InferenceEngine {
    fn visit_expr(&mut self, expr: &UntypedExpr) -> TypedExpr {
        let id = expr.id;
        let range = expr.range;
        match expr.node {
            ExprNode::NilLiteral => TypedExpr::new(
                id,
                range,
                ExprNode::NilLiteral,
                ImpoType::Nil,
            ),
            ExprNode::BooleanLiteral(v) => TypedExpr::new(
                id,
                range,
                ExprNode::BooleanLiteral(v),
                ImpoType::Primitive(PrimitiveKind::Bool)
            ),
            ExprNode::IntLiteral(v) => TypedExpr::new(
                id,
                range,
                ExprNode::IntLiteral(v),
                ImpoType::Primitive(PrimitiveKind::Int)
            ),
            ExprNode::FloatLiteral(v) => TypedExpr::new(
                id,
                range,
                ExprNode::FloatLiteral(v),
                ImpoType::Primitive(PrimitiveKind::Float)
            ),
            ExprNode::StringLiteral(v) => TypedExpr::new(
                id,
                range,
                ExprNode::StringLiteral(v),
                ImpoType::Primitive(PrimitiveKind::String)
            ),
            ExprNode::Group(ref v) => {
                let inner = self.visit_expr(v);
                let inner_type = inner.typ;
                TypedExpr::new(
                    id,
                    range,
                    ExprNode::Group(Box::new(inner)),
                    inner_type,
                )
            },
            ExprNode::UnaryOp { kind, ref item } => {
                let inner = self.visit_expr(item);
                let output_kind = if kind == UnaryOpKind::NumericNegation {
                    PrimitiveKind::Int
                } else {
                    PrimitiveKind::Bool
                };
                let output_type = if is_any(inner.typ) {
                    ImpoType::Any(AnyKind::Inferred)
                } else {
                    ImpoType::Primitive(output_kind)
                };

                if output_kind == PrimitiveKind::Int && !is_one_of(inner.typ, &vec![&is_int, &is_float, &is_any]) {
                    self.add_error(
                        range,
                        "Attempting to negate something that is not a number.");
                }
                if output_kind == PrimitiveKind::Bool&& !is_one_of(inner.typ, &vec![&is_bool, &is_any]) {
                    self.add_error(
                        range,
                        "Attempting to negate something that is not a bool.");
                }

                TypedExpr::new(
                    id,
                    range,
                    ExprNode::UnaryOp { kind, item: Box::new(inner) },
                    output_type)
            },
            ExprNode::BinaryOp { kind, ref left, ref right} => {
                let inner_left = self.visit_expr(left);
                let inner_right = self.visit_expr(right);

                let left_type = inner_left.typ;
                let right_type = inner_right.typ;

                let output_type = if is_any(left_type) || is_any(right_type) {
                    ImpoType::Any(AnyKind::Inferred)
                } else if is_numeric(left_type) && is_numeric(right_type) && kind == BinaryOpKind::Division {
                    ImpoType::Primitive(PrimitiveKind::Float)
                } else if is_numeric(left_type) && is_numeric(right_type) && kind == BinaryOpKind::Exponentiate {
                    ImpoType::Primitive(PrimitiveKind::Float)
                } else if is_int(left_type) && is_int(right_type) {
                    ImpoType::Primitive(PrimitiveKind::Int)
                } else if is_numeric(left_type) && is_numeric(right_type) {
                    ImpoType::Primitive(PrimitiveKind::Float)
                } else if kind == BinaryOpKind::Addition && is_string(left_type) && is_string(right_type) {
                    ImpoType::Primitive(PrimitiveKind::String)
                } else {
                    self.add_error(range, "Both the LHS and RHS must be numeric or a str");
                    ImpoType::Any(AnyKind::FromError)
                };

                TypedExpr::new(
                    id,
                    range,
                    ExprNode::BinaryOp{ kind, left: Box::new(inner_left), right: Box::new(inner_right) },
                    output_type)
            },
        }
    }
}

fn is_one_of(typ: ImpoType, matchers: &Vec<&Fn(ImpoType) -> bool>) -> bool
{
    matchers.iter().any(|f| f(typ))
}


fn is_bool(typ: ImpoType) -> bool {
    match typ {
        ImpoType::Primitive(PrimitiveKind::Bool) => true,
        _ => false,
    }
}

fn is_int(typ: ImpoType) -> bool {
    match typ {
        ImpoType::Primitive(PrimitiveKind::Int) => true,
        _ => false,
    }
}

fn is_float(typ: ImpoType) -> bool {
    match typ {
        ImpoType::Primitive(PrimitiveKind::Float) => true,
        _ => false,
    }
}

fn is_numeric(typ: ImpoType) -> bool {
    is_int(typ) || is_float(typ)
}

fn is_string(typ: ImpoType) -> bool {
    match typ {
        ImpoType::Primitive(PrimitiveKind::String) => true,
        _ => false,
    }
}

fn is_any(typ: ImpoType) -> bool {
    match typ {
        ImpoType::Any(_) => true,
        _ => false,
    }
}
