use crate::ops::{BinOp, UnOp};

mod basic_block;
mod block_id;
mod const_folding;
mod display;
mod fun;
mod operand;
mod r#type;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Reg(pub usize);

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
    pub srcs: Vec<(BlockID, Operand)>,
}

#[derive(Debug, Clone)]
pub struct Instr {
    pub dest: Reg,
    pub kind: InstrKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operand {
    Bool(bool),
    Num(i32),
    Reg(Reg),
}

#[derive(Debug, Clone)]
pub enum InstrKind {
    Unary {
        op: UnOp,
        arg: Operand,
    },

    Binary {
        op: BinOp,
        lhs: Operand,
        rhs: Operand,
    },

    Call {
        name: String,
        args: Vec<Operand>,
    },
}

#[derive(Debug, Clone)]
pub enum Term {
    Jump {
        target: BlockID,
    },

    Branch {
        cond: Operand,
        then_block: BlockID,
        else_block: BlockID,
    },

    Return {
        value: Option<Operand>,
    },
}

#[derive(Debug, Clone)]
pub enum MirType {
    Num,
    Bool,
}
