//! aarch64 cortex a53 syncronization modules.

// Testing
pub mod atomic;
pub mod mutex_spinlock;

// TODO : Test with multithreading...
// and MMU set.
/*
#[cfg(test)]
mod tests {
    use super::*;
    use atomic::AtomicUsize;
    use core::arch::asm;
    use test_macro::kernel_test;

    #[kernel_test]
    fn test_atomic_load() {
        let a = AtomicUsize::new(0);
        assert!(a.load() == 0);
        a.store(10);
        //assert!(a.load() == 10);
    }
}*/
