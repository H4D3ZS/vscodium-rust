use crate::devices::bus::{IoDevice, MmioDevice};

pub struct DebugPort;

impl DebugPort {
    pub fn new() -> Self {
        DebugPort
    }
}

impl IoDevice for DebugPort {
    fn read(&mut self, _port: u16) -> u8 {
        0
    }

    fn write(&mut self, _port: u16, value: u8) {
        eprint!("{}", value as char);
    }
}

impl MmioDevice for DebugPort {
    fn read(&mut self, _addr: u64, data: &mut [u8]) {
        if let Some(first) = data.first_mut() {
            *first = 0;
        }
    }

    fn write(&mut self, _addr: u64, data: &[u8]) {
        if let Some(&first) = data.first() {
            eprint!("{}", first as char);
        }
    }
}
