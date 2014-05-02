#![no_main]
#![no_start]

#![crate_id = "blink"]

//TODO: Move startup.S into rust

extern crate cortex;
//extern crate freescale;

use cortex::regs::set;

//#[link_section=".isr_vector_temp"]
//#[no_mangle]
//pub static ISRVectors: [extern unsafe fn(), ..4] = [
//      _stack_base,
//      main,
//      isr_nmi,
//      isr_hardfault,
//];

extern {
    static mut __STACK_LIMIT: u32;
    static __StackLimit: u32;
}
//
//#[no_mangle]
//pub static mut STACK_LIMIT: u32 = __StackLimit;

//#[link_section=".test"]
//#[no_mangle]
//pub static FlashConfig: [u8, ..16] = [
//      0xFF,
//      0xFF,
//      0xFF,
//      0xFF,
//      0xFF,
//      0xFF,
//      0xFF,
//      0xFF,
//      0xFF,
//      0xFF,
//      0xFF,
//      0xFF,
//      0xFE,
//      0xFF,
//      0xFF,
//      0xFF,
//];

#[no_mangle]
pub extern "C" fn _start() {
    unsafe { __STACK_LIMIT = (&__StackLimit as *u32) as u32; }
    loop { }
}

#[no_mangle]
#[no_split_stack]
pub extern "C" fn HardFault_Handler() {
}

//#[no_mangle]
//pub extern "C" fn get_sp_limit() {
//
//}
//
//#[no_mangle]
//pub extern "C" fn record_sp_limit() {
//
//}

//extern "rust-intrinsic" { fn abort() -> !; }

#[no_mangle]
#[no_split_stack]
pub extern "C" fn __morestack() {
        unsafe { ::std::intrinsics::abort() }
}

#[no_mangle]
#[no_split_stack]
pub extern "C" fn SystemInit() {
}
