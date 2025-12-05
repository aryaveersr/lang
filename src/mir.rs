use crate::ops::{BinOp, UnOp};

pub mod builder;
mod impls;
mod printer;

#[derive(Debug, Clone)]
pub struct MirModule {
    pub funs: Vec<MirFun>,
}

#[derive(Debug, Clone)]
pub struct MirFun {
    pub name: String,
    pub blocks: Vec<BasicBlock>,
    pub return_ty: Option<MirType>,
    next_block: usize,
    next_value: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockID(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueID(pub usize);

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BlockID,
    pub phis: Vec<Phi>,
    pub instrs: Vec<Instr>,
    pub term: Option<Term>,
}

#[derive(Debug, Clone)]
pub struct Phi {
    pub dest: ValueID,
    pub srcs: Vec<(BlockID, ValueID)>,
}

#[derive(Debug, Clone)]
pub enum Instr {
    ConstBool {
        dest: ValueID,
        value: bool,
    },

    ConstNum {
        dest: ValueID,
        value: i32,
    },

    Copy {
        dest: ValueID,
        src: ValueID,
    },

    Unary {
        dest: ValueID,
        op: UnOp,
        arg: ValueID,
    },

    Binary {
        dest: ValueID,
        op: BinOp,
        lhs: ValueID,
        rhs: ValueID,
    },
}

#[derive(Debug, Clone)]
pub enum Term {
    Jump {
        block: BlockID,
    },

    Branch {
        cond: ValueID,
        then_block: BlockID,
        else_block: BlockID,
    },

    Return {
        value: Option<ValueID>,
    },
}

#[derive(Debug, Clone)]
pub enum MirType {
    Num,
    Bool,
}
