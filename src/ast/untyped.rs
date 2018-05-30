use ast::nodes::{ExprNode, NodeId};
use text::TextRange;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct UntypedExpr {
    pub id: NodeId,
    pub range: TextRange,
    pub node: ExprNode<UntypedExpr>,
}

impl UntypedExpr {
    pub fn fresh(range: TextRange, node: ExprNode<UntypedExpr>) -> UntypedExpr {
        UntypedExpr { id: NodeId::new(), range, node }
    }

    pub fn new(id: NodeId, range: TextRange, node: ExprNode<UntypedExpr>) -> UntypedExpr {
        UntypedExpr { id, range, node }
    }
}


pub trait UntypedNodeVisitor<T> {
    fn visit_expr(&mut self, node: &UntypedExpr) -> T;
}

