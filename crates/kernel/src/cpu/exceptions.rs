//! exceptions management module.

use core::arch::asm;
use core::arch::global_asm;
use core::fmt;

use crate::{ilog, println};

global_asm!(include_str!("exceptions.s"));

/// Structure containing saved context.
/// We uses C representation as we will
/// push the context on the stack in our exception vector.
/// Then we put stack pointer into `x0` for our rust handler.
#[repr(C)]
struct ExceptionContext {
    /// general registers
    gpr: [u64; 30],
    /// link registers
    lr: u64,
    /// Exception syndrome register
    esr: u64,
    /// stored process state
    spsr: u64,
    /// fault address register
    far: u64,
    /// address causing the excetpion.
    elr: u64,
}

impl fmt::Display for ExceptionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#######################################\n")?;
        write!(f, "Context : \n");
        for mut i in 0..15 {
            write!(
                f,
                "x{i:<2} : 0x{: <24x} x{} : 0x{:x}\n",
                self.gpr[i],
                i + 1,
                self.gpr[i + 1]
            );
            i += 1;
        }
        write!(f, "lr  : 0x{: <24x}\n", self.lr);
        write!(f, "esr : 0x{: <25x}", self.esr);
        write!(f, "spsr : 0x{:x}\n", self.spsr);
        write!(f, "far : 0x{: <25x}", self.far);
        write!(f, "elr : 0x{:x}\n", self.elr);
        write!(f, "#######################################\n")?;

        Ok(())
    }
}

fn default_handler(_econtext: &mut ExceptionContext) {
    println!("{}", _econtext);
    _econtext.elr += 4;
}

#[no_mangle]
extern "C" fn el1t_synchronous(_econtext: &mut ExceptionContext) {
    println!("el1t_synchronous");
    default_handler(_econtext);
}

#[no_mangle]
extern "C" fn el1t_irq(_econtext: &mut ExceptionContext) {
    println!("el1t_irq");
    default_handler(_econtext);
}

#[no_mangle]
extern "C" fn el1t_serror(_econtext: &mut ExceptionContext) {
    println!("el1t_serror");
    default_handler(_econtext);
}

#[no_mangle]
extern "C" fn el1h_synchronous(_econtext: &mut ExceptionContext) {
    println!("el1t_synchronous");
    default_handler(_econtext);
}

fn handle_irq() {}

#[no_mangle]
extern "C" fn el1h_irq(_econtext: &mut ExceptionContext) {
    println!("el1h_irq : \n {}", _econtext);
    handle_irq();
}

#[no_mangle]
extern "C" fn el1h_serror(_econtext: &mut ExceptionContext) {
    println!("el1h_serror");
    default_handler(_econtext);
}

#[no_mangle]
extern "C" fn el0_aarch64_synchronous(_econtext: &mut ExceptionContext) {
    println!("el0_aarch64_synchronous");
    default_handler(_econtext);
}

#[no_mangle]
extern "C" fn el0_aarch64_irq(_econtext: &mut ExceptionContext) {
    println!("el0_aarch64_irq");
    default_handler(_econtext);
}

#[no_mangle]
extern "C" fn el0_aarch64_serror(_econtext: &mut ExceptionContext) {
    println!("el0_aarch64_serror");
    default_handler(_econtext);
}

#[no_mangle]
extern "C" fn el0_aarch32_synchronous(_econtext: &mut ExceptionContext) {
    println!("el0_aarch32_synchronous");
    default_handler(_econtext);
}

#[no_mangle]
extern "C" fn el0_aarch32_irq(_econtext: &mut ExceptionContext) {
    println!("el0_aarch32_irq");
    default_handler(_econtext);
}

#[no_mangle]
extern "C" fn el0_aarch32_serror(_econtext: &mut ExceptionContext) {
    println!("el0_aarch32_serror");
    default_handler(_econtext);
}
