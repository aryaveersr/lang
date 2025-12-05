use std::fmt::{self, Display, Formatter};

use crate::mir::{BasicBlock, BlockID, Instr, InstrKind, MirFun, MirModule, Phi, Term, ValueID};

impl Display for MirModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for fun in &self.funs {
            write!(f, "{}", fun)?;
        }

        Ok(())
    }
}

impl Display for MirFun {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "fun {}() {{", self.name)?;

        for block in &self.blocks {
            write!(f, "{}", block)?;
        }

        write!(f, "}}")
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.id)?;

        for phi in &self.phis {
            writeln!(f, "    {}", phi)?;
        }

        for instr in &self.instrs {
            writeln!(f, "    {}", instr)?;
        }

        if let Some(term) = &self.term {
            writeln!(f, "    {}", term)?;
        }

        Ok(())
    }
}

impl Display for Phi {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = phi [", self.dest)?;

        for (i, (block, value)) in self.srcs.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}: {}", block, value)?;
        }

        write!(f, "]")
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = ", self.dest)?;

        match self.kind {
            InstrKind::ConstBool { value } => write!(f, "const {}", value),
            InstrKind::ConstNum { value } => write!(f, "const {}", value),
            InstrKind::Copy { src } => write!(f, "copy {}", src),
            InstrKind::Unary { op, arg } => write!(f, "{} {}", op, arg),
            InstrKind::Binary { op, lhs, rhs } => {
                write!(f, "{} {}, {}", lhs, op, rhs)
            }
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Term::Jump { block } => write!(f, "jump {}", block),

            Term::Branch {
                cond,
                then_block,
                else_block,
            } => write!(f, "branch {} ? {} : {}", cond, then_block, else_block),

            Term::Return { value } => {
                if let Some(val) = value {
                    write!(f, "return {}", val)
                } else {
                    write!(f, "return")
                }
            }
        }
    }
}

impl Display for ValueID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}

impl Display for BlockID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "bb{}", self.0)
    }
}
