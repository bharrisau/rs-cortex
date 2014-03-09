
//use core::vec::Vec;
//use core::option::{Option, Some, None};
//use core::fail::abort;
use control::Ep0Handler;
use stream::StreamHandler;
use std::vec_ng::Vec;
use std::intrinsics::abort;

static mut USB_MODULE: Option<UsbModule> = None; 

pub enum EndpointType {
    Control,
    TxOnly,
    RxOnly,
    TxRx,
    IsochronousTx,
    IsochronousRx
}

/// The trait for the low level USB peripheral
pub trait UsbPeripheral {
    fn init(&self);
    fn attach(&self);
    fn reset(&self);

    fn poll(&self);

    fn max_endpoints(&self) -> uint;

    fn queue_next(&mut self, uint, bool, &StreamHandler);
    fn set_address(&self, u8);
    fn ep_enable(&self, uint, EndpointType);
    fn ep_stall(&self, uint);
    fn ep_unstall(&self, uint);
}

/// The trait for the endpoint handlers
pub trait EndpointHandler {
//    fn handle_setup(&self, &UsbModule, uint, uint) -> bool;
//    fn handle_out(&self, &UsbModule, uint, uint) -> bool;
//    fn handle_in(&self, &UsbModule, uint, uint) -> bool;

    fn on_reset(&mut self, &'static mut UsbModule);

    fn on_token(&mut self, &'static mut UsbModule, uint, bool, TokenPid, uint);
}


/// Enum for USB state machine
pub enum UsbState {
    Unattached,
    Attached,
    Default,
    SetAddress,
    Address,
    Configured,
    Suspended
}

pub struct UsbModule {
    peripheral: &'static mut UsbPeripheral,
    state: UsbState,
    handler: Ep0Handler,
}

impl UsbModule {
    pub fn new(peripheral: &'static mut UsbPeripheral) -> &'static mut UsbModule {
        // Abort if already initialised
        if UsbModule::is_ready() {
            unsafe { abort(); }
        }

        // Create struct and store as singleton
        let module = UsbModule {
            peripheral: peripheral,
            state: Unattached,
            handler: Ep0Handler::new(64)
        };
        unsafe {
            USB_MODULE = Some(module);
        }

        UsbModule::get()
    }

    /// Return true if the singleton is ready
    pub fn is_ready() -> bool {
        unsafe {
            match USB_MODULE {
                Some(_) => true,
                None => false
            }
        }
    }

    /// Get the singleton object
    pub fn get() -> &'static mut UsbModule {
        unsafe {
            match USB_MODULE {
                Some(ref mut module) => module,
                None => abort()
            }
        }
    }

    /// Enable and setup USB device
    pub fn init(&mut self) {
        // Initialise the low level device
        self.peripheral.init();

        // Attach and wait for reset
        self.peripheral.attach();

        // Set state to attached
        self.state = Attached;
    }

    /// React to USB bus reset
    pub fn on_reset(&'static mut self) {
        // Reset peripheral
        self.peripheral.reset();

        // Prepare endpoint 0
        self.handler.on_reset(self);

        // Set state to default
        self.state = Default;
    }

    /// Handle after STALL has been sent
    pub fn on_stall(&'static mut self) {
        // Ensure EP0 isn't stalled

        // Pass stall event to other handlers
        // Prepare endpoint 0 (if ep0 has stalled)
        // TODO: Add call for EP0 here
    }

    /// Handle transaction
    pub fn on_token(&'static mut self, endpoint: uint, is_tx: bool, pid: uint, length: uint) {
        // Get the handler for the endpoint
        // TODO: Support more than EP0
        let handler = &mut self.handler;

        // Convert pid into enum
        match FromPrimitive::from_uint(pid) {
            Some(pid) => handler.on_token(self, endpoint, is_tx, pid, length),
            None => {}
        }
    }

    /// Enable the endpoint for given transactions
    pub fn enable_ep(&mut self, endpoint: uint, transfer: EndpointType) {
        // Tell peripheral to enable the endpoint
        self.peripheral.ep_enable(endpoint, transfer);
    }

    /// Set the STALL flag on an endpoint
    pub fn stall_ep(&self, endpoint: uint) {
        self.peripheral.ep_stall(endpoint);
    }

    /// Prepare to receive transaction on endpoint
    /// Takes endpoint number and stream handler
    pub fn queue_rx(&mut self, endpoint: uint, stream: &StreamHandler) {
        // Pass to peripheral
        self.peripheral.queue_next(endpoint, false, stream);
    }
    
    /// Prepare to transmit transaction on endpoint
    pub fn queue_tx(&self, endpoint: uint, data1: bool, buf: &mut [u8]) {

    }
}

/// Prepare endpoint 0
fn setup() {

}

/// Struct with endpoint info
pub struct EpState {
    ep: u8,
    is_setup: bool,
    data: ~Vec<u8>,
    remaining: u16,
    max_size: u16,
    callback: fn (~Vec<u8>) 
}

/// PIDs for tokens
#[deriving(Eq, FromPrimitive)]
pub enum TokenPid {
    Out         = 0b0001,
    In          = 0b1001,
    Sof         = 0b0101,
    Setup       = 0b1101,

    Data0       = 0b0011,
    Data1       = 0b1011,
    Data2       = 0b0111,
    Mdata       = 0b1111,

    Ack         = 0b0010,
    Nak         = 0b1010,
    Stall       = 0b1110,
    Nyet        = 0b0110,

    Pre         = 0b1100,
    Split       = 0b1000,
    Ping        = 0b0100,
    Reserved    = 0b0000,
}

//impl TokenPid {
//    pub fn from_u8(num: u8) -> TokenPid {
//        unsafe {
//            core::mem::transmute(num & 0xF)
//        }
//    }
//}

/// Handle a transaction (called after TOKDNE set)
pub fn handle_transaction(ep: u8, tx: bool, odd: bool, pid: u8, len: u16) {
    // Retrieve endpoint info struct
    let ep_state = match get_ep_state(ep, tx, odd) {
        Some(state) => state,
        None        => unsafe { abort() }
    };

}

/// Handles a setup token
fn handle_setup(state: &mut EpState, len: u16) {
    // Check ep state is expecting a SETUP
    if state.is_setup {
        // Queue data same as an out token
        handle_out(state, len);
    } else {
        stall_ep(state.ep);
    }

    // Tell the peripheral to resume
    //freescale::usb::resume();
}

/// Handles an out token
fn handle_out(state: &mut EpState, len: u16) {
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
        //state.callback(state.data);
    }
}

/// Handles an in token
fn handle_in(state: &mut EpState, len: u16) {

}

/// Setup the state and BDT ready for OUT/SETUP token
pub fn prepare_out(state: &mut EpState, buf: ~[u8], len: u16, callback: fn (~[u8], u16)) {

}

/// Transfer the information in the state struct to the BDT
fn queue_next(state: &mut EpState) {

}

/// Gets the state for the specified endpoint
pub fn get_ep_state(ep: u8, tx: bool, odd: bool) -> Option<&mut EpState> {
    None
}

/// Stall the selected endpoint
fn stall_ep(ep: u8) {
    //freescale::usb::stall_ep(ep);
}
