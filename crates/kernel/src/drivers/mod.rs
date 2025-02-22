use device_tree::DeviceTree;

use crate::ilog;

pub mod device_tree;
pub mod gic;
pub mod gpio;
pub mod mbox;
pub mod mmio;
pub mod systimer;
pub mod uart;

pub trait Driver {
    /// Usually set MMIO registers from base address of the device
    fn new(dt: &DeviceTree) -> Self;
    /// Init the device (set bit in registers etc.)
    fn init(&mut self);
    /// de-init the device.
    fn deinit(&mut self);
}

/// Struct that contains drivers for peripherals.
#[derive(Default)]
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
    pub fn new(dt: &DeviceTree) -> Self {
        Drivers {
            //gpio: gpio::GPIO::new(),
            uart: uart::UARTPL011::new(&dt),
            //systimer: systimer::SysTimer::new(),
            // gic: gic::Gic::new(),
        }
    }

    /// init drivers function.
    /// Set gpio pin for UART.
    /// Init uart.
    pub fn init(&mut self) {
        //self.gpio.set_alt5_gpio14();
        //self.gpio.set_alt5_gpio15();
        //self.gpio.clear_pu_pd_clk0(14);
        //self.gpio.clear_pu_pd_clk0(15);

        self.uart.init();

        //let mut big_addr: u64 = 8 * 1024 * 1024 * 1024;
        //unsafe { core::ptr::read_volatile(big_addr as *mut u64) };
        //ilog!("##############################\n\n\n\n");
        //self.gic.enable_system_timer();
        //ilog!("#######################\n\n\n\n");
        //self.systimer.set_cmp1(5000000);
        ilog!("drivers [ok]");
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
