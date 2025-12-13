use crate::{
    mir::{InstrKind, Operand},
    ops::{BinOp, UnOp},
};

impl InstrKind {
    pub fn try_fold(&self) -> Option<Operand> {
        match self {
            Self::Call { .. } => None,

            Self::Unary { op, arg } => arg.is_const().then(|| match op {
                UnOp::Negate => Operand::Num(-arg.as_num()),
                UnOp::Not => Operand::Bool(!arg.as_bool()),
            }),

            Self::Binary { op, lhs, rhs } => (lhs.is_const() && rhs.is_const())
                .then(|| match op {
                    BinOp::Add => Operand::Num(lhs.as_num() + rhs.as_num()),
                    BinOp::Sub => Operand::Num(lhs.as_num() - rhs.as_num()),
                    BinOp::Mul => Operand::Num(lhs.as_num() * rhs.as_num()),
                    BinOp::Div => Operand::Num(lhs.as_num() / rhs.as_num()),

                    BinOp::And => Operand::Bool(lhs.as_bool() && rhs.as_bool()),
                    BinOp::Or => Operand::Bool(lhs.as_bool() || rhs.as_bool()),

                    BinOp::Eq => Operand::Bool(lhs == rhs),
                    BinOp::NotEq => Operand::Bool(lhs != rhs),
                    BinOp::Lesser => Operand::Bool(lhs.as_num() < rhs.as_num()),
                    BinOp::LesserEq => Operand::Bool(lhs.as_num() <= rhs.as_num()),
                    BinOp::Greater => Operand::Bool(lhs.as_num() > rhs.as_num()),
                    BinOp::GreaterEq => Operand::Bool(lhs.as_num() >= rhs.as_num()),
                })
                .or_else(|| {
                    (lhs.is_const() || rhs.is_const())
                        .then(|| {
                            let (cons, nconst) = if lhs.is_const() {
                                (*lhs, *rhs)
                            } else {
                                (*rhs, *lhs)
                            };

                            match op {
                                BinOp::Add | BinOp::Sub if cons.as_num() == 0 => Some(nconst),
                                BinOp::Mul | BinOp::Div if cons.as_num() == 1 => Some(nconst),
                                BinOp::Mul if cons.as_num() == 0 => Some(Operand::Num(0)),

                                BinOp::And if cons.as_bool() => Some(nconst),
                                BinOp::And if !cons.as_bool() => Some(Operand::Bool(false)),
                                BinOp::Or if cons.as_bool() => Some(Operand::Bool(true)),
                                BinOp::Or if !cons.as_bool() => Some(nconst),

                                _ => None,
                            }
                        })
                        .flatten()
                })
                .or_else(|| {
                    (lhs == rhs)
                        .then(|| match op {
                            BinOp::Sub => Some(Operand::Num(0)),
                            BinOp::Div => Some(Operand::Num(1)),
                            BinOp::And | BinOp::Or => Some(*lhs),
                            BinOp::NotEq | BinOp::Lesser | BinOp::Greater => Some(false.into()),
                            BinOp::Eq | BinOp::LesserEq | BinOp::GreaterEq => Some(true.into()),

                            _ => None,
                        })
                        .flatten()
                }),
        }
    }
}
