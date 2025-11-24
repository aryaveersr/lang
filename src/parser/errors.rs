use serde::Serialize;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Serialize)]
pub enum ParseError {
    InvalidStmt,
    InvalidExpr,

    MissingVarName,
    MissingTypeName,
    MissingFunName,
    MissingFunLeftParen,
    MissingFunRightParen,
    MissingFunBody,

    MissingCondLeftParen,
    MissingCondRightParen,

    MissingClosingParen,
    MissingSemicolon,

    NumberTooLarge,

    Eof,
}
