#[macro_escape];
// load and store. word, half, byte. normal and wait (dmb) and wait_all (dsb)

#[macro_export]
macro_rules! sfr_store_word(
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

// #[allow(dead_code)]
// pub fn store_word(addr: u32, val: u32) {
//   unsafe {
//     asm!("str $0, [$1]"
//       :
//       : "r"(val), "r"(addr)
//       :
//       : "volatile"
//       );
//   }
// }
// 
// #[allow(dead_code)]
// pub fn store_word_wait(addr: u32, val: u32) {
//   unsafe {
//     asm!("str $0, [$1]\n\t
//           dmb"
//       :
//       : "r"(val), "r"(addr)
//       :
//       : "volatile"
//       );
//   }
// }
// 
// #[allow(dead_code)]
// pub fn store_word_wait_all(addr: u32, val: u32) {
//   unsafe {
//     asm!("str $0, [$1]\n\t
//           dsb"
//       :
//       : "r"(val), "r"(addr)
//       :
//       : "volatile"
//       );
//   }
// }
