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

use core::arch::asm;

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {}
}
