use crate::ops::{BinOp, UnOp};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Module {
    pub functions: HashMap<String, Function>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Function {
    pub ty: Type,
    pub body: Block,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind")]
pub enum Stmt {
    Break,

    Block(Block),

    Return {
        expr: Option<Box<Expr>>,
    },

    Loop {
        body: Block,
    },

    If {
        cond: Box<Expr>,
        body: Block,

        #[serde(rename = "else")]
        else_: Option<Block>,
    },

    Let {
        name: String,
        ty: Type,
        expr: Option<Box<Expr>>,
    },
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Block {
    pub scope: HashMap<String, Type>,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Expr {
    pub ty: Type,

    #[serde(flatten)]
    pub kind: ExprKind,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind")]
pub enum ExprKind {
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
    Void,
    Bool,
    Number,
}
