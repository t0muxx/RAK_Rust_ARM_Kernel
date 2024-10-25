// https://www.qemu.org/docs/master/system/arm/virt.html#hardware-configuration-information-for-bare-metal-programming
//https://github.com/qemu/qemu/blob/master/hw/arm/virt.c

#![feature(format_args_nl)]
// We do not want standar library linked.
#![no_std]
// Use a custom entrypoint that will be defined
// in linker script.
#![no_main]

use libkernel::ilog;

use core::arch::global_asm;

use libkernel::{cpu, drivers};

global_asm!(include_str!("start.s"));

// disable name mangling or `entry` symbol will not be found!
#[no_mangle]
pub extern "C" fn entry() {
    el1_run();
}

#[no_mangle]
pub extern "C" fn el1_run() {
    let _ret = cpu::el::get_current_el();
    let mut drivers = drivers::Drivers::new();
    drivers.init();
    //ilog!("el {ret}");
    loop {
        let chr = drivers.uart.recv();
        drivers.uart.send(chr);
        drivers.uart.send('\n');
        ilog!("end");
    }
}
