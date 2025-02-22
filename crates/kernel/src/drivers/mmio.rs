use core::ptr;

/// Helper around memory mapped register.
pub struct Register<T> {
    /// Contains memory address of the register.
    address: *mut T,
}

impl<T> Register<T> {
    pub const fn new(address: usize) -> Self {
        Register {
            address: address as *mut T,
        }
    }

    /// Read value from register.
    /// Can be used to read `T` (u8, u16, u32 etc...)
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.address) }
    }

    /// Write value into register.
    pub fn write(&self, val: T) {
        unsafe { ptr::write_volatile(self.address, val) }
    }
}

unsafe impl<T> Send for Register<T> where T: Send {}
unsafe impl<T> Sync for Register<T> where T: Send {}
