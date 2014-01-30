use core::option::{Option, None, Some};
use core::fail::abort;
use core::container::Container;
use core::slice::to_mut_ptr;
use core::cmp::min;

pub struct Stream_Handler {
    max_packet: uint,
    data1: bool,
    buf: Option<*mut u8>,
    transfer_size: uint,
    position: uint,
}

impl Stream_Handler {
    /// Create a new Stream_Handler
    pub fn new(max: uint) -> Stream_Handler {
        Stream_Handler {
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
            abort();
        }

        // Save dst buffer
        self.position = 0;
        self.transfer_size = buf.len();
        self.buf = Some(to_mut_ptr(buf));

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
        self.buf.get()
    }

    pub fn len(&self) -> uint {
        min(self.max_packet, self.transfer_size)
    }

    pub fn data1(&self) -> bool {
        self.data1
    }
}
