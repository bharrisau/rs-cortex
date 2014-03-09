
extern crate cortex;
extern crate rustusb = "usb";

use std::intrinsics::abort;
use std::ptr::set_memory;
use std::rt::global_heap::malloc_raw;
use cortex::regs::{store, load, set, wait_for};
use sim::{enable_clock, USBOTG};
use sim::{select_usb_source};
use rustusb::usb::{UsbPeripheral, UsbModule};
use rustusb::usb::{EndpointType};
use rustusb::stream::StreamHandler;

mod sim;

static mut USB_PERIPHERAL: Option<FreescaleUsb> = None;

static BASE_USB: u32        = 0x4007_2000;
static USB_USBTRC0: u32     = BASE_USB + 0x010C;
static USB_USBCTRL: u32     = BASE_USB + 0x0100;
static USB_CTL: u32         = BASE_USB + 0x0094;
static USB_ADDR: u32        = BASE_USB + 0x0098;
static USB_CONTROL: u32     = BASE_USB + 0x0108;
static USB_INTEN: u32       = BASE_USB + 0x0084;

static USB_STAT: u32        = BASE_USB + 0x0090;
static USB_ISTAT: *mut u8   = (BASE_USB + 0x0080) as *mut u8;
static USB_ERRSTAT: u32     = BASE_USB + 0x0088;
static USB_OTGISTAT: u32    = BASE_USB + 0x0010;

static USB_BDTPAGE1: u32    = BASE_USB + 0x009C;
static USB_BDTPAGE2: u32    = BASE_USB + 0x00B0;
static USB_BDTPAGE3: u32    = BASE_USB + 0x00B4;

static USB_ENDPT0: u32      = BASE_USB + 0x00C0;

pub enum UsbInt {
    USBRSTEN = 0x01,
    ERROREN  = 0x02,
    SOFTOKEN = 0x04,
    TOKDNEEN = 0x08,
    SLEEPEN  = 0x10,
    RESUMEEN = 0x20,
    ATTACHEN = 0x40,
    STALLEN  = 0x80
}

// fn zero_bdt() {
// }
// 
// fn set_bdt() {
//     // Need a 512 byte aligned memory
//     let addr: u32 = 0x00; // This should be a pointer to the BDT
//     unsafe {
//         store(USB_BDTPAGE1 as *mut u8, (addr << 8) as u8);
//         store(USB_BDTPAGE2 as *mut u8, (addr << 16) as u8);
//         store(USB_BDTPAGE3 as *mut u8, (addr << 24) as u8);
//     }
// }
// 
// 
// /// Read PID from BDT
// pub fn get_pid(ep: u8) {
// 
// }
// 
// /// Sets the STALL flag for ENDPTx register
// pub fn stall_ep(ep: u32) {
//     if ep <= 15 {
//         let addr = USB_ENDPT0 + ep*4;
//         unsafe {
//             set(addr as *mut u8, 0x02);
//         }
//     }
// }
// 
// /// Clears the STALL flag for ENDPTx register
// pub fn unstall_ep(ep: u32) {
//     if ep <= 15 {
//         let addr = USB_ENDPT0 + ep*4;
//         unsafe {
//             clear(addr as *mut u8, 0x02);
//         }
//     }
// }
// 
// /// Enables the endpoint (also unstalls)
// pub fn enable_ep(ep: u32, control: bool, handshake: bool) {
//     if ep <= 15 {
//         let addr = USB_ENDPT0 + ep*4;
//         let val = 0x0c; // |
//         //    (if !control 0x10 else 0) |
//         //    (if handshake 0x01 else 0)
//         unsafe {
//             store(addr as *mut u8, val as u8);
//         }
//     }
// }
// 
// /// Resume token processing
// pub fn resume() {
//     unsafe {
//         clear(USB_CTL as *mut u8, 0x20);
//     }
// }
// 
// pub fn set_interrupt(val: UsbInt) {
//     unsafe {
//         set(USB_INTEN as *mut u8, val as u8);
//     }
// }
// 

pub struct FreescaleUsb {
    bdt: *mut u32,
    max_ep: uint,
    ping: [bool, ..32],
}

impl FreescaleUsb {
    /// Create a new instance
    /// Specify number of endpoints including EP0
    pub fn new(max_endpoint: uint) -> &'static mut FreescaleUsb {
        let size = if max_endpoint > 15 {
            512
        } else {
            max_endpoint * 32
        };
        
