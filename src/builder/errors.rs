use serde::Serialize;

pub type Result<T> = std::result::Result<T, BuilderError>;

#[derive(Debug, Serialize)]
pub enum BuilderError {
    VarNotFound,
    TypeNotFound,
    MainNotFound,
    BreakOutsideLoop,

    // Type errors.
    ExpectedNum,
    ExpectedBool,

    WrongReturnType,
    OperandsNotSameType,
    CannotInferType,
    TypeMismatch,
}
