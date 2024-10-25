use core::arch::asm;
use core::arch::global_asm;

#[macro_export]
macro_rules! clear_bit {
    ( $num:expr, $bit:expr ) => {
        $num &= !(1 << $bit);
    };
}

#[macro_export]
macro_rules! set_bit {
    ( $num:expr, $bit:expr ) => {
        $num |= (1 << $bit);
    };
}

/// Return True if bit is up !
#[macro_export]
macro_rules! check_bit {
    ( $num:expr, $bit:expr ) => {
        ($num & (1 << $bit)) != 0
    };
}

global_asm!(include_str!("utils.s"));

extern "C" {
    pub fn delay(delay: u64);
}

pub fn infloop() {
    unsafe { asm!("1:", "cmp x20, 123", "bne 1b",) }
}
