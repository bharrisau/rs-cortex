
use cortex::regs::{set, clear};

pub static BASE_SIM: u32 = 0x4004_7000;

pub enum Clock {
    DAC0    = 0x102C_0C,
    FTM2    = 0x1030_18,
    ADC1    = 0x1030_1B,
    EWM     = 0x1034_01,
    CMT     = 0x1034_02,
    I2C0    = 0x1034_06,
    I2C1    = 0x1034_07,
    UART0   = 0x1034_0A,
    UART1   = 0x1034_0B,
    UART2   = 0x1034_0C,
    USBOTG  = 0x1034_12,
    CMP     = 0x1034_13,
    VREF    = 0x1034_14,
    LPTIMER = 0x1038_00,
    TSI     = 0x1038_05,
    PORTA   = 0x1038_09,
    PORTB   = 0x1038_0A,
    PORTC   = 0x1038_0B,
    PORTD   = 0x1038_0C,
    PORTE   = 0x1038_0D,
    FTFL    = 0x103C_00,
    DMAMUX  = 0x103C_01,
    FLEXCAN0 = 0x103C_04,
    SPI0    = 0x103C_0C,
    SPI1    = 0x103C_0D,
    I2S     = 0x103C_0F,
    CRC     = 0x103C_12,
    USBDCD  = 0x103C_15,
    PDB     = 0x103C_16,
    PIT     = 0x103C_17,
    FTM0    = 0x103C_18,
    FTM1    = 0x103C_19,
    ADC0    = 0x103C_1B,
    RTC     = 0x103C_1D,
    DMA     = 0x1040_01
}

pub fn enable_clock(clock: Clock) {
    let addr = BASE_SIM + ((clock as u32) >> 8);
    let mask: u32 = 1 << ((clock as u32) & 31);

    unsafe {
        set(addr as *mut u32, mask);
    }
}

pub fn select_usb_source(internal: bool) {
    let addr = BASE_SIM + 0x1004;
    let mask = 1 << 18;
    unsafe {
        if internal {
            set(addr as *mut u32, mask);
        } else {
            clear(addr as *mut u32, mask);
        }
    }
}
