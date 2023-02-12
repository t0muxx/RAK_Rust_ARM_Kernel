// adapted from : https://github.com/andre-richter/qemu-exit/blob/master/src/aarch64.rs
use core::arch::asm;

const EXIT_SUCCESS: u32 = 0;
const EXIT_FAILURE: u32 = 1;

#[allow(non_upper_case_globals)]
const ADP_Stopped_ApplicationExit: u64 = 0x20026;

/// The parameter block layout that is expected by QEMU.
///
/// If QEMU finds `ADP_Stopped_ApplicationExit` in the first parameter, it uses the second parameter
/// as exit code.
///
/// If first paraemter != `ADP_Stopped_ApplicationExit`, exit code `1` is used.
#[repr(C)]
struct QEMUParameterBlock {
    arg0: u64,
    arg1: u64,
}

/// A Semihosting call using `0x18` - `SYS_EXIT`.
fn semihosting_sys_exit_call(block: &QEMUParameterBlock) -> ! {
    unsafe {
        asm!(
            "hlt #0xF000",
            in("x0") 0x18,
            in("x1") block as *const _ as u64,
            options(nostack)
        );

        // For the case that the QEMU exit attempt did not work, transition into an infinite loop.
        // Calling `panic!()` here is unfeasible, since there is a good chance this function here is
        // the last expression in the `panic!()` handler itself. This prevents a possible
        // infinite loop.
        loop {
            asm!("wfe", options(nomem, nostack));
        }
    }
}

fn exit(code: u32) -> ! {
    let block = QEMUParameterBlock {
        arg0: ADP_Stopped_ApplicationExit,
        arg1: code as u64,
    };

    semihosting_sys_exit_call(&block)
}

pub fn exit_success() -> ! {
    exit(EXIT_SUCCESS)
}

pub fn exit_failure() -> ! {
    exit(EXIT_FAILURE)
}
