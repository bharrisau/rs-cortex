#![no_main]
#![no_start]

#![crate_id = "blink"]

extern crate cortex;
//extern crate freescale;

#[no_mangle]
pub extern "C" fn _start() {
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
