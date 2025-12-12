use std::collections::HashMap;

use crate::{
    mir::{MirFun, Value},
    mir_passes::rename_operands::rename_operands,
};

pub fn remove_trivial_phis(fun: &mut MirFun) {
    let mut renames = HashMap::new();
    let mut changed = true;

    while changed {
        changed = false;

        for block in &mut fun.blocks {
            block.phis.retain(|phi| {
                let trivial = phi
                    .srcs
                    .iter()
                    .all(|src| src.1 == phi.srcs[0].1 || src.1 == Value::Reg(phi.dest));

                if trivial {
                    let value = phi
                        .srcs
                        .iter()
                        .find(|src| src.1 != Value::Reg(phi.dest))
                        .unwrap()
                        .1;

                    changed = true;
                    renames.insert(Value::Reg(phi.dest), value);
                }

                !trivial
            });
        }

        if changed {
            for block in &mut fun.blocks {
                for phi in &mut block.phis {
                    for (_, value) in &mut phi.srcs {
                        if let Some(new_value) = renames.get(value) {
                            *value = *new_value;
                        }
                    }
                }
            }

            rename_operands(fun, &renames);
            renames.clear();
        }
    }
}
