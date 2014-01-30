//#[feature(phase)];

#[no_std];
#[crate_id = "freescale"];
#[crate_type = "rlib"];
#[license = "MIT"];

extern mod core;
extern mod cortex;
extern mod rustusb = "usb";

pub mod sim;
pub mod usb;
