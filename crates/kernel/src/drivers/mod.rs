use device_tree::DeviceTree;
use uart::UARTPL011;

use crate::ilog;

pub mod device_tree;
pub mod gic;
pub mod gpio;
pub mod mbox;
pub mod mmio;
pub mod systimer;
pub mod uart;

use crate::syncronization::mutex_spinlock::Mutex;
pub static G_DRIVERS: Mutex<Drivers> = Mutex::new(Drivers::new());

pub trait Driver {
    /// Usually set MMIO registers from base address of the device
    fn new(dt: &DeviceTree) -> Self;
    /// Init the device (set bit in registers etc.)
    fn init(&mut self, dt: &DeviceTree);
    /// de-init the device.
    fn deinit(&mut self);
}

/// Struct that contains drivers for peripherals.
pub struct Drivers {
    // Gpio drivers
    //pub gpio: gpio::GPIO,
    /// Uart drivers
    pub uart: uart::UARTPL011,
    // Systimer drivers
    //pub systimer: systimer::SysTimer,
    //pub gic: gic::Gic,
}

impl Drivers {
    pub const fn new() -> Self {
        Drivers {
            uart: UARTPL011 {
                is_init: false,
                DR: mmio::Register::new(0),
                RSRECR: mmio::Register::new(0),
                FR: mmio::Register::new(0),
                IBRD: mmio::Register::new(0),
                FBRD: mmio::Register::new(0),
                LCR_H: mmio::Register::new(0),
                CR: mmio::Register::new(0),
                IFLS: mmio::Register::new(0),
                IMSC: mmio::Register::new(0),
                RIS: mmio::Register::new(0),
                MIS: mmio::Register::new(0),
                ICR: mmio::Register::new(0),
                DMACR: mmio::Register::new(0),
            },
        }
    }

    /// init drivers function.
    /// Set gpio pin for UART.
    /// Init uart.
    pub fn init(&mut self, dt: &DeviceTree) {
        //self.gpio.set_alt5_gpio14();
        //self.gpio.set_alt5_gpio15();
        //self.gpio.clear_pu_pd_clk0(14);
        //self.gpio.clear_pu_pd_clk0(15);

        self.uart.init(dt);

        //let mut big_addr: u64 = 8 * 1024 * 1024 * 1024;
        //unsafe { core::ptr::read_volatile(big_addr as *mut u64) };
        //ilog!("##############################\n\n\n\n");
        //self.gic.enable_system_timer();
        //ilog!("#######################\n\n\n\n");
        //self.systimer.set_cmp1(5000000);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_macro::kernel_test;

    #[kernel_test]
    fn test_device_tree_parsing() {
        let dt = DeviceTree::new(BASE_ADDR_DEVICETREE).unwrap();
        let _n = dt.get_node_address("/pl011").unwrap();
    }
}
