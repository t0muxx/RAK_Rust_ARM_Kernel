#[macro_use]
use core::fmt;
use core::fmt::Write;

use crate::drivers::systimer;
use crate::drivers::uart::UARTPL011;

struct Log {}

impl Log {
    pub fn new() -> Self {
        Log {}
    }
}

/// Can be enhanced by using global to access drivers.
pub fn _print(args: fmt::Arguments) {
    UARTPL011::new().write_fmt(args).unwrap();
    // TODO : Enable when woring uart
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

#[macro_export]
macro_rules! ilog {
    ($($arg:tt)*) => {{
        //$crate::log::_print(format_args!("[i] [{:.10}] - ", $crate::drivers::systimer::SysTimer::new().get_second()));
        $crate::log::_print(format_args_nl!($($arg)*));

    }};
}