        // Need 512byte aligned memory for BDT
        // TODO: Need to fix up alignment. Maybe just static assignment?
        let ptr = unsafe { malloc_raw(512) };
        let this =FreescaleUsb {
            bdt: ptr as *mut u32,
            max_ep: max_endpoint,
            ping: [false, ..32],
        };
        unsafe { USB_PERIPHERAL = Some(this); }
        FreescaleUsb::get()
    }

    pub fn get() -> &'static mut FreescaleUsb {
        unsafe {
            match USB_PERIPHERAL {
                Some(ref mut module) => module,
                None => abort()
            }
        }
    }

    fn get_bdt_offset(ep: uint, is_tx: bool, is_odd: bool) -> uint {
        32*ep + (if is_tx { 16 } else { 0 } ) +
            (if is_odd { 8 } else { 0 } )
    }
    
    pub fn get_bdt_setting(&self, ep: uint, is_tx: bool, is_odd: bool) -> u32 {
        let offset = FreescaleUsb::get_bdt_offset(ep, is_tx, is_odd);
        let addr = ((self.bdt as u32) + offset as u32) as *mut u32;
        unsafe {
            load(addr)
        }
    }
    
    pub fn set_bdt_setting(&self, ep: uint, is_tx: bool, is_odd: bool, val: u32) {
        let offset = FreescaleUsb::get_bdt_offset(ep, is_tx, is_odd);
        let addr = ((self.bdt as u32) + offset as u32) as *mut u32;
        unsafe {
            store(addr, val);
        }
    }

    pub fn get_bdt_address(&self, ep: uint, is_tx: bool, is_odd: bool) -> u32 {
        let offset = FreescaleUsb::get_bdt_offset(ep, is_tx, is_odd) + 4;
        let addr = ((self.bdt as u32) + offset as u32) as *mut u32;
        unsafe {
            load(addr)
        }
    }
    
    pub fn set_bdt_address(&self, ep: uint, is_tx: bool, is_odd: bool, val: u32) {
        let offset = FreescaleUsb::get_bdt_offset(ep, is_tx, is_odd) + 4;
        let addr = ((self.bdt as u32) + offset as u32) as *mut u32;
        unsafe {
            store(addr, val);
        }
    }
    
    fn usb_reset_hard(&self) {
        unsafe {
            let addr: *mut u8 = (BASE_USB + 0x010C) as *mut u8;
            store(addr, 0x80);
            wait_for(addr as *u8, 0x80, 0x00);
        }
    }
    
    fn set_interrupts(&self, val: u8) {
        unsafe {
            store(USB_INTEN as *mut u8, val as u8);
        }
    }
}

// TODO: Implement Drop trait to free the bdt

impl UsbPeripheral for FreescaleUsb {
    /// Enables the USB peripheral
    /// Selects clock source, resets peripheral hardware,
    /// allocates needed memory, runs software usb reset
    fn init(&self) {
        // Enable the SIM clock for USB
        enable_clock(USBOTG);

        // Set the USB clock source to internal
        select_usb_source(true);

        // Cycle the USB module through a hard reset
        self.usb_reset_hard();

        // Load the address of the BDT
        unsafe {
            let addr = self.bdt as u32;
            store(USB_BDTPAGE1 as *mut u8, (addr << 8) as u8);
            store(USB_BDTPAGE2 as *mut u8, (addr << 16) as u8);
            store(USB_BDTPAGE3 as *mut u8, (addr << 24) as u8);
        }
        
        // Enable the transceiver and disable pulldowns
        unsafe { store(USB_USBCTRL as *mut u8, 0x00); }

        // Run the software USB reset
        self.reset();

        // Enable interrupts for USBRST, TOKDNE and STALL
        self.set_interrupts(STALLEN as u8 |
                      TOKDNEEN as u8 |
                      USBRSTEN as u8);
    }

    /// Attaches the device to the bus
    /// Enables relevant pullup
    fn attach(&self) {
        // Enable the DP pullup
        unsafe { set(USB_CONTROL as *mut u8, 0x10); }
    }

