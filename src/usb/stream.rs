//use core::option::{Option, None, Some};
//use core::fail::abort;
//use core::container::Container;
//use core::slice::to_mut_ptr;
//use core::cmp::min;

use std::intrinsics::abort;
use std::cmp::min;

pub struct StreamHandler {
    max_packet: uint,
    data1: bool,
    buf: Option<*mut u8>,
    transfer_size: uint,
    position: uint,
}

impl StreamHandler {
    /// Create a new StreamHandler
    pub fn new(max: uint) -> StreamHandler {
        StreamHandler {
            max_packet: max,
            data1: false,
            buf: None,
            transfer_size: 0,
            position: 0,
        }
    }

    /// Prepare to receive setup transaction
    pub fn setup(&mut self, buf: &mut [u8], data1: bool) {
        // Ensure there is no pending transaction
        if self.transfer_size != 0 {
            unsafe { abort(); }
        }

        // Save dst buffer
        self.position = 0;
        self.transfer_size = buf.len();
        self.buf = Some(buf.as_mut_ptr());

        // Set data toggle
        self.data1 = data1;
    }

    /// Update state, return true if finished
    pub fn on_token(&mut self, length: uint) -> bool {
        // Update counters
        self.transfer_size -= length;
        self.position += length;

        // Find out if transfer is finished
        // TODO: Need to know the max packet size to detect short transaction
        if self.transfer_size == 0 || length < self.max_packet {
            // TODO: Reset stream

            true
        } else {
            // Toggle DATA01
            self.data1 = !self.data1;
            false
        }
    }

    pub fn address(&self) -> *mut u8 {
        self.buf.unwrap()
    }

    pub fn len(&self) -> uint {
        min(self.max_packet, self.transfer_size)
    }

    pub fn data1(&self) -> bool {
        self.data1
    }
}
