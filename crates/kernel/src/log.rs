use core::fmt::Write;
use core::{arch::asm, fmt};

use crate::drivers::{
    device_tree::{self, BASE_ADDR_DEVICETREE},
    uart::{self, UARTPL011},
    Driver,
};

/// Can be enhanced by using global to access drivers.
pub fn _print(args: fmt::Arguments) {
    //UARTPL011::new().unwrap().write_fmt(args).unwrap();
    // TODO : Enable when woring uart
    let dt = device_tree::DeviceTree::new(BASE_ADDR_DEVICETREE);
    UARTPL011::new(&dt).write_fmt(args);
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::log::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    () => ($crate::printt!("\n"));
    ($($arg:tt)*) => ({
        $crate::log::_print(format_args_nl!($($arg)*));
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
        $crate::log::_print(format_args!("[i] [{:.10}] - ", $crate::log::get_current_timer()));
        $crate::log::_print(format_args_nl!($($arg)*));

    }};
}
