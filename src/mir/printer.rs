use itertools::Itertools as _;
use std::fmt::{self, Display, Formatter};

use crate::mir::{BasicBlock, BlockID, Instr, InstrKind, MirFun, MirModule, Phi, Register, Term};

impl Display for MirModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for fun in &self.funs {
            write!(f, "{fun}")?;
        }

        Ok(())
    }
}

impl Display for MirFun {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "fun {}() {{", self.name)?;

        for block in &self.blocks {
            write!(f, "{block}")?;
        }

        writeln!(f, "}}")
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.id)?;

        for phi in &self.phis {
            writeln!(f, "    {phi}")?;
        }

        for instr in &self.instrs {
            writeln!(f, "    {instr}")?;
        }

        if let Some(term) = &self.term {
            writeln!(f, "    {term}")?;
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

            write!(f, "{block}: {value}")?;
        }

        write!(f, "]")
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.dest, self.kind)
    }
}

impl Display for InstrKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConstBool { value } => write!(f, "const {value}"),
            Self::ConstNum { value } => write!(f, "const {value}"),
            Self::Copy { src } => write!(f, "copy {src}"),
            Self::Unary { op, arg } => write!(f, "{op} {arg}"),
            Self::Binary { op, lhs, rhs } => write!(f, "{op} {lhs} {rhs}"),
            Self::Call { name, args } => write!(f, "call {name}({})", args.iter().join(", ")),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jump { target } => write!(f, "jump {target}"),

            Self::Branch {
                cond,
                then_block,
                else_block,
            } => write!(f, "branch {cond} ? {then_block} : {else_block}"),

            Self::Return { value } => {
                if let Some(val) = value {
                    write!(f, "return {val}")
                } else {
                    write!(f, "return")
                }
            }
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Var(var_id, genn) => write!(f, "v{var_id}:{genn}"),
            Self::Temp(temp_id) => write!(f, "%{temp_id}"),
        }
    }
}

impl Display for BlockID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "bb{}", self.0)
    }
}
