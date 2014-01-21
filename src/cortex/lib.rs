#[feature(managed_boxes, globs, macro_registrar, macro_rules)];
#[macro_escape];

#[no_std];
#[crate_id = "cortex#0.1"];
#[crate_type = "lib"];
#[license = "MIT"];

#[feature(asm)];

pub mod regs;

#[macro_export]
macro_rules! sfr_store_word2(
  ($addr:ident, $val:ident) => (
    unsafe {
      asm!("str $0, [$1]"
           :
           : "r"($val), "r"($addr)
           :
           : "volatile"
           );
    }
  );
)
