use serde::Serialize;
use thiserror::Error;

use crate::{
    hir::HirType,
    ops::{BinOp, UnOp},
};

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TypeError {
    #[error("Non-boolean condition: found {found}.")]
    NonBooleanCondition { found: HirType },

    #[error("Undefined variable: {name}.")]
    UndefinedVar { name: String },

    #[error("Undefined function: {name}.")]
    UndefinedFun { name: String },

    #[error("Cannot infer type for variable: {name}.")]
    CannotInferType { name: String },

    #[error("Type mismatch: expected {expected}, found {found}.")]
    TypeMismatch { expected: HirType, found: HirType },

    #[error("Invalid unary operation {op} for type {ty}.")]
    InvalidUnaryOp { op: UnOp, ty: HirType },

    #[error("Invalid binary operation {op} for types {lhs} and {rhs}.")]
    InvalidBinaryOp {
        op: BinOp,
        lhs: HirType,
        rhs: HirType,
    },
}
