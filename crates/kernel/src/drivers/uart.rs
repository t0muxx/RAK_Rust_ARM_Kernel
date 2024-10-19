use crate::drivers::mbox;
use crate::drivers::mmio;
use crate::drivers::periph_map;
use crate::{check_bit, clear_bit, set_bit};
use core::arch::asm;
use core::fmt;
use core::fmt::Write;
use core::{arch::global_asm, ptr};

pub const PBASE_MU: usize = periph_map::UART0;

#[allow(non_snake_case)]
pub struct MiniUART {
    /// Auxiliary enables
    AUX_ENABLES: mmio::Register<u32>,
    /// Mini Uart Extra Control
    AUX_MU_CNTL_REG: mmio::Register<u32>,
    /// Mini Uart Interrupt Enable
    AUX_MU_IER_REG: mmio::Register<u32>,
    /// Mini Uart Line Control
    AUX_MU_LCR_REG: mmio::Register<u32>,
    /// Mini Uart Modem Control
    AUX_MU_MCR_REG: mmio::Register<u32>,
    /// Mini Uart Baudrate
    AUX_MU_BAUD_REG: mmio::Register<u32>,
    ///  shows the data status
    AUX_MU_LSR_REG: mmio::Register<u32>,
    /// Mini Uart I/O Data
    AUX_MU_IO_REG: mmio::Register<u32>,
}

impl MiniUART {
    pub fn new() -> Self {
        MiniUART {
            AUX_ENABLES: mmio::Register::new((PBASE_MU + 0x4) as usize),
            AUX_MU_CNTL_REG: (mmio::Register::new((PBASE_MU + 0x60) as usize)),
            AUX_MU_IER_REG: (mmio::Register::new((PBASE_MU + 0x44) as usize)),
            AUX_MU_LCR_REG: (mmio::Register::new((PBASE_MU + 0x4c) as usize)),
            AUX_MU_MCR_REG: (mmio::Register::new((PBASE_MU + 0x50) as usize)),
            AUX_MU_BAUD_REG: (mmio::Register::new((PBASE_MU + 0x68) as usize)),
            AUX_MU_LSR_REG: (mmio::Register::new((PBASE_MU + 0x54) as usize)),
            AUX_MU_IO_REG: (mmio::Register::new((PBASE_MU + 0x40) as usize)),
        }
    }

    pub fn init(&self) {
        // enable mini uart
        self.AUX_ENABLES.write(0x1);
        // disable before configuration is finished.
        self.AUX_MU_CNTL_REG.write(0x0);
        // disable recv and tx interrupts
        // ATM we don't use inter.
        self.AUX_MU_IER_REG.write(0x0);
        // enable 8 bit mode
        self.AUX_MU_LCR_REG.write(3);
        self.AUX_MU_MCR_REG.write(0x0);
        // set baud ret (TODO : a creuser)
        self.AUX_MU_BAUD_REG.write(270);
        // enable rx and tx
        self.AUX_MU_CNTL_REG.write(0x3);
    }

    pub fn send(&self, c: char) {
        loop {
            let val = self.AUX_MU_LSR_REG.read();
            if check_bit!(val, 0) {
                // data is ready
                break;
            }
            self.AUX_MU_IO_REG.write(c.into());
        }
    }

    pub fn send_str(&self, string: &str) {
        for chr in string.chars() {
            self.send(chr);
        }
    }

    pub fn recv(&self) -> char {
        loop {
            let val = self.AUX_MU_LSR_REG.read();
            if check_bit!(val, 1) {
                // data is ready
                break;
            }
        }
        let chr = self.AUX_MU_IO_REG.read();
        ((chr as u8) & 0xFF) as u8 as char
    }
}

pub const PBASE_UPL011: usize = periph_map::PBASE + 0x201000;

#[allow(non_snake_case)]
pub struct UARTPL011 {
    /// Data register
    DR: mmio::Register<u32>,
    /// ...
    RSRECR: mmio::Register<u32>,
    /// Flag register
    FR: mmio::Register<u32>,
    // not used
    // ILPR
    /// Integer Baud rate divisor
    IBRD: mmio::Register<u32>,
    /// Fractional Baud rate divisor
    FBRD: mmio::Register<u32>,
    /// Line Ccontrol register
    LCRH: mmio::Register<u32>,
    CR: mmio::Register<u32>,
    IFLS: mmio::Register<u32>,
    IMSC: mmio::Register<u32>,
    RIS: mmio::Register<u32>,
    MIS: mmio::Register<u32>,
    ICR: mmio::Register<u32>,
    DMACR: mmio::Register<u32>,
    ITCR: mmio::Register<u32>,
    ITIP: mmio::Register<u32>,
    ITOP: mmio::Register<u32>,
    TDR: mmio::Register<u32>,
}

