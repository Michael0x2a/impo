use ast::nodes::{ExprNode, map_binop_kind_to_str, map_unary_kind_to_str, NodeId};
use ast::types::ImpoType;
use text::TextRange;
use ast::stringpool::ReadOnlyStringPool;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct TypedExpr {
    pub id: NodeId,
    pub range: TextRange,
    pub node: ExprNode<TypedExpr>,
    pub typ: ImpoType,
}

impl TypedExpr {
    pub fn new(id: NodeId, range: TextRange, node: ExprNode<TypedExpr>, typ: ImpoType) -> TypedExpr {
        TypedExpr { id, range, node, typ }
    }
}

pub trait MutatingTypedNodeVisitor<T> {
    fn visit_expr(&mut self, node: &TypedExpr) -> T;
}

pub trait TypedNodeVisitor<T> {
    fn visit_expr(&self, node: &TypedExpr) -> T;
}

pub struct TypedNodeToStrVisitor<'a> {
    pool: &'a ReadOnlyStringPool,
}

impl<'a> TypedNodeToStrVisitor<'a> {
    pub fn new(pool: &'a ReadOnlyStringPool) -> TypedNodeToStrVisitor<'a> {
        TypedNodeToStrVisitor { pool }
    }
}

impl<'a> TypedNodeVisitor<String> for TypedNodeToStrVisitor<'a> {
    fn visit_expr(&self, node: &TypedExpr) -> String {
        let mut out = String::new();
        match node.node {
            ExprNode::BinaryOp { kind, ref left, ref right } => {
                out.push('(');
                out.push_str(&self.visit_expr(left));
                out.push(' ');
                out.push_str(map_binop_kind_to_str(kind));
                out.push(' ');
                out.push_str(&self.visit_expr(right));
                out.push(')');
            },
            ExprNode::UnaryOp { kind, ref item } => {
                out.push_str(map_unary_kind_to_str(kind));
                out.push_str(&self.visit_expr(item));
            },
            ExprNode::Group(ref item) => {
                out.push('(');
                out.push_str(&self.visit_expr(item));
                out.push(')');
            },
            ExprNode::BooleanLiteral(val) => {
                out.push_str(&val.to_string())
            },
            ExprNode::IntLiteral(val) => {
                out.push_str(&val.to_string())
            },
            ExprNode::FloatLiteral(val) => {
                out.push_str(&f64::from_bits(val).to_string());
            },
            ExprNode::StringLiteral(val) => {
                let s = self.pool.lookup(val);
                out.push_str(&format!("{:?}", s));
            },
            ExprNode::NilLiteral => {
                out.push_str("nil");
            },
        }
        out
    }
}

