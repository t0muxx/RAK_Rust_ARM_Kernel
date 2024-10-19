use crate::drivers::mmio;
use crate::drivers::periph_map;
use crate::set_bit;
//pub const PBASE_SYSTIMER: usize = periph_map::PBASE + 0x3000;
//
//pub struct SysTimer {
//    /// System Timer Control/Status
//    CS: mmio::Register<u32>,
//    /// System Timer Counter Lower 32 bits
//    CLO: mmio::Register<u32>,
//    /// System Timer Counter Higher 32 bits
//    CHI: mmio::Register<u32>,
//    C0: mmio::Register<u32>,
//    C1: mmio::Register<u32>,
//    C2: mmio::Register<u32>,
//    C3: mmio::Register<u32>,
//}
//
//impl SysTimer {
//    pub fn new() -> Self {
//        SysTimer {
//            CS: mmio::Register::new(PBASE_SYSTIMER),
//            CLO: mmio::Register::new(PBASE_SYSTIMER + 0x4),
//            CHI: mmio::Register::new(PBASE_SYSTIMER + 0x8),
//            C0: mmio::Register::new(PBASE_SYSTIMER + 0xc),
//            C1: mmio::Register::new(PBASE_SYSTIMER + 0x10),
//            C2: mmio::Register::new(PBASE_SYSTIMER + 0x14),
//            C3: mmio::Register::new(PBASE_SYSTIMER + 0x18),
//        }
//    }
//
//    /// set compare registers 1
//    pub fn set_cmp1(&self, interval: u32) {
//        let mut val: u32 = 0;
//        val = self.CLO.read();
//        self.C3.write(val + interval);
//        val = 0;
//        set_bit!(val, 3);
//        //self.CS.write(val);
//    }
//
//    /// Get value of systimer.
//    pub fn get(&self) -> u64 {
//        let mut val: u64 = 0;
//        val = (self.CHI.read() as u64) << 32 as u64;
//        val += self.CLO.read() as u64;
//        val
//    }
//
//    /// Get value as seconds.
//    /// Timer operate as 1MHz.
//    /// So seconds = val / 1000000.
//    pub fn get_second(&self) -> f64 {
//        let mut val: f64 = 0.0;
//        val = self.get() as f64;
//        (val / 1000000_f64) as f64
//    }
//}
