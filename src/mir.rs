use crate::ops::{BinOp, UnOp};

pub struct MirModule {
    pub funs: Vec<MirFun>,
}

pub struct MirFun {
    pub name: String,
    pub blocks: Vec<BasicBlock>,
    pub return_ty: MirType,
}

pub struct BlockID(pub usize);

pub struct ValueID(pub usize);

pub struct BasicBlock {
    pub id: BlockID,
    pub phis: Vec<Phi>,
    pub instrs: Vec<Instr>,
    pub terminator: Option<Terminator>,
}

pub struct Phi {
    pub dest: ValueID,
    pub srcs: Vec<(BlockID, ValueID)>,
}

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

pub enum Terminator {
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

pub enum MirType {
    Num,
    Bool,
}
