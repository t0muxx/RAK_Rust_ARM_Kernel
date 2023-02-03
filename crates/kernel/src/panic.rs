use core::panic::PanicInfo;

#[panic_handler]
// `!` -> indicate that the function never returns
fn on_panic(_info: &PanicInfo) -> ! {
    loop {}
}
