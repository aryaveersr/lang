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
mod r#type;
mod value;

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
    Var { var_id: VarID, genn: Gen },
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
    pub srcs: Vec<(BlockID, Value)>,
}

#[derive(Debug, Clone)]
pub struct Instr {
    pub dest: Reg,
    pub kind: InstrKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Value {
    Bool(bool),
    Num(i32),
    Reg(Reg),
}

#[derive(Debug, Clone)]
pub enum InstrKind {
    ConstBool { value: bool },
    ConstNum { value: i32 },
    Copy { src: Value },
    Unary { op: UnOp, arg: Value },
    Binary { op: BinOp, lhs: Value, rhs: Value },
    Call { name: String, args: Vec<Value> },
}

#[derive(Debug, Clone)]
pub enum Term {
    Jump {
        target: BlockID,
    },

    Branch {
        cond: Value,
        then_block: BlockID,
        else_block: BlockID,
    },

    Return {
        value: Option<Value>,
    },
}

#[derive(Debug, Clone)]
pub enum MirType {
    Num,
    Bool,
}
