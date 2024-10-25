// Address Field Name Description Size Read/Write
// 0x 7E20 0000 GPFSEL0 GPIO Function Select 0 32 R/W
// 0x 7E20 0004 GPFSEL1 GPIO Function Select 1 32 R/W
// 0x 7E20 0008 GPFSEL2 GPIO Function Select 2 32 R/W
// 0x 7E20 000C GPFSEL3 GPIO Function Select 3 32 R/W
// 0x 7E20 0010 GPFSEL4 GPIO Function Select 4 32 R/W
// 0x 7E20 0014 GPFSEL5 GPIO Function Select 5 32 R/W
// 0x 7E20 0018 - Reserved - -
// 0x 7E20 001C GPSET0 GPIO Pin Output Set 0 32 W
// 0x 7E20 0020 GPSET1 GPIO Pin Output Set 1 32 W
// 0x 7E20 0024 - Reserved - -
// 0x 7E20 0028 GPCLR0 GPIO Pin Output Clear 0 32 W
// 0x 7E20 002C GPCLR1 GPIO Pin Output Clear 1 32 W
// 0x 7E20 0030 - Reserved - -
// 0x 7E20 0034 GPLEV0 GPIO Pin Level 0 32 R
// 0x 7E20 0038 GPLEV1 GPIO Pin Level 1 32 R
// 0x 7E20 003C - Reserved - -
// 0x 7E20 0040 GPEDS0 GPIO Pin Event Detect Status 0 32 R/W
// 0x 7E20 0044 GPEDS1 GPIO Pin Event Detect Status 1 32 R/W
// 0x 7E20 0048 - Reserved - -
// 0x 7E20 004C GPREN0 GPIO Pin Rising Edge Detect Enable 0 32 R/W
// 0x 7E20 0050 GPREN1 GPIO Pin Rising Edge Detect Enable 1 32 R/W
// 0x 7E20 0054 - Reserved - -
// 0x 7E20 0058 GPFEN0 GPIO Pin Falling Edge Detect Enable 0 32 R/W
// 0x 7E20 005C GPFEN1 GPIO Pin Falling Edge Detect Enable 1 32 R/W
//

use crate::drivers::mmio;
use crate::drivers::periph_map;
use crate::utils::delay;
use crate::{clear_bit, set_bit};

pub const GPIO_PBASE: usize = periph_map::PBASE + 0x200000;

#[allow(non_snake_case)]
pub struct GPIO {
    /// GPIO Function Select 1
    GPFSEL1: mmio::Register<u32>,
    GPPUD: mmio::Register<u32>,
    GPPUDCLK0: mmio::Register<u32>,
}

impl GPIO {
    pub fn new() -> Self {
        GPIO {
            GPFSEL1: mmio::Register::new((GPIO_PBASE + 0x4) as usize),
            GPPUD: mmio::Register::new((GPIO_PBASE + 0x94) as usize),
            GPPUDCLK0: mmio::Register::new((GPIO_PBASE + 0x98) as usize),
        }
    }

    /// Set GPIO pin 14 to use alternate function 5 : `TXD1` (UART 1 Transmit Data)
    pub fn set_alt5_gpio14(&self) {
        let mut val;
        val = self.GPFSEL1.read();
        clear_bit!(val, 12);
        clear_bit!(val, 13);
        set_bit!(val, 14);
        self.GPFSEL1.write(val);
    }

    /// Set GPIO pin 14 to use alternate function 5 : `RXD1` (UART 1 Receive Data)
    pub fn set_alt5_gpio15(&self) {
        let mut val: u32;
        val = self.GPFSEL1.read();
        clear_bit!(val, 15);
        clear_bit!(val, 16);
        set_bit!(val, 17);
        self.GPFSEL1.write(val);
    }

    /// Clear pull-up and pull-down for GPIO 0 and 31.
    pub fn clear_pu_pd_clk0(&self, pin: u8) {
        self.GPPUD.write(0);
        unsafe {
            delay(150);
        }
        self.GPPUDCLK0.write(1 << pin);
        unsafe {
            delay(150);
        }
        self.GPPUD.write(0);
    }
}
