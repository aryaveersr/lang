use serde::Serialize;
use std::{collections::HashMap, fmt::Display};

use crate::ops::{BinOp, UnOp};

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct HirModule {
    pub funs: HashMap<String, HirFun>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct HirFun {
    pub ty: HirFunType,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct HirFunType {
    pub params: Vec<(String, HirType)>,
    pub returns: HirType,
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
        ty: Option<HirType>,
        expr: Option<Box<Expr>>,
    },

    Assign {
        name: String,
        expr: Box<Expr>,
    },

    Call {
        name: String,
        args: Vec<Box<Expr>>,
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

    Call {
        name: String,
        args: Vec<Box<Expr>>,
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
pub enum HirType {
    Void,
    Bool,
    Num,
}

impl Display for HirType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, "void"),
            Self::Bool => write!(f, "bool"),
            Self::Num => write!(f, "num"),
        }
    }
}
