#[feature(asm, macro_rules)];

#[no_std];
#[crate_id = "cortex"];
#[crate_type = "rlib"];
#[license = "MIT"];

extern mod core;

pub mod regs;
