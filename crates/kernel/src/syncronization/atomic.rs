//! Atomic operation implementation based on : https://whenderson.dev/blog/implementing-atomics-in-rust/
//! spinlock based.

use core::arch::asm;
use core::cell::UnsafeCell;
use core::marker::Send;
use core::marker::Sync;
use core::result::Result;
use core::result::Result::Err;
use core::result::Result::Ok;

use crate::ilog;

pub struct AtomicUsize {
    inner: UnsafeCell<usize>,
}

unsafe impl Send for AtomicUsize {}
unsafe impl Sync for AtomicUsize {}

impl AtomicUsize {
    pub fn new(v: usize) -> Self {
        AtomicUsize {
            inner: UnsafeCell::new(v),
        }
    }

    /// Return the value
    pub fn load(&self) -> usize {
        unsafe { *self.inner.get() }
    }

    pub fn store(&self, v: usize) {
        unsafe {
            asm!("1:", "mov x18, 0", "cmp x18, 1", "bne 1b",);
        }
        unsafe {
            asm!(
                "1:",
                "stxr w3, {v}, [{address}]",
                "cbnz w3, 1b",
                address = in(reg) self.inner.get(),
                v = in(reg) v
            );
        }
    }

    pub fn fetch_add(&self, mut v: usize) -> usize {
        unsafe {
            asm!(
                "1:",
                "ldxr x2, [{address}]",
                "add x1, x2, {v}",
                "stxr w3, x1, [{address}]",
                "cbnz w3, 1b",
                address = in(reg) self.inner.get(),
                v = inout(reg) v
            );
        }
        v
    }
    /// Set the value and return the old value.
    pub fn swap(&self, mut v: usize) -> usize {
        let mut old: usize;
        unsafe {
            asm!(
                "1:",
                "ldxr {old}, [{address}]",
                "stxr w3, {v}, [{address}]",
                "cbnz w3, 1b",
                address = in(reg) self.inner.get(),
                v = in(reg) v,
                old = out(reg) old,
            );
        }
        old
    }

    ///  if value matches a given expected value : set the val while return old value
    ///  else return current value
    /// ```ignore
    /// result = atomic.val()
    /// if current == atomic.val():
    ///	atomic.val() = new
    /// result = new
    /// return result
    /// ```
    pub fn compare_exchange(&self, current: usize, new: usize) -> Result<usize, usize> {
        let mut result: usize;
        unsafe {
            asm!(
                "ldxr x3, [{address}]",
                "cmp x3, {current}",
                "bne 2f",
                "1:",
                "stxr w4, {new}, [{address}]",
                "cbnz w4, 1b",
                "mov x3, {new}",
                "2:",
                "mov {result}, x3",
                address = in(reg) self.inner.get(),
                new = in(reg) new,
                current = in(reg) current,
                result = out(reg) result,
            );
        }
        if result == new {
            Ok(result)
        } else {
            Err(result)
        }
    }
}
