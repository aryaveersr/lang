mod errors;
mod expr;
mod parser;
mod stmt;
mod ty;
mod utils;

use crate::{ast::*, ops::*, token::*};
use errors::*;

type To = TokenKind;
type Pe = ParseError;

pub use errors::ParseError;
pub use parser::Parser;
