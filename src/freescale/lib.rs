//#[feature(phase)];

#![crate_id = "freescale"]
#![crate_type = "rlib"]
#![license = "MIT"]

extern crate cortex;
extern crate rustusb = "usb";

pub mod sim;
pub mod usb;
