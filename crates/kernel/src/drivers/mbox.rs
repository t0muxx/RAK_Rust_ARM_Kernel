use crate::drivers::mmio;
use crate::drivers::periph_map;

#[repr(C, align(16))]
pub struct MailboxMsg {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
    g: u32,
    h: u32,
    i: u32,
}

impl MailboxMsg {
    pub fn new() -> Self {
        MailboxMsg {
            a: 9 * 4,
            b: 0,
            c: 0x38002,
            d: 12,
            e: 8,
            f: 2,
            g: 3000000,
            h: 0,
            i: 0,
        }
    }
}
pub const PBASE_MAILBOX: usize = periph_map::PBASE + 0xB880;

#[allow(non_snake_case)]
pub struct Mailbox {
    MBOX_READ: mmio::Register<u32>,
    MBOX_STATUS: mmio::Register<u32>,
    MBOX_WRITE: mmio::Register<u32>,
}

impl Mailbox {
    pub fn new() -> Self {
        Mailbox {
            MBOX_READ: mmio::Register::new(PBASE_MAILBOX),
            MBOX_STATUS: mmio::Register::new(PBASE_MAILBOX + 0x18),
            MBOX_WRITE: mmio::Register::new(PBASE_MAILBOX + 0x20),
        }
    }

    pub fn send_msg(&self) {
        let msg = MailboxMsg::new();
        let pmsg = &msg as *const _ as usize;
        let r = pmsg & !0xF | 8;
        loop {
            let val = self.MBOX_STATUS.read();
            if val & 0x80000000 == 0 {
                break;
            }
        }
        self.MBOX_WRITE.write(r as u32);
        loop {
            let val = self.MBOX_STATUS.read();
            let val2 = self.MBOX_READ.read();
            if val & 0x40000000 == 0 || val2 as usize != r {
                break;
            }
        }
    }
}
