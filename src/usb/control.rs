/// Logic for the control endpoint

use usb::{EndpointHandler, UsbModule, Control, TokenPid};
use stream::StreamHandler;

static REQ_GET_STATUS     : u8 = 0x00;
static REQ_CLEAR_FEATURE  : u8 = 0x01;
static REQ_SET_FEATURE    : u8 = 0x03;
static REQ_SET_ADDRESS    : u8 = 0x05;
static REQ_GET_DESCRIPTOR : u8 = 0x06;
static REQ_SET_DESCRIPTOR : u8 = 0x07;
static REQ_GET_CONFIG     : u8 = 0x08;
static REQ_SET_CONFIG     : u8 = 0x09;
static REQ_GET_INTERFACE  : u8 = 0x0A;
static REQ_SET_INTERFACE  : u8 = 0x11;

pub struct Ep0Handler {
    num: uint,
    buf: [u8, ..64],
    rx_stream: StreamHandler,
}

impl Ep0Handler {
    pub fn new(max_packet: uint) -> Ep0Handler {
        Ep0Handler {
            num: 0,
            buf: [0, ..64],
            rx_stream: StreamHandler::new(max_packet),
        }
    }
}

impl EndpointHandler for Ep0Handler {
//    fn handle_setup(&self, module: &UsbModule, endpoint: uint, length: uint) -> bool {
//        true
//    }
//
//    fn handle_out(&self, module: &UsbModule, endpoint: uint, length: uint) -> bool {
//        true
//    }
//
//    fn handle_in(&self, module: &UsbModule, endpoint: uint, length: uint) -> bool {
//        true
//    }
    
    /// Activate endpoint 0 and prepare to receive setup transaction
    fn on_reset(&mut self, module: &'static mut UsbModule) {
        // Enable EP0 for control transfers
        module.enable_ep(0, Control);

        // Ready stream handler for setup transaction
        let buf = self.buf.as_mut_slice();
        self.rx_stream.setup(buf, false);
        
        // Setup RX on EP0
        module.queue_rx(0, &self.rx_stream);
    }

    /// Handle token addressed to endpoint
    fn on_token(&mut self, module: &'static mut UsbModule, ep: uint, is_tx: bool, pid: TokenPid, len: uint) {
        // Check PID matches expected state
        // Feed token into stream handler
        // If finished, process as control transfer
        // If not, queue next transaction
    }
}

/// Handles transfers on the control endpoint
///
/// bm_type is one of
///  0 = standard
///  1 = class
///  2 = vendor
///  _ = reserved
///
/// bm_rcpt is one of
///  0 = device
///  1 = interface
///  2 = endpoint
///  3 = other
///  _ = reserved
pub fn handle_control(data: Vec<u8>) {
    let packet = data.as_slice();

    // Break down bmRequestType into dir, type, rcpt
    let bmRequestType = packet[0];
    let bm_dir_in = bmRequestType >= 0x80;

    //TODO: Mash this into enum
    let bm_type = (bmRequestType >> 5) & 3;
    let bm_rcpt = bmRequestType & 0x1F;

    // Call out to non-standard handler if needed
    
    // Match on bRequest
    let bRequest = packet[1];
    match bRequest {
        REQ_GET_STATUS      => {
            // Return 16 bit response
            // remote_wakeup << 1 | self_powered
        }
        REQ_CLEAR_FEATURE   => {
            // No op
        }
        REQ_SET_FEATURE     => {
            // No op
        }
        REQ_SET_ADDRESS     => {
            // Store the address value and set state machine
            // to update address at end of status stage.
            // Address is in wValue
        }
        REQ_GET_DESCRIPTOR  => {
            // Call function to return descriptor in data phase
            // Needs wValue (type & index), wIndex (lang), and wLength
        }
        REQ_SET_DESCRIPTOR  => {
            // No op
        }
        REQ_GET_CONFIG      => {
            // Return the 1 byte value in the data phase
        }
        REQ_SET_CONFIG      => {
            // Set the configuration value (changes state machine)
        }
        REQ_GET_INTERFACE   => {
            // Return 0 in the data phase to indicate no
            // alternative interfaces
        }
        REQ_SET_INTERFACE   => {
            // Shouldn't be called with anything but 0
            // TODO: Use this and get_interface to support alt interfaces
        }
        _                   => {
            // Unsupported: stall the EP
        }
    }

    // Prepare for status transaction based on bm_dir_in
    // Set data1 on EP0 bm_dir_in ? OUT : IN
    // Set callback function
    // Ready for 0 byte IN/OUT transaction
}

/// Sets up EP0 to receive a SETUP transaction
pub fn setup_control() {
    // Get the EP0 OUT state object
//    let state = usb::usb::get_ep_state(0, false, false);
//
//    // Expect setup token
//    state.is_setup = true;
//
//    // Expect DATA0
//    state.data1 = false;
//    
//    // Prepare for next transaction
//    usb::usb::prepare_out(state, BUFFER, state.max_size, CALLBACK);
}

/// Called after end of status phase
fn finish_control(data: Vec<u8>) {
    // Commit any pending address change
    
    // Ready for next SETUP transaction
    setup_control();
}
