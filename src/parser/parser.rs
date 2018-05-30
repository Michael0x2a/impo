use std::iter::Peekable;
use std::str::FromStr;

use parser::tokens::{Token, TokenType, PLACEHOLDER_TOKEN};
use ast::untyped::UntypedExpr;
use ast::nodes::{ExprNode,
                 map_token_type_to_binop_kind,
                 map_token_type_to_unary_kind};
use ast::stringpool::StringPool;
use text::{TextOffset, TextRange};
use errors::{ImpoError, ErrorStage};

pub struct Parser<'a, I: Iterator<Item=&'a Token>> {
    tokens: Peekable<I>,
    pool: &'a mut StringPool,
    context: Vec<TextOffset>,
    current: &'a Token,
}

impl<'a, I: Iterator<Item=&'a Token>> Parser<'a, I> {
    pub fn new(tokens: I, pool: &'a mut StringPool) -> Parser<'a, I> {
        Parser {
            tokens: tokens.peekable(),
            pool: pool,
            context: vec![PLACEHOLDER_TOKEN.location.start()],
            current: &PLACEHOLDER_TOKEN,
        }
    }

    pub fn parse_as_line(&mut self) -> Result<UntypedExpr, ImpoError> {
        let out = self.expr()?;
        while self.matches(&|x| x.token_type == TokenType::Newline || x.token_type == TokenType::Eof) {
            self.consume()?;
        }
        match self.tokens.peek() {
            Some(_) => Err(self.make_error("Expression unexpectedly continues")),
            None => Ok(out)
        }
    }

    pub fn expr(&mut self) -> Result<UntypedExpr, ImpoError> {
        self.equality()
    }

    fn get_range(&self) -> TextRange {
        TextRange::new_absolute(
            *self.context.last().expect("Context unexpectedly empty"),
            self.current.location.end())
    }

    fn make_error(&self, message: &str) -> ImpoError {
        ImpoError::new(self.get_range(), ErrorStage::Parsing, message)
    }

    fn push_context(&mut self, t: &UntypedExpr) {
        self.context.push(t.range.start());
    }

    fn pop_context(&mut self) {
        self.context.pop().expect("Context unexpectedly empty");
    }

    fn consume(&mut self) -> Result<(), ImpoError> {
        match self.advance() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn advance(&mut self) -> Result<&Token, ImpoError> {
        match self.tokens.next() {
            Some(t) => {
                self.current = t;
                Ok(&self.current)
            },
            None => Err(self.make_error("Unexpectedly could not advance.")),
        }
    }

    fn matches<F>(&mut self, pred: &F) -> bool
        where F: Fn(&Token) -> bool
    {
        let out = match self.tokens.peek() {
            Some(tok) if pred(tok) => true,
            _ => false,
        };
        if out {
            self.advance().unwrap();
        }
        out
    }

    fn equality(&mut self) -> Result<UntypedExpr, ImpoError> {
        self.abstract_binop_match(Parser::comparison, Token::is_equality_comp)
    }

    fn comparison(&mut self) -> Result<UntypedExpr, ImpoError> {
        self.abstract_binop_match(Parser::arithmetic, Token::is_order_comp)
    }

    fn arithmetic(&mut self) -> Result<UntypedExpr, ImpoError> {
        self.abstract_binop_match(Parser::multiplication, Token::is_arithmetic_comp)
    }

    fn multiplication(&mut self) -> Result<UntypedExpr, ImpoError> {
        self.abstract_binop_match(Parser::unary, Token::is_multiplicative_comp)
    }

    fn unary(&mut self) -> Result<UntypedExpr, ImpoError> {
        if self.matches(&Token::is_unary_op) {
            self.context.push(self.current.location.start());
            let kind = map_token_type_to_unary_kind(self.current.token_type);
            let right = self.unary()?;
            let out = UntypedExpr::fresh(
                self.get_range(),
                ExprNode::UnaryOp {
                    kind: kind,
                    item: Box::new(right),
                });
            self.pop_context();
            Ok(out)
        } else {
             self.exponentiation()
        }
    }

    fn exponentiation(&mut self) -> Result<UntypedExpr, ImpoError> {
        self.abstract_binop_match(Parser::primary, Token::is_exponentiation)
    }

    fn primary(&mut self) -> Result<UntypedExpr, ImpoError> {
        if self.matches(&|t| t.token_type == TokenType::False) {
            Ok(UntypedExpr::fresh(
                self.current.location,
                ExprNode::BooleanLiteral(false),
            ))
        } else if self.matches(&|t| t.token_type == TokenType::True) {
            Ok(UntypedExpr::fresh(
                self.current.location,
                ExprNode::BooleanLiteral(true),
            ))
        } else if self.matches(&|t| t.token_type == TokenType::IntLiteral) {
            let value = self.current.lexeme
                .as_ref()
                .map(|x| x.as_ref())
                .map(i64::from_str)
                .map(|x| x.unwrap())
                .unwrap();

            Ok(UntypedExpr::fresh(
                self.current.location,
                ExprNode::IntLiteral(value)))
        } else if self.matches(&|t| t.token_type == TokenType::FloatLiteral) {
            let value = self.current.lexeme
                .as_ref()
                .map(|x| x.as_ref())
                .map(f64::from_str)
                .map(|x| x.unwrap())
                .map(f64::to_bits)
                .unwrap();
            Ok(UntypedExpr::fresh(
                self.current.location,
                ExprNode::FloatLiteral(value)))
        } else if self.matches(&|t| t.token_type == TokenType::Nil) {
            Ok(UntypedExpr::fresh(
                self.current.location,
                ExprNode::NilLiteral,
            ))
        } else if self.matches(&|t| t.token_type == TokenType::StringLiteral) {
            let id = self.pool.add(self.current.lexeme.as_ref().unwrap().clone());
            Ok(UntypedExpr::fresh(
                self.current.location,
                ExprNode::StringLiteral(id),
            ))
        } else if self.matches(&|t| t.token_type == TokenType::LeftParen) {
            self.context.push(self.current.location.start());
            let expr = self.expr()?;
            if self.matches(&|t| t.token_type == TokenType::RightParen) {
                Ok(UntypedExpr::fresh(
                    self.get_range(),
                    ExprNode::Group(Box::new(expr)),
                ))
            } else {
                self.consume()?;
                Err(self.make_error(&format!("Expected closing paren ')'; encountered {:?}", self.current.token_type)))
            }
        } else {
            self.consume()?;
            Err(self.make_error(&format!("Unexpected token when parsing expression: {:?}", self.current.token_type)))
        }
    }

    fn abstract_binop_match<FNext, FPred>(&mut self, mut next: FNext, pred: FPred) -> Result<UntypedExpr, ImpoError>
        where FNext: FnMut(&mut Self) -> Result<UntypedExpr, ImpoError>,
              FPred: Fn(&Token) -> bool {
        let mut expr = next(self)?;
        self.push_context(&expr);

        while self.matches(&pred) {
            let kind = map_token_type_to_binop_kind(self.current.token_type);
            let right = next(self)?;
            expr = UntypedExpr::fresh(
                self.get_range(),
                ExprNode::BinaryOp {
                    kind: kind,
                    left: Box::new(expr),
                    right: Box::new(right)
                });
        }
        self.pop_context();
        Ok(expr)
    }
}


