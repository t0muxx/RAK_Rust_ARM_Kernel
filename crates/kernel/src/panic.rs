#[cfg(feature = "test_build")]
use crate::cpu::qemu;
use crate::println;
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
        println!("\n\n\t{:?}", info);
    }
}
