// https://www.qemu.org/docs/master/system/arm/virt.html#hardware-configuration-information-for-bare-metal-programming
//https://github.com/qemu/qemu/blob/master/hw/arm/virt.c

#![feature(format_args_nl)]
// We do not want standar library linked.
#![no_std]
// Use a custom entrypoint that will be defined
// in linker script.
#![no_main]

use libkernel::{
    drivers::{
        device_tree::{self, BASE_ADDR_DEVICETREE},
        G_DRIVERS,
    },
    ilog,
};

use core::arch::global_asm;

use libkernel::cpu;

global_asm!(include_str!("start.s"));

// disable name mangling or `entry` symbol will not be found!
#[no_mangle]
pub extern "C" fn entry() {
    el1_run();
}

#[no_mangle]
pub extern "C" fn el1_run() {
    let ret = cpu::el::get_current_el();
    let dt = device_tree::DeviceTree::new(BASE_ADDR_DEVICETREE);
    let mut drivers = G_DRIVERS.lock().unwrap();
    drivers.init(&dt);
    drop(drivers);
    ilog!("el {ret}");
    loop {
        drivers = G_DRIVERS.lock().unwrap();
        let chr = drivers.uart.recv();
        drivers.uart.send(chr);
        drivers.uart.send('\n');
        drop(drivers);
        ilog!("end");
    }
}
