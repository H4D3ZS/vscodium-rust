use crate::error::{Error, Result};

pub trait IoDevice: Send {
    fn read(&mut self, port: u16) -> u8;
    fn write(&mut self, port: u16, value: u8);
}

/// Adapter that registers a single shared device (held behind `Arc<Mutex<_>>`)
/// at one or more, possibly non-contiguous, I/O port ranges. Clone the `Arc`
/// and register one adapter per range; all of them dispatch to the same
/// underlying device state (e.g. the 8237 DMA controller is reachable at
/// 0x00-0x0F, 0x80-0x8F and 0xC0-0xDF but is one device).
pub struct SharedIoDevice<D: IoDevice> {
    inner: std::sync::Arc<std::sync::Mutex<D>>,
}

impl<D: IoDevice> SharedIoDevice<D> {
    pub fn new(inner: std::sync::Arc<std::sync::Mutex<D>>) -> Self {
        SharedIoDevice { inner }
    }
}

impl<D: IoDevice> IoDevice for SharedIoDevice<D> {
    fn read(&mut self, port: u16) -> u8 {
        self.inner.lock().map(|mut d| d.read(port)).unwrap_or(0xFF)
    }

    fn write(&mut self, port: u16, value: u8) {
        if let Ok(mut d) = self.inner.lock() {
            d.write(port, value);
        }
    }
}

pub trait MmioDevice: Send {
    fn read(&mut self, addr: u64, data: &mut [u8]);
    fn write(&mut self, addr: u64, data: &[u8]);
}

#[derive(Clone, Copy, Debug)]
pub struct IoRange {
    pub base: u16,
    pub len: u16,
}

impl IoRange {
    pub fn contains(&self, port: u16) -> bool {
        port >= self.base && port < self.base.saturating_add(self.len)
    }

    pub fn overlaps(&self, other: &IoRange) -> bool {
        let end = self.base.saturating_add(self.len);
        let other_end = other.base.saturating_add(other.len);
        self.base < other_end && other.base < end
    }
}

pub struct IoBus {
    devices: Vec<(IoRange, Box<dyn IoDevice>)>,
    /// Optional PCI host bridge consulted for ports not claimed by a fixed
    /// device — this is how dynamically-assigned PCI I/O BARs are reached.
    pci: Option<std::sync::Arc<std::sync::Mutex<crate::devices::pci::PciStub>>>,
}

impl IoBus {
    pub fn new() -> Self {
        IoBus {
            devices: Vec::new(),
            pci: None,
        }
    }

    /// Attach the PCI host bridge as the fallback for unclaimed I/O ports.
    pub fn set_pci(
        &mut self,
        bridge: std::sync::Arc<std::sync::Mutex<crate::devices::pci::PciStub>>,
    ) {
        self.pci = Some(bridge);
    }

    pub fn register(&mut self, range: IoRange, dev: Box<dyn IoDevice>) -> Result<()> {
        for (existing_range, _) in &self.devices {
            if existing_range.overlaps(&range) {
                return Err(Error::DeviceOverlap {
                    base: range.base,
                    len: range.len,
                });
            }
        }
        self.devices.push((range, dev));
        Ok(())
    }

    pub fn read(&mut self, port: u16, data: &mut [u8]) -> Result<()> {
        for (index, byte) in data.iter_mut().enumerate() {
            let current_port = port.saturating_add(index as u16);
            if let Some(dev_index) = self
                .devices
                .iter()
                .position(|(range, _)| range.contains(current_port))
            {
                let device = &mut self.devices[dev_index].1;
                *byte = device.read(current_port);
            } else if let Some(ref pci) = self.pci {
                // Fall back to a PCI I/O BAR, else open bus (0xff).
                *byte = pci
                    .lock()
                    .ok()
                    .and_then(|mut b| b.io_read(current_port))
                    .unwrap_or(0xff);
            } else {
                // Return 0xff for unhandled ports (no device present)
                *byte = 0xff;
            }
        }
        Ok(())
    }

