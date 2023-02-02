// https://www.qemu.org/docs/master/system/arm/virt.html#hardware-configuration-information-for-bare-metal-programming
//https://github.com/qemu/qemu/blob/master/hw/arm/virt.c

// We do not want standar library linked.
#![no_std]
// Use a custom entrypoint that will be defined
// in linker script.
#![no_main]
#![feature(format_args_nl)]

extern crate drivers;

mod panic;

use core::arch::asm;
use core::{arch::global_asm, ptr};
use drivers::{ilog, println};

global_asm!(include_str!("start.s"));

// disable name mangling or `entry` symbol will not be found!
#[no_mangle]
pub extern "C" fn entry() {
    let gpio = drivers::gpio::GPIO::new();
    gpio.set_alt5_gpio14();
    gpio.set_alt5_gpio15();
    gpio.clear_pu_pd_clk0(14);
    gpio.clear_pu_pd_clk0(15);

    let mut uart = drivers::uart::UARTPL011::new();
    uart.init();
    ilog!(&mut uart, "uart init");
    unsafe {
        loop {
            let chr = uart.recv();
            uart.send(chr);
            //uart.flush();
            //uart.send('\n');
        }
    }
}
