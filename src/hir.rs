use std::collections::HashMap;

use serde::Serialize;

use crate::ops::{BinOp, UnOp};

pub mod visitor;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Module {
    pub funs: HashMap<String, Fun>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Fun {
    pub return_ty: Option<Type>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind")]
pub enum Stmt {
    Break,

    Block {
        body: Vec<Stmt>,
    },

    Return {
        expr: Option<Box<Expr>>,
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

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
#[serde(tag = "kind")]
pub enum Type {
    Void,
    Bool,
    Num,
}
