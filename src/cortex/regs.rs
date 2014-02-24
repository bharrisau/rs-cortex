// load, store, set, clear, wait_for

//extern mod core;

//use core::ops::{BitOr, BitAnd, Not};
//use core::cmp::Eq;
//use core::mem::{volatile_load, volatile_store};

use std::intrinsics::{volatile_load, volatile_store};

pub enum SyncMethod {
    Memory,
    MemoryFlush,
    InstructionFlush
}

// Note: Probably going to have to inline this stuff properly.
// Not really synchronising if there is another function call involved.
#[inline]
pub fn sync(sync_method: SyncMethod) {
    unsafe {
        match sync_method {
            Memory            => asm!("dmb"),
            MemoryFlush      => asm!("dsb"),
            InstructionFlush => asm!("isb")
        }
    }
}

#[inline]
pub unsafe fn store<T>(dst: *mut T, val: T) {
    volatile_store(dst, val);
}

#[inline]
pub unsafe fn store_sync<T>(dst: *mut T, val: T, sync_method: SyncMethod) {
    volatile_store(dst, val);
    sync(sync_method);
}

#[inline]
pub unsafe fn load<T>(src: *mut T) -> T {
    volatile_load(src as *T)
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
