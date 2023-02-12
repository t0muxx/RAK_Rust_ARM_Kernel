//! Helper around cpu operations
//!

use core::arch::asm;

pub mod el;
pub mod qemu;

/// Make the host QEMU binary execute `exit(1)`.
#[cfg(feature = "test_build")]
pub fn qemu_exit_failure() -> ! {
    qemu::exit_failure()
}

/// Make the host QEMU binary execute `exit(0)`.
#[cfg(feature = "test_build")]
pub fn qemu_exit_success() -> ! {
    qemu::exit_success()
}

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_macro::kernel_test;

    #[kernel_test]
    fn test_get_current_el() {
        let ret = el::get_current_el();
        assert!(ret == 3);
    }
}
