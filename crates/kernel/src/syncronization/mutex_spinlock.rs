use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

use core::sync::atomic::AtomicUsize;

pub struct Mutex<T> {
    inner: UnsafeCell<T>,
    /// 0 if unlocked
    /// 1 if locked
    status: AtomicUsize,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

unsafe impl<T> Send for Mutex<T> where T: Send {}
unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner: UnsafeCell::new(inner),
            status: AtomicUsize::new(0),
        }
    }
    pub fn lock(&self) -> Result<MutexGuard<T>, ()> {
        loop {
            match self.status.compare_exchange(
                0,
                1,
                core::sync::atomic::Ordering::Relaxed,
                core::sync::atomic::Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
        Ok(MutexGuard { mutex: self })
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.inner.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.inner.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex
            .status
            .store(0, core::sync::atomic::Ordering::Relaxed);
    }
}
