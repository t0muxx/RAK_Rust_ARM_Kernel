#![feature(format_args_nl)]
#![no_std]
#![no_main]

// 0x3F003000 - System Timer
// 0x3F00B000 - Interrupt controller
// 0x3F00B880 - VideoCore mailbox
// 0x3F100000 - Power management
// 0x3F104000 - Random Number Generator
// 0x3F200000 - General Purpose IO controller
// 0x3F201000 - UART0 (serial port, PL011)
// 0x3F215000 - UART1 (serial port, AUX mini UART)
// 0x3F300000 - External Mass Media Controller (SD card reader)
// 0x3F980000 - Universal Serial Bus controller

pub mod gpio;
pub mod mbox;
pub mod mmio;
pub mod uart;

#[cfg(feature = "qemu")]
mod periph_map {
    /// Peripheral base address.
    /// Differs from qemu and Raspberry 3.
    pub const PBASE: usize = 0x3F00_0000;
}

#[cfg(feature = "raspberry")]
mod periph_map {
    /// Peripheral base address for Raspberry 3.
    pub const PBASE: usize = 0x7E00_0000;
}

#[cfg(test)]
mod tests {
    //    use super::*;
}
