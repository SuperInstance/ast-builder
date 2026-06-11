use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int, Float, Bool, String, Void,
    Custom(String),
    Array(Box<Type>),
    Optional(Box<Type>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    Binary { left: Box<Expr>, op: BinOp, right: Box<Expr> },
    Unary { op: UnOp, operand: Box<Expr> },
    Call { callee: Box<Expr>, args: Vec<Expr> },
    Member { object: Box<Expr>, field: String },
    Index { object: Box<Expr>, index: Box<Expr> },
    Lambda { params: Vec<Param>, body: Box<Expr> },
    If { cond: Box<Expr>, then: Box<Expr>, else_: Option<Box<Expr>> },
    Match { scrutinee: Box<Expr>, arms: Vec<(Pattern, Expr)> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal { Int(i64), Float(f64), Bool(bool), Str(String), Null }

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp { Add, Sub, Mul, Div, Mod, Eq, Ne, Lt, Gt, Le, Ge, And, Or }

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp { Neg, Not }

#[derive(Debug, Clone, PartialEq)]
pub struct Param { pub name: String, pub ty: Option<Type>, pub default: Option<Expr> }

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern { Wildcard, Ident(String), Literal(Literal), Destructure(Vec<(String, Pattern)>), Or(Vec<Pattern>) }

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let { pattern: Pattern, ty: Option<Type>, init: Option<Expr> },
    Expr(Expr),
    Return(Option<Expr>),
    Block(Vec<Stmt>),
    If { cond: Expr, then: Vec<Stmt>, else_: Option<Vec<Stmt>> },
    While { cond: Expr, body: Vec<Stmt> },
    For { pattern: Pattern, iter: Expr, body: Vec<Stmt> },
    Func { name: String, params: Vec<Param>, ret: Option<Type>, body: Vec<Stmt> },
    Struct { name: String, fields: Vec<(String, Type)> },
    Enum { name: String, variants: Vec<(String, Vec<Type>)> },
    Impl { target: String, methods: Vec<Box<Stmt>> },
}

pub struct AstBuilder { stmts: Vec<Stmt> }

impl AstBuilder {
    pub fn new() -> Self { Self { stmts: Vec::new() } }
    pub fn push(&mut self, stmt: Stmt) { self.stmts.push(stmt); }
    pub fn build(self) -> Vec<Stmt> { self.stmts }

    pub fn let_stmt(name: &str, init: Option<Expr>) -> Stmt {
        Stmt::Let { pattern: Pattern::Ident(name.to_string()), ty: None, init }
    }

    pub fn func(name: &str, params: Vec<Param>, body: Vec<Stmt>) -> Stmt {
        Stmt::Func { name: name.to_string(), params, ret: None, body }
    }

    pub fn binary(left: Expr, op: BinOp, right: Expr) -> Expr {
        Expr::Binary { left: Box::new(left), op, right: Box::new(right) }
    }

    pub fn call(callee: Expr, args: Vec<Expr>) -> Expr {
        Expr::Call { callee: Box::new(callee), args }
    }

    pub fn var(name: &str) -> Expr { Expr::Variable(name.to_string()) }
    pub fn int_lit(n: i64) -> Expr { Expr::Literal(Literal::Int(n)) }
    pub fn str_lit(s: &str) -> Expr { Expr::Literal(Literal::Str(s.to_string())) }
}

impl Default for AstBuilder { fn default() -> Self { Self::new() } }
