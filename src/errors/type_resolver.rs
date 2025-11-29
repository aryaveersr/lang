use crate::{hir::Type, ops::*};
use serde::Serialize;
use std::fmt::Display;
use thiserror::Error;

pub type TypeResult<T> = std::result::Result<T, TypeError>;

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TypeError {
    #[error("Non-boolean condition: found {found}.")]
    NonBooleanCondition { found: Type },

    #[error("Undefined variable: {name}.")]
    UndefinedVar { name: String },

    #[error("Cannot infer type for variable: {name}.")]
    CannotInferType { name: String },

    #[error("Type mismatch: expected {expected}, found {found}.")]
    TypeMismatch { expected: Type, found: Type },

    #[error("Invalid unary operation {op} for type {ty}.")]
    InvalidUnaryOp { op: UnOp, ty: Type },

    #[error("Invalid binary operation {op} for types {lhs} and {rhs}.")]
    InvalidBinaryOp { op: BinOp, lhs: Type, rhs: Type },
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Bool => write!(f, "bool"),
            Type::Num => write!(f, "num"),
        }
    }
}
