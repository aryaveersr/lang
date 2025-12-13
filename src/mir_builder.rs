use crate::{
    mir::{BlockID, MirFun, MirType},
    mir_builder::value::Value,
    ops::{BinOp, UnOp},
};

pub mod value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarID(usize);

type Gen = usize;

pub struct MirBuilder {}

impl MirBuilder {
    pub fn new(name: String) -> Self {
        todo!()
    }

    pub fn set_return_type(&mut self, ty: Option<MirType>) {
        todo!()
    }

    pub fn create_block(&mut self) -> BlockID {
        todo!()
    }

    pub fn seal_block(&mut self, id: BlockID) {
        todo!()
    }

    pub fn set_active_block(&mut self, id: BlockID) {
        todo!()
    }

    pub fn declare_var(&mut self) -> VarID {
        todo!()
    }

    pub fn assign_var(&mut self, var_id: VarID, value: Value) {
        todo!()
    }

    pub fn has_terminator(&self) -> bool {
        todo!()
    }

    pub fn build_unary(&mut self, op: UnOp, arg: Value) -> Value {
        todo!()
    }

    pub fn build_binary(&mut self, op: BinOp, lhs: Value, rhs: Value) -> Value {
        todo!()
    }

    pub fn build_call(&mut self, name: String, args: Vec<Value>) -> Value {
        todo!()
    }

    pub fn build_jump(&mut self, target: BlockID) {
        todo!()
    }

    pub fn build_branch(&mut self, cond: Value, then_block: BlockID, else_block: BlockID) {
        todo!()
    }

    pub fn build_return(&mut self, value: Option<Value>) {
        todo!()
    }

    pub fn finish(mut self) -> MirFun {
        todo!()
    }
}
