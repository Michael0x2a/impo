use text::TextRange;
use ast::nodes::{ExprNode, UnaryOpKind, BinaryOpKind};
use ast::typed::{MutatingTypedNodeVisitor, TypedExpr};
use ast::stringpool::StringPool;
use interpreter::values::Value;
use errors::{ImpoError, ErrorStage};

fn error<T>(range: TextRange, description: &'static str) -> Result<T, ImpoError> {
    Err(ImpoError::new(range, ErrorStage::Interpreting, description))
}

macro_rules! numeric_op {
    ($range:expr, $left:expr, $right:expr, $op:tt) => {
        coerce_op($range, $left, $right, &|a, b| a $op b, &|a, b| a $op b)
    };
}

fn coerce_op(range: TextRange,
             left: &Value,
             right: &Value,
             int_op: &Fn(i64, i64) -> i64,
             float_op: &Fn(f64, f64) -> f64) -> Result<Value, ImpoError> {
    if let Value::Int(li) = left {
        if let Value::Int(ri) = right {
            return Ok(Value::Int(int_op(*li, *ri)));
        }
    }
    let li = coerce_to_float(range, left)?;
    let ri = coerce_to_float(range, right)?;
    Ok(Value::Float(float_op(li, ri).to_bits()))
}

fn coerce_to_float(range: TextRange, val: &Value) -> Result<f64, ImpoError> {
    if let Value::Int(i) = val {
        Ok(*i as f64)
    } else if let Value::Float(i) = val {
        Ok(f64::from_bits(*i))
    } else {
        error(range, "Cannot coerce left operand to float")
    }
}


pub struct Interpreter<'a> {
    pool: &'a mut StringPool,
}

impl<'a> Interpreter<'a> {
    pub fn new(pool: &mut StringPool) -> Interpreter {
        Interpreter { pool }
    }

    pub fn display(&self, value: &Value) -> String {
        match *value {
            Value::Nil => "nil".to_owned(),
            Value::Bool(v) => v.to_string(),
            Value::Int(v) => v.to_string(),
            Value::Float(v) => f64::from_bits(v).to_string(),
            Value::String(id) => self.pool.lookup(id).to_owned(),
        }
    }
}

impl<'a> MutatingTypedNodeVisitor<Result<Value, ImpoError>> for Interpreter<'a> {
    fn visit_expr(&mut self, expr: &TypedExpr) -> Result<Value, ImpoError> {
        let range = expr.range;
        match expr.node {
            ExprNode::NilLiteral => Ok(Value::Nil),
            ExprNode::StringLiteral(id) => Ok(Value::String(id)),
            ExprNode::FloatLiteral(num) => Ok(Value::Float(num)),
            ExprNode::IntLiteral(num) => Ok(Value::Int(num)),
            ExprNode::BooleanLiteral(val) => Ok(Value::Bool(val)),
            ExprNode::Group(ref item) => self.visit_expr(item),
            ExprNode::UnaryOp { kind, ref item } => {
                let inner_val = self.visit_expr(item)?;
                match inner_val {
                    Value::Nil => {
                        error(range, "Cannot negate nil")
                    },
                    Value::String(_) => {
                        error(range, "Cannot negate a string")
                    },
                    Value::Int(i) => {
                        if kind != UnaryOpKind::NumericNegation {
                            error(range, "Attempting to boolean negate an int")
                        } else {
                            Ok(Value::Int(-i))
                        }
                    },
                    Value::Float(f) => {
                        if kind != UnaryOpKind::NumericNegation {
                            error(range, "Attempting to boolean negate a float")
                        } else {
                            let val = f64::from_bits(f);
                            Ok(Value::Float((-val).to_bits()))
                        }
                    },
                    Value::Bool(b) => {
                        if kind != UnaryOpKind::BooleanNegation {
                            error(range, "Attempting to numerically negate a bool")
                        } else {
                            Ok(Value::Bool(!b))
                        }
                    }
                }
            },
            ExprNode::BinaryOp { kind, ref left, ref right } => {
                let left = self.visit_expr(left)?;
                let right = self.visit_expr(right)?;

                match kind {
                    BinaryOpKind::Addition => {
                        match (left, right) {
                            (Value::String(s1), Value::String(s2)) => {
                                let mut concat = String::new();
                                concat.push_str(self.pool.lookup(s1));
                                concat.push_str(self.pool.lookup(s2));
                                Ok(Value::String(self.pool.add(concat)))
                            },
                            _ => {
                                numeric_op!(range, &left, &right, +)
                            }
                        }
                    },
                    BinaryOpKind::Subtraction => numeric_op!(range, &left, &right, -),
                    BinaryOpKind::Multiplication => numeric_op!(range, &left, &right, *),
                    BinaryOpKind::Mod => numeric_op!(range, &left, &right, %),
                    BinaryOpKind::Division => {
                        let li = coerce_to_float(range, &left)?;
                        let ri = coerce_to_float(range, &right)?;
                        Ok(Value::Float((li / ri).to_bits()))
                    },
                    BinaryOpKind::FloorDivision => coerce_op(
                        range,
                        &left,
                        &right,
                        &|a, b| a / b,
                        &|a, b| (a / b).trunc(),
                    ),
                    BinaryOpKind::Exponentiate => {
                        let li = coerce_to_float(range, &left)?;
                        let ri = coerce_to_float(range, &right)?;
                        Ok(Value::Float((li.powf(ri)).to_bits()))
                    },
                    _ => panic!("Not yet implemented"),
                }
            }
        }
    }
}
