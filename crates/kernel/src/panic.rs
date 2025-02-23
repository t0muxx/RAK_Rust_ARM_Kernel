#[cfg(feature = "test_build")]
use crate::cpu::qemu;
use crate::{log::_print_panic, print_panic, println};
use core::panic::PanicInfo;

// `!` -> indicate that the function never returns
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(feature = "test_build")]
    {
        println!("\n\n\t{:?}", info);
        qemu::exit_failure()
    }
    loop {
        print_panic!("\n\n\t{:?}", info);
    }
}
