//use crate::drivers::mmio;
//use crate::drivers::periph_map;
//use crate::set_bit;
//pub const GIC_PBASE: usize = periph_map::PBASE + 0xB000;
//
//#[allow(non_snake_case, dead_code)]
//pub struct Gic {
//    IRQBasicPending: mmio::Register<u32>,
//    IRQPending1: mmio::Register<u32>,
//    IRQPending2: mmio::Register<u32>,
//    FIQCTRL: mmio::Register<u32>,
//    IRQEnable1: mmio::Register<u32>,
//    IRQEnable2: mmio::Register<u32>,
//    IRQBasicEnable: mmio::Register<u32>,
//    IRQDisable1: mmio::Register<u32>,
//    IRQDisable2: mmio::Register<u32>,
//    IRQBasicDisable: mmio::Register<u32>,
//}
//
//impl Gic {
//    pub fn new() -> Self {
//        Gic {
//            IRQBasicPending: mmio::Register::new(GIC_PBASE + 0x200),
//            IRQPending1: mmio::Register::new(GIC_PBASE + 0x204),
//            IRQPending2: mmio::Register::new(GIC_PBASE + 0x208),
//            FIQCTRL: mmio::Register::new(GIC_PBASE + 0x20C),
//            IRQEnable1: mmio::Register::new(GIC_PBASE + 0x210),
//            IRQEnable2: mmio::Register::new(GIC_PBASE + 0x214),
//            IRQBasicEnable: mmio::Register::new(GIC_PBASE + 0x218),
//            IRQDisable1: mmio::Register::new(GIC_PBASE + 0x21C),
//            IRQDisable2: mmio::Register::new(GIC_PBASE + 0x220),
//            IRQBasicDisable: mmio::Register::new(GIC_PBASE + 0x224),
//        }
//    }
//
//    pub fn enable_system_timer(&self) {
//        let mut val;
//        val = self.IRQEnable1.read();
//        set_bit!(val, 3);
//        self.IRQEnable1.write(val);
//    }
//}
//
