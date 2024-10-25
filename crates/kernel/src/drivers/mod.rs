use crate::ilog;

pub mod gic;
pub mod gpio;
pub mod mbox;
pub mod mmio;
pub mod systimer;
pub mod uart;

const BASE_ADDR_DEVICETREE: usize = 0x40000000;

pub struct DeviceTree<'a> {
    pub base_address: usize,
    pub fdt: fdt::Fdt<'a>,
}

impl DeviceTree<'_> {
    pub fn new(base_addrese: usize) -> Result<Self, fdt::FdtError> {
        unsafe {
            let fdt = fdt::Fdt::from_ptr(base_addrese as *const u8)?;
            Ok(Self {
                base_address: base_addrese,
                fdt,
            })
        }
    }

    pub fn get_node_address(self, node_name: &str) -> Option<usize> {
        let node = self.fdt.find_node(node_name);
        if let Some(node) = node {
            if let Some(mut regs) = node.reg() {
                if let Some(reg) = regs.next() {
                    return Some(reg.starting_address as usize);
                }
            }
        }
        None
    }
}

mod periph_map {
    /// Peripheral base address.
    pub const PBASE: usize = 0x08000000;
    /// UART0 base address
    pub const UART0: usize = 0x09000000;
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
    pub fn new() -> Self {
        Drivers {
            //gpio: gpio::GPIO::new(),
            uart: uart::UARTPL011::new(),
            // systimer: systimer::SysTimer::new(),
            // gic: gic::Gic::new(),
        }
    }

    /// init drivers function.
    /// Set gpio pin for UART.
    /// Init uart.
    pub fn init(&mut self) {
        ilog!("init drivers");

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
        //ilog!("drivers [ok]");
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
