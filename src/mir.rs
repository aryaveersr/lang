use crate::ops::{BinOp, UnOp};

mod basic_block;
mod block_id;
pub mod builder;
mod cfg;
mod fun;
mod printer;
mod value_id;

#[derive(Debug, Clone)]
pub struct MirModule {
    pub funs: Vec<MirFun>,
}

#[derive(Debug, Clone)]
pub struct MirFun {
    pub name: String,
    pub params: Vec<(ValueID, MirType)>,
    pub blocks: Vec<BasicBlock>,
    pub return_ty: Option<MirType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockID(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueID(u32, u32);

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
pub struct Instr {
    pub dest: ValueID,
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
        src: ValueID,
    },

    Unary {
        op: UnOp,
        arg: ValueID,
    },

    Binary {
        op: BinOp,
        lhs: ValueID,
        rhs: ValueID,
    },

    Call {
        name: String,
        args: Vec<ValueID>,
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
