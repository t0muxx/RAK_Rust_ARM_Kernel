pub const BASE_ADDR_DEVICETREE: usize = 0x40000000;
pub struct DeviceTree<'a> {
    pub base_address: usize,
    pub fdt: fdt::Fdt<'a>,
}

impl DeviceTree<'_> {
    pub fn new(base_address: usize) -> Self {
        unsafe {
            match fdt::Fdt::from_ptr(base_address as *const u8) {
                Ok(fdt) => Self {
                    base_address: base_address,
                    fdt,
                },
                Err(_) => panic!("Error reading device tree"),
            }
        }
    }

    pub fn get_node_address(&self, node_name: &str) -> Option<usize> {
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
