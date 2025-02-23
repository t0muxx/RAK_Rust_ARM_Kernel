use crate::drivers::device_tree::BASE_ADDR_DEVICETREE;
use crate::drivers::{device_tree, Driver};
use core::fmt::Write;
use core::{arch::asm, fmt};

use crate::drivers::uart::UARTPL011;
use crate::drivers::G_DRIVERS;

/// Can be enhanced by using global to access drivers.
pub fn _print_uart(args: fmt::Arguments) {
    //UARTPL011::new().unwrap().write_fmt(args).unwrap();
    // TODO : Enable when woring uart
    let mut drivers = G_DRIVERS.lock().unwrap();
    if drivers.uart.is_init {
        drivers.uart.write_fmt(args).unwrap();
    }
    drop(drivers);
}

pub fn _print_panic(args: fmt::Arguments) {
    let dt = device_tree::DeviceTree::new(BASE_ADDR_DEVICETREE);
    let mut uart = UARTPL011::new(&dt);
    uart.init(&dt);
    uart.write_fmt(args).unwrap();
}
/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::log::_print_uart(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_panic {
    ($($arg:tt)*) => ($crate::log::_print_panic(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    () => ($crate::printt!("\n"));
    ($($arg:tt)*) => ({
        $crate::log::_print_uart(format_args_nl!($($arg)*));
    })
}

pub fn get_current_timer() -> u64 {
    let timer: u64;
    unsafe {
        asm!(
            "MRS {timer}, CNTPCT_EL0",
            timer = out(reg) timer,
        );
    }
    timer
}

/// INFO logger with timer value
#[macro_export]
macro_rules! ilog {
    ($($arg:tt)*) => {{
        $crate::log::_print_uart(format_args!("[i] [{:.10}] - ", $crate::log::get_current_timer()));
        $crate::log::_print_uart(format_args_nl!($($arg)*));

    }};
}
