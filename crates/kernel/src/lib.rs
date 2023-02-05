#![feature(linkage)]
#![macro_use]
#![feature(format_args_nl)]
#![no_std]
// testing
// No main when cargo test is run.
#![cfg_attr(test, no_main)]
// Enable custom test frameworks
#![feature(custom_test_frameworks)]
// We change the name of the test main function.
#![reexport_test_harness_main = "test_main"]
// we set this runner
#![test_runner(crate::test_runner)]

use core::arch::asm;
use core::{arch::global_asm, ptr};
pub mod cpu;
pub mod drivers;
pub mod log;
pub mod panic;
pub mod syncronization;
pub mod utils;

/// The default runner for unit tests.
pub fn test_runner(tests: &[&test_types::UnitTest]) {
    // This line will be printed as the test header.
    println!("Running {} tests", tests.len());

    for (i, test) in tests.iter().enumerate() {
        print!("{:>3}. {:.<58}", i + 1, test.name);

        // Run the actual test.
        (test.test_func)();

        // Failed tests call panic!(). Execution reaches here only if the test has passed.
        println!("[ok]")
    }
}

/// The point of exit for `libkernel`.
///
/// It is linked weakly, so that the integration tests can overload its standard behavior.
#[linkage = "weak"]
#[no_mangle]
fn _panic_exit() -> ! {
    #[cfg(not(feature = "test_build"))]
    {
        cpu::wait_forever()
    }

    #[cfg(feature = "test_build")]
    {
        cpu::qemu::exit_failure()
    }
}

#[cfg(test)]
global_asm!(include_str!("start.s"));

#[cfg(feature = "test_build")]
#[no_mangle]
pub extern "C" fn entry() {
    let drivers = drivers::Drivers::new();
    drivers.init();
    test_main();
    cpu::qemu::exit_success();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_macro::kernel_test;

    #[kernel_test]
    fn test_kernel_1() {
        assert!(1 == 1);
    }
}
