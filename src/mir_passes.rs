use crate::mir::MirModule;

mod dead_blocks;

trait MirPass: Default {
    fn run(&mut self, module: &mut MirModule);
}

pub fn run_passes(module: &mut MirModule) {
    dead_blocks::DeadBlocks::default().run(module);
}
