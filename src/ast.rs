use crate::ops::{BinOp, UnOp};
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Ast {
    pub functions: Vec<Function>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Function {
    pub name: String,
    pub ty: Option<Type>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind")]
pub enum Stmt {
    Break,

    Return {
        expr: Option<Box<Expr>>,
    },

    Block {
        body: Vec<Stmt>,
    },

    Loop {
        body: Vec<Stmt>,
    },

    If {
        cond: Box<Expr>,
        body: Vec<Stmt>,

        #[serde(rename = "else")]
        else_: Option<Vec<Stmt>>,
    },

    While {
        cond: Box<Expr>,
        body: Vec<Stmt>,
    },

    Let {
        name: String,
        ty: Option<Type>,
        expr: Option<Box<Expr>>,
    },
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind")]
pub enum Expr {
    Bool {
        value: bool,
    },

    Num {
        value: i32,
    },

    Var {
        name: String,
    },

    Unary {
        op: UnOp,
        expr: Box<Expr>,
    },

    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind")]
pub enum Type {
    Simple { name: String },
}
