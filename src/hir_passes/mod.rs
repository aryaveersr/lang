mod type_resolver;
mod visitor;

use crate::{hir::*, ops::*, scope::*};

pub use type_resolver::TypeResolver;
pub use visitor::*;
