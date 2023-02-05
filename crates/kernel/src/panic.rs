use crate::cpu::qemu;
use crate::println;
use core::panic::PanicInfo;

// `!` -> indicate that the function never returns
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(feature = "test_build")]
    {
        qemu::exit_failure()
    }
    loop {
        println!("panic");
    }
}
