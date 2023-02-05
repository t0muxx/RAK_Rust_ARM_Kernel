// https://www.qemu.org/docs/master/system/arm/virt.html#hardware-configuration-information-for-bare-metal-programming
//https://github.com/qemu/qemu/blob/master/hw/arm/virt.c

#![feature(format_args_nl)]
// We do not want standar library linked.
#![no_std]
// Use a custom entrypoint that will be defined
// in linker script.
#![no_main]

use core::arch::asm;
use core::{arch::global_asm, ptr};

use libkernel::{cpu, drivers, log, syncronization, utils};

global_asm!(include_str!("start.s"));

// disable name mangling or `entry` symbol will not be found!
#[no_mangle]
pub extern "C" fn entry() {
    let drivers = drivers::Drivers::new();
    drivers.init();
    unsafe {
        loop {
            let chr = drivers.uart.recv();
            drivers.uart.send(chr);
            //uart.flush();
            //uart.send('\n');
        }
    }
}
