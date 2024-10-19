//! exception level Helper.
use core::arch::asm;

use crate::utils::infloop;
const SCR_EL1_MMU_DISABLED: usize = 0 << 0;
const SCR_EL1_MMU_ENABLED: usize = 1 << 0;
const SCR_EL1_CACHE_DISABLED: usize = 0 << 2;
const SCR_EL1_CACHE_ENABLED: usize = 1 << 2;
const SCR_EL1_ICACHE_DISABLED: usize = 0 << 12;
const SCR_EL1_ICACHE_ENABLED: usize = 1 << 12;
const SCR_EL1_RESERVED1: usize = (3 << 28) | (3 << 22) | (1 << 20) | (1 << 11);
const SCR_EL1_EOE_LITTLEE: usize = 0 << 24;
const SCR_EL1_EE_LITTLEE: usize = 0 << 25;

/// FPEN floating point no trap.
const CPACR_EL1_FPEN: usize = 3 << 20;

//const HCR_EL2_DISABLE_CACHE: usize = 3 << 32;
const HCR_EL2_RW_AARCH64: usize = 1 << 31;

const SCR_EL3_RW_AARCH64: usize = 1 << 10;
const SCR_EL3_REVERSED1: usize = 3 << 4;
// non secure bit
const SCR_EL3_NS: usize = 1 << 0;

const SPSR_MASK_ALL: usize = 7 << 6;

pub fn get_current_el() -> u64 {
    let el: u64;
    unsafe {
        asm!(
        "mrs {el}, CurrentEL",
        "lsr {el}, {el}, #2",
        el = out(reg) el,
        );
    }
    el
}

type Eretfn = extern "C" fn();

/// switch from el3 to el2
/// retf is a function address passed to elr_el3.
/// in Qemu with `-kernel` option, the code starts in EL2 directly. It
/// is not need to switch from el3 to  el2 (and special register are undefined)
/// https://stackoverflow.com/questions/71006360/qemu-system-aarch64-always-entering-exception-level-2-and-hangs-when-switching-t
#[no_mangle]
pub extern "C" fn switch_el2(retf: Eretfn) {
    let hcr_val: usize = HCR_EL2_RW_AARCH64;
    let spsr_val: usize = SPSR_MASK_ALL | (9 << 0);
    let sctlr_el1: usize = SCR_EL1_MMU_DISABLED
        | SCR_EL1_CACHE_DISABLED
        | SCR_EL1_ICACHE_DISABLED
        | SCR_EL1_RESERVED1
        | SCR_EL1_EOE_LITTLEE
        | SCR_EL1_EE_LITTLEE;
    let scr_el3: usize = SCR_EL3_RW_AARCH64 | SCR_EL3_NS | SCR_EL3_REVERSED1;
    let cpacr_el1 = CPACR_EL1_FPEN;
    unsafe {
        asm!(
            "msr cpacr_el1, {cpacr_el1}",
            "msr scr_el3, {scr_el3}",
            "msr hcr_el2, {hcrval}",
            "msr spsr_el3, {spsr_val}",
            "msr elr_el3, {retf}",
            "eret",
            cpacr_el1 = in(reg) cpacr_el1,
            scr_el3 = in(reg) scr_el3,
            hcrval = in(reg) hcr_val,
            spsr_val = in(reg) spsr_val,
            retf = in(reg) retf as *const (),
        );
    }
}

/// Switch from el2 to el1.
/// retf is a function address passed to elr_el2.
#[no_mangle]
pub extern "C" fn switch_el1(retf: Eretfn) {
    let hcr_val: usize = HCR_EL2_RW_AARCH64;
    let spsr_val: usize = SPSR_MASK_ALL | (5 << 0);
    let sctlr_el1: usize = SCR_EL1_MMU_DISABLED
        | SCR_EL1_CACHE_DISABLED
        | SCR_EL1_ICACHE_DISABLED
        | SCR_EL1_RESERVED1
        | SCR_EL1_EOE_LITTLEE
        | SCR_EL1_EE_LITTLEE;
    unsafe {
        asm!(
            "msr sctlr_el1, {sctlr_el1}",
            "msr hcr_el2, {hcrval}",
            "msr spsr_el2, {spsr_val}",
            "msr elr_el2, {retf}",
            "eret",
            hcrval = in(reg) hcr_val,
            spsr_val = in(reg) spsr_val,
            retf = in(reg) retf as *const (),
            sctlr_el1 = in(reg) sctlr_el1,
        );
    }
}
