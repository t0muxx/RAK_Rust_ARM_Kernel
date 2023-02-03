// https://www.qemu.org/docs/master/system/arm/virt.html#hardware-configuration-information-for-bare-metal-programming
//https://github.com/qemu/qemu/blob/master/hw/arm/virt.c

// We do not want standar library linked.
#![no_std]
// Use a custom entrypoint that will be defined
// in linker script.
#![no_main]
#![feature(format_args_nl)]

mod drivers;
pub mod log;
mod panic;

use core::arch::asm;
use core::{arch::global_asm, ptr};

global_asm!(include_str!("start.s"));

// disable name mangling or `entry` symbol will not be found!
#[no_mangle]
pub extern "C" fn entry() {
    let drivers = drivers::Drivers::new();
    drivers.init();
    ilog!("{} - uart init", drivers.systimer.get());
    unsafe {
        loop {
            let chr = drivers.uart.recv();
            drivers.uart.send(chr);
            ilog!("timer : {}", drivers.systimer.get());
            //uart.flush();
            //uart.send('\n');
        }
    }
}
