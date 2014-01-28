// load, store, set, clear, wait_for

extern mod core;

use core::ops::{BitOr, BitAnd, Not};
use core::cmp::Eq;
use core::mem::{volatile_load, volatile_store};

pub enum Sync_Method {
    Memory,
    Memory_Flush,
    Instruction_Flush
}

// Note: Probably going to have to inline this stuff properly.
// Not really synchronising if there is another function call involved.
#[inline]
pub fn sync(sync_method: Sync_Method) {
    unsafe {
        match sync_method {
            Memory            => asm!("dmb"),
            Memory_Flush      => asm!("dsb"),
            Instruction_Flush => asm!("isb")
        }
    }
}

#[inline]
pub unsafe fn store<T>(dst: *mut T, val: T) {
    volatile_store(dst, val);
}

#[inline]
pub unsafe fn store_sync<T>(dst: *mut T, val: T, sync_method: Sync_Method) {
    volatile_store(dst, val);
    sync(sync_method);
}

#[inline]
pub unsafe fn load<T>(src: *T) -> T {
    volatile_load(src)
}

#[inline]
pub unsafe fn set<T: BitOr<T,T>>(dst: *mut T, mask: T) {
    let val = volatile_load(dst as *T);
    volatile_store(dst, val | mask);
}

#[inline]
pub unsafe fn clear<T: BitAnd<T,T>+Not<T>>(dst: *mut T , mask: T) {
    let val = volatile_load(dst as *T);
    volatile_store(dst, val & !mask);
}

#[inline]
pub unsafe fn wait_for<T: BitAnd<T,T>+Eq>(src: *T, mask: T, val: T) {
    while volatile_load(src) & mask != val {}
}

// #[inline]
// pub unsafe fn wait_for_zero<T: BitAnd<T,T>+Eq>(src: *T, mask: T) {
//     while volatile_load(src) & mask {}
// }
