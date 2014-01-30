//#[feature(phase)];

#[no_std];
#[crate_id = "usb"];
#[crate_type = "rlib"];
#[license = "MIT"];

extern mod core;

pub mod usb;
pub mod stream;
pub mod control;