    /// Moves peripheral to default state
    /// Reset address, reset all endpoints,
    /// clear any status flags
    fn reset(&self) {
        unsafe {
            // Disable and suspend USB module
            set(USB_CTL as *mut u8, 0x22);

            // TODO: Reset pingpong register

            // Clear any remaining status flags
            store(USB_ISTAT as *mut u8, 0xFF);
            store(USB_ERRSTAT as *mut u8, 0xFF);
            store(USB_OTGISTAT as *mut u8, 0xFF);

            // Zero out BDT
            set_memory(self.bdt as *mut u8, 0, self.max_ep*32);
            
            // Reset address
            self.set_address(0x00);

            // Enable USB module
            store(USB_CTL as *mut u8, 0x01);
        }
    }

    fn poll(&self) {
        USB0_Handler();
    }

    fn max_endpoints(&self) -> uint {
        16
    }

    fn queue_next(&mut self, ep: uint, is_tx: bool, stream: &StreamHandler) {
        // Get pingpong status
        let ping_index = ep*2 + if is_tx { 1 } else { 0 };
        let odd = self.ping[ping_index];

        // Check BDT is free
        let stat = self.get_bdt_setting(ep, is_tx, odd);
        if stat & 0x80 > 0 {
            unsafe { abort(); }
        }

        // Transfer StreamHandler into BDT
        let addr = stream.address() as u32;
        self.set_bdt_address(ep, is_tx, odd, addr);
        
        let len = stream.len();
        let data1 = if stream.data1() { 1 } else { 0 };

        // Activate BDT
        let val = ((len & 0x3FF) << 16) |
            0x88 | (data1 << 6);
        let stat = self.set_bdt_setting(ep, is_tx, odd, val as u32);
        
        // Swap pingpong
        self.ping[ping_index] = !odd;
    }

    fn set_address(&self, addr: u8) {
        unsafe {
            store(USB_ADDR as *mut u8, addr);
        }
    }

    fn ep_enable(&self, ep: uint, typ: EndpointType) {
        // Make sure ep is in range
        if ep > 15 {
            unsafe { abort(); }
        }

        // Work out flags required for type
        let val = match typ {
            rustusb::usb::Control         => 0x0D,
            rustusb::usb::TxOnly         => 0x15,
            rustusb::usb::RxOnly         => 0x19,
            rustusb::usb::TxRx           => 0x1D,
            rustusb::usb::IsochronousTx  => 0x14,
            rustusb::usb::IsochronousRx  => 0x18
        };

        // Set enpoint
        let addr = USB_ENDPT0 + (ep as u32)*4;
        unsafe {
            store(addr as *mut u8, val as u8);
        }
    }

    fn ep_stall(&self, ep: uint) {

    }

    fn ep_unstall(&self, ep: uint) {

    }
}

/// Handler for usb interrupts
/// Uses following regs
///   ISTAT - To see which flasg are set and clear them after
///   STAT - To get info on completed transaction
/// Also reads from the BDT specified by STAT register
#[no_mangle]
pub extern "C" fn USB0_Handler() {
    // Check module has been initialised (needed?)
    if !UsbModule::is_ready() {
        return;
    }

    // Get interrupt status
    let istat = unsafe { load(USB_ISTAT) };
    
    // Get module
    let module = UsbModule::get();
    
    // On usbrst call reset and return
    if istat & 1 > 0 {
        module.on_reset();
        return;
    }

    // On stall call stall
    if istat & 0x80 > 0 {
        module.on_stall();
    }

    // On tokdne call out to handle_transaction
    // TODO: Will probably need to work differently for isochronous stuff
    if istat & 0x08 > 0 {
        // Get token info
        let stat = unsafe { load(USB_STAT as *mut u8) as uint };
        let ep = stat >> 4;
        let tx = (stat & 0x08) > 0;
        let odd = (stat & 0x04) > 0;

        let this = FreescaleUsb::get();
        let bdt_info = this.get_bdt_setting(ep, tx, odd);
        let pid = (bdt_info >> 2) & 0x0F;
        let len = (bdt_info >> 16) & 0x3FF;

        module.on_token(ep, tx, pid as uint, len as uint);

        // Update CTL after processing a SETUP token (will have been paused)
        if pid == 0x0D {
            unsafe { store(USB_CTL as *mut u8, 0x01); }
        }
    }

    // Clear flags
    // TODO: Does this need to be more complicated?
    unsafe { store(USB_ISTAT, istat); }
}
