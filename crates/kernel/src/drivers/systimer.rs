use crate::drivers::mmio;
use crate::drivers::periph_map;
pub const PBASE_SYSTIMER: usize = periph_map::PBASE + 0x3000;

pub struct SysTimer {
    /// System Timer Control/Status
    CS: mmio::Register<u32>,
    /// System Timer Counter Lower 32 bits
    CLO: mmio::Register<u32>,
    /// System Timer Counter Higher 32 bits
    CHI: mmio::Register<u32>,
}

impl SysTimer {
    pub fn new() -> Self {
        SysTimer {
            CS: mmio::Register::new(PBASE_SYSTIMER),
            CLO: mmio::Register::new(PBASE_SYSTIMER + 0x4),
            CHI: mmio::Register::new(PBASE_SYSTIMER + 0x8),
        }
    }
    /// Get value of systimer.
    pub fn get(&self) -> u64 {
        let mut val: u64 = 0;
        val = (self.CHI.read() as u64) << 32 as u64;
        val += self.CLO.read() as u64;
        val
    }

    /// Get value as seconds.
    /// Timer operate as 1MHz.
    /// So seconds = val / 1000000.
    pub fn get_second(&self) -> f64 {
        let mut val: f64 = 0.0;
        val = self.get() as f64;
        (val / 1000000_f64) as f64
    }
}
