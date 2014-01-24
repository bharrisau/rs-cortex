//#[feature(phase)];

#[no_std];
#[crate_id = "freescale"];
#[crate_type = "rlib"];
#[license = "MIT"];

extern mod core;
extern mod cortex;

pub mod sim;
pub mod usb;
