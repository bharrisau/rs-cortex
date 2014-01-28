
extern mod core;
extern mod freescale;

use core::vec::Vec;
use core::option::{Option, Some, None};

/// Enable and setup USB device
pub fn init() {
    freescale::usb::init(|| {
        setup();
    });
}

/// Prepare endpoint 0
fn setup() {

}

/// Enum for USB state machine
enum Usb_State {
    Default,
    Addressed,
    Configured
}

/// Struct with endpoint info
struct Ep_State {
    ep: u8,
    is_setup: bool,
    data: ~Vec<u8>,
    remaining: u16,
    max_size: u16,
    callback: fn (~Vec<u8>) 
}

/// PIDs for tokens
enum Token_Pid {
    SETUP = 0x0D,
    OUT   = 0x01,
    IN    = 0x09
}

/// Handle a transaction (called after TOKDNE set)
pub fn handle_transaction(ep: u8, tx: bool, odd: bool, pid: u8, len: u16) {
    // Retrieve endpoint info struct
    let ep_state = match get_ep_state(ep, tx, odd) {
        Some(state) => state,
        None        => core::fail::abort()
    };

    // TODO: Replace constants with the enum values
    match pid {
        0x0D => handle_setup(ep_state, len),
        0x01 => handle_out(ep_state, len),
        0x09 => handle_in(ep_state, len),
        _    => stall_ep(ep)
    }
}

/// Handles a setup token
fn handle_setup(state: &mut Ep_State, len: u16) {
    // Check ep state is expecting a SETUP
    if state.is_setup {
        // Queue data same as an out token
        handle_out(state, len);
    } else {
        stall_ep(state.ep);
    }

    // Tell the peripheral to resume
    freescale::usb::resume();
}

/// Handles an out token
fn handle_out(state: &mut Ep_State, len: u16) {
    // Toggle data01
    
    // Subtract remaining bytes
    state.remaining -= len;

    // Decide if more bytes are expected
    if state.remaining <= 9 || len < state.max_size {
        // Toggle data01
        // Update buffer address/copy data out
        queue_next(state);
    } else {
        // Call the closure/function

    }
}

/// Handles an in token
fn handle_in(state: &mut Ep_State, len: u16) {

}

/// Setup the state and BDT ready for OUT/SETUP token
fn prepare_out(state: &mut Ep_State, buf: ~[u8], len: u16, callback: fn (~[u8], u16)) {

}

/// Transfer the information in the state struct to the BDT
fn queue_next(state: &mut Ep_State) {

}

/// Gets the state for the specified endpoint
fn get_ep_state(ep: u8, tx: bool, odd: bool) -> Option<&mut Ep_State> {
    None
}

/// Stall the selected endpoint
fn stall_ep(ep: u8) {
    freescale::usb::stall_ep(ep);
}