impl UARTPL011 {
    pub fn new() -> Self {
        UARTPL011 {
            DR: mmio::Register::new(PBASE_UPL011 + 0x0),
            RSRECR: mmio::Register::new(PBASE_UPL011 + 0x4),
            FR: mmio::Register::new(PBASE_UPL011 + 0x18),
            IBRD: mmio::Register::new(PBASE_UPL011 + 0x24),
            FBRD: mmio::Register::new(PBASE_UPL011 + 0x28),
            LCRH: mmio::Register::new(PBASE_UPL011 + 0x2c),
            CR: mmio::Register::new(PBASE_UPL011 + 0x30),
            IFLS: mmio::Register::new(PBASE_UPL011 + 0x34),
            IMSC: mmio::Register::new(PBASE_UPL011 + 0x38),
            RIS: mmio::Register::new(PBASE_UPL011 + 0x3c),
            MIS: mmio::Register::new(PBASE_UPL011 + 0x40),
            ICR: mmio::Register::new(PBASE_UPL011 + 0x44),
            DMACR: mmio::Register::new(PBASE_UPL011 + 0x48),
            ITCR: mmio::Register::new(PBASE_UPL011 + 0x80),
            ITIP: mmio::Register::new(PBASE_UPL011 + 0x84),
            ITOP: mmio::Register::new(PBASE_UPL011 + 0x88),
            TDR: mmio::Register::new(PBASE_UPL011 + 0x8c),
        }
    }

    pub fn init(&self) {
        // 1. Disable the UART.
        self.CR.write(0);
        self.ICR.write(0x7FF);
        let mbox = mbox::Mailbox::new();
        mbox.send_msg();
        self.set_baudrate();
        let mut val = 0;
        self.set_lchr();
        // UARTEN enable
        set_bit!(val, 0);
        // RXE enable
        set_bit!(val, 9);
        // TXE enable
        set_bit!(val, 8);
        // disabled all interrupts atm.
        self.IMSC.write(0);
        self.CR.write(val);
    }

    /// Set parity SPS to 0. (Stick parity disabled)
    /// Enable FIFO and 8 bit transmission. (1 stop bit).
    /// TODO : Module function to set either :
    /// - None
    /// Set word lenght. Number of data bits
    /// transmitted or received in a frame.
    pub fn set_lchr(&self) {
        let mut val = self.LCRH.read();
        set_bit!(val, 4);
        set_bit!(val, 5);
        set_bit!(val, 6);
        self.LCRH.write(val);
    }

    /// TODO understand that !.
    pub fn set_baudrate(&self) {
        // On raspberry the clock is : 30000000
        let bauddiv: u64 = 3000000 / 16 / 115200;
        // 3000000 / (16 * 115200) = 1.627 = ~1
        let ibrd = 3;
        // Fractional part register = (.627 * 64) + 0.5 = 40.6 = ~40.
        let mut fbrd = 16;
        self.IBRD.write(ibrd as u32);
        self.FBRD.write(fbrd as u32);
    }

    pub fn send(&self, c: char) {
        while check_bit!(self.FR.read(), 5) {
            unsafe {
                asm!("nop");
            }
        }
        self.DR.write(c.into());
    }

    pub fn flush(&self) {
        while check_bit!(self.FR.read(), 3) {
            unsafe {
                asm!("nop");
            }
        }
    }

    pub fn send_str(&self, string: &str) {
        for chr in string.chars() {
            self.send(chr);
        }
    }

    pub fn recv(&self) -> char {
        while check_bit!(self.FR.read(), 4) {
            unsafe {
                asm!("nop");
            }
        }
        let mut chr = (self.DR.read() & 0xFF) as u8 as char;
        if chr == '\r' {
            chr = '\n';
        }
        ((chr as u8) & 0xFF) as u8 as char
    }
}

impl fmt::Write for UARTPL011 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.send_str(s);
        Ok(())
    }
}
