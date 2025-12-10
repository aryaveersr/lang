use crate::ops::{BinOp, UnOp};

mod basic_block;
mod block_id;
pub mod builder;
mod cfg;
mod fun;
mod instr;
mod printer;
mod register;
mod term;

#[derive(Debug, Clone)]
pub struct MirModule {
    pub funs: Vec<MirFun>,
}

#[derive(Debug, Clone)]
pub struct MirFun {
    pub name: String,
    pub params: Vec<(Reg, MirType)>,
    pub blocks: Vec<BasicBlock>,
    pub return_ty: Option<MirType>,
}

type VarID = usize;
type Gen = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reg {
    Var(VarID, Gen),
    Temp(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockID(pub usize);

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BlockID,
    pub phis: Vec<Phi>,
    pub instrs: Vec<Instr>,
    pub term: Option<Term>,
}

#[derive(Debug, Clone)]
pub struct Phi {
    pub dest: Reg,
    pub srcs: Vec<(BlockID, Reg)>,
}

#[derive(Debug, Clone)]
pub struct Instr {
    pub dest: Reg,
    pub kind: InstrKind,
}

#[derive(Debug, Clone)]
pub enum InstrKind {
    ConstBool { value: bool },

    ConstNum { value: i32 },

    Copy { src: Reg },

    Unary { op: UnOp, arg: Reg },

    Binary { op: BinOp, lhs: Reg, rhs: Reg },

    Call { name: String, args: Vec<Reg> },
}

#[derive(Debug, Clone)]
pub enum Term {
    Jump {
        target: BlockID,
    },

    Branch {
        cond: Reg,
        then_block: BlockID,
        else_block: BlockID,
    },

    Return {
        value: Option<Reg>,
    },
}

#[derive(Debug, Clone)]
pub enum MirType {
    Num,
    Bool,
}
