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
    pub params: Vec<(Register, MirType)>,
    pub blocks: Vec<BasicBlock>,
    pub return_ty: Option<MirType>,
}

type Variable = usize;
type Generation = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    Variable(Variable, Generation),
    Temporary(usize),
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
    pub dest: Register,
    pub srcs: Vec<(BlockID, Register)>,
}

#[derive(Debug, Clone)]
pub struct Instr {
    pub dest: Register,
    pub kind: InstrKind,
}

#[derive(Debug, Clone)]
pub enum InstrKind {
    ConstBool {
        value: bool,
    },

    ConstNum {
        value: i32,
    },

    Copy {
        src: Register,
    },

    Unary {
        op: UnOp,
        arg: Register,
    },

    Binary {
        op: BinOp,
        lhs: Register,
        rhs: Register,
    },

    Call {
        name: String,
        args: Vec<Register>,
    },
}

#[derive(Debug, Clone)]
pub enum Term {
    Jump {
        target: BlockID,
    },

    Branch {
        cond: Register,
        then_block: BlockID,
        else_block: BlockID,
    },

    Return {
        value: Option<Register>,
    },
}

#[derive(Debug, Clone)]
pub enum MirType {
    Num,
    Bool,
}