    pub fn write(&mut self, port: u16, data: &[u8]) -> Result<()> {
        for (index, byte) in data.iter().enumerate() {
            let current_port = port.saturating_add(index as u16);
            if let Some(dev_index) = self
                .devices
                .iter()
                .position(|(range, _)| range.contains(current_port))
            {
                let device = &mut self.devices[dev_index].1;
                device.write(current_port, *byte);
            } else if let Some(ref pci) = self.pci {
                // Fall back to a PCI I/O BAR if one decodes this port.
                if let Ok(mut b) = pci.lock() {
                    b.io_write(current_port, *byte);
                }
            }
            // Silently ignore writes to unhandled ports
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MmioRange {
    pub base: u64,
    pub len: u64,
}

impl MmioRange {
    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.base && addr < self.base.saturating_add(self.len)
    }

    pub fn overlaps(&self, other: &MmioRange) -> bool {
        let end = self.base.saturating_add(self.len);
        let other_end = other.base.saturating_add(other.len);
        self.base < other_end && other.base < end
    }
}

pub struct MmioBus {
    devices: Vec<(MmioRange, Box<dyn MmioDevice>)>,
}

impl MmioBus {
    pub fn new() -> Self {
        MmioBus {
            devices: Vec::new(),
        }
    }

    pub fn register(&mut self, range: MmioRange, dev: Box<dyn MmioDevice>) -> Result<()> {
        for (existing_range, _) in &self.devices {
            if existing_range.overlaps(&range) {
                return Err(Error::MmioOverlap {
                    base: range.base,
                    len: range.len,
                });
            }
        }
        self.devices.push((range, dev));
        Ok(())
    }

    pub fn read(&mut self, addr: u64, data: &mut [u8]) -> Result<()> {
        for (index, byte) in data.iter_mut().enumerate() {
            let current_addr = addr.saturating_add(index as u64);
            if let Some(dev_index) = self
                .devices
                .iter()
                .position(|(range, _)| range.contains(current_addr))
            {
                let device = &mut self.devices[dev_index].1;
                let mut tmp = [0u8; 1];
                device.read(current_addr, &mut tmp);
                *byte = tmp[0];
            } else {
                *byte = 0xff;
            }
        }
        Ok(())
    }

    pub fn write(&mut self, addr: u64, data: &[u8]) -> Result<()> {
        for (index, byte) in data.iter().enumerate() {
            let current_addr = addr.saturating_add(index as u64);
            if let Some(dev_index) = self
                .devices
                .iter()
                .position(|(range, _)| range.contains(current_addr))
            {
                let device = &mut self.devices[dev_index].1;
                device.write(current_addr, &[*byte]);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod mmio_tests {
    use super::*;

    struct DummyMmio {
        last: u8,
    }

    impl MmioDevice for DummyMmio {
        fn read(&mut self, _addr: u64, data: &mut [u8]) {
            if let Some(first) = data.first_mut() {
                *first = self.last;
            }
        }

        fn write(&mut self, _addr: u64, data: &[u8]) {
            if let Some(first) = data.first() {
                self.last = *first;
            }
        }
    }

    #[test]
    fn mmio_register_rejects_overlap() {
        let mut bus = MmioBus::new();
        bus.register(
            MmioRange {
                base: 0x1000,
                len: 0x10,
            },
            Box::new(DummyMmio { last: 0 }),
        )
        .unwrap();
        let err = bus
            .register(
                MmioRange {
                    base: 0x1008,
                    len: 0x10,
                },
                Box::new(DummyMmio { last: 0 }),
            )
            .unwrap_err();
        match err {
            Error::MmioOverlap { .. } => {}
            _ => panic!("unexpected error"),
        }
    }

    #[test]
    fn mmio_read_write_dispatches() {
        let mut bus = MmioBus::new();
        bus.register(
            MmioRange {
                base: 0x2000,
                len: 1,
            },
            Box::new(DummyMmio { last: 0 }),
        )
        .unwrap();
        bus.write(0x2000, &[0xaa]).unwrap();
        let mut data = [0u8; 1];
        bus.read(0x2000, &mut data).unwrap();
        assert_eq!(data[0], 0xaa);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyDevice {
        last: u8,
    }

    impl IoDevice for DummyDevice {
        fn read(&mut self, _port: u16) -> u8 {
            self.last
        }

        fn write(&mut self, _port: u16, value: u8) {
            self.last = value;
        }
    }

    #[test]
    fn register_rejects_overlap() {
        let mut bus = IoBus::new();
        bus.register(
            IoRange {
                base: 0x3f8,
                len: 8,
            },
            Box::new(DummyDevice { last: 0 }),
        )
        .unwrap();
        let err = bus
            .register(
                IoRange {
                    base: 0x3fc,
                    len: 8,
                },
                Box::new(DummyDevice { last: 0 }),
            )
            .unwrap_err();
        match err {
            Error::DeviceOverlap { .. } => {}
            _ => panic!("unexpected error"),
        }
    }

    #[test]
    fn read_write_dispatches() {
        let mut bus = IoBus::new();
        bus.register(
            IoRange { base: 0x60, len: 1 },
            Box::new(DummyDevice { last: 0 }),
        )
        .unwrap();

        bus.write(0x60, &[0x5a]).unwrap();
        let mut data = [0u8; 1];
        bus.read(0x60, &mut data).unwrap();
        assert_eq!(data[0], 0x5a);
    }
}
