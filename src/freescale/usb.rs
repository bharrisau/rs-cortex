
extern mod cortex;

use cortex::regs::{store, set, wait_for};
use sim::{enable_clock, USBOTG};
use sim::{select_usb_source};

mod sim;

static BASE_USB: u32        = 0x4007_2000;
static USB_USBTRC0: u32     = BASE_USB + 0x010C;
static USB_USBCTRL: u32     = BASE_USB + 0x0100;
static USB_CTL: u32         = BASE_USB + 0x0094;
static USB_ADDR: u32        = BASE_USB + 0x0098;
static USB_CONTROL: u32     = BASE_USB + 0x0108;
static USB_INTEN: u32       = BASE_USB + 0x0084;

static USB_ISTAT: u32       = BASE_USB + 0x0080;
static USB_ERRSTAT: u32     = BASE_USB + 0x0088;
static USB_OTGISTAT: u32    = BASE_USB + 0x0010;

static USB_BDTPAGE1: u32    = BASE_USB + 0x009C;
static USB_BDTPAGE2: u32    = BASE_USB + 0x00B0;
static USB_BDTPAGE3: u32    = BASE_USB + 0x00B4;

pub enum Usb_Int {
    USBRSTEN = 0x01,
    ERROREN  = 0x02,
    SOFTOKEN = 0x04,
    TOKDNEEN = 0x08,
    SLEEPEN  = 0x10,
    RESUMEEN = 0x20,
    ATTACHEN = 0x40,
    STALLEN  = 0x80
}

fn zero_bdt() {
}

fn set_bdt() {
    // Need a 512 byte aligned memory
    let addr: u32 = 0x00; // This should be a pointer to the BDT
    unsafe {
        store(USB_BDTPAGE1 as *mut u8, (addr << 8) as u8);
        store(USB_BDTPAGE2 as *mut u8, (addr << 16) as u8);
        store(USB_BDTPAGE3 as *mut u8, (addr << 24) as u8);
    }
}

fn usb_reset_hard() {
    unsafe {
        let addr: *mut u8 = (BASE_USB + 0x010C) as *mut u8;
        store(addr, 0x80);
        wait_for(addr as *u8, 0x808, 0x00);
    }
}

pub fn set_interrupt(val: Usb_Int) {
    unsafe {
        set(USB_INTEN as *mut u8, val as u8);
    }
}

pub fn set_interrupts(val: u8) {
    unsafe {
        store(USB_INTEN as *mut u8, val as u8);
    }
}

pub fn usb_address(addr: u8) {
    unsafe {
        store(USB_ADDR as *mut u8, addr);
    }
}

pub fn usb_reset(on_before_enable: ||) {
    unsafe {
        // Disable and suspend USB module
        set(USB_CTL as *mut u8, 0x22);

        // Clear any remaining status flags
        store(USB_ISTAT as *mut u8, 0xFF);
        store(USB_ERRSTAT as *mut u8, 0xFF);
        store(USB_OTGISTAT as *mut u8, 0xFF);

        // Zero out BDT
        zero_bdt();
        
        // Reset address
        usb_address(0x00);

        // Call supplied function (should initialise EP0)
        on_before_enable();

        // Enable USB module
        store(USB_CTL as *mut u8, 0x01);
    }
}

pub fn usb_init(on_before_enable: ||) {
    // Enable the SIM clock for USB
    enable_clock(USBOTG);

    // Set the USB clock source to internal
    select_usb_source(true);

    // Cycle the USB module through a hard reset
    usb_reset_hard();

    // Load the address of the BDT
    set_bdt();
    
    // Enable the DP pullup
    unsafe { set(USB_CONTROL as *mut u8, 0x10); }
    
    // Enable the transceiver and disable pulldowns
    unsafe { store(USB_USBCTRL as *mut u8, 0x00); }

    // Run the software USB reset
    usb_reset(on_before_enable);

    // Enable interrupts for USBRST, TOKDNE and STALL
    set_interrupts(STALLEN as u8 |
                  TOKDNEEN as u8 |
                  USBRSTEN as u8);
}
