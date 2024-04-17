use crate::ram::Ram;

/// NES CPU Bus
pub struct Bus {
    /// NES RAM, consisting of Zero Page, Stack, and RAM \
    /// Locations 0x0000 - 0x07FF
    pub ram: Ram,
}
impl Bus {
    pub fn new(ram: Ram) -> Self {
        Self { ram }
    }

    /// Write byte `data` to bus at address `addr`
    pub fn write(&mut self, addr: u16, data: u8) {
        if addr < 0x2000 {
            self.ram.write((addr & 0x07FF) as usize, data)
        }

        // todo: add more devices to bus
    }

    /// Read byte from bus at address `addr`
    pub fn read(&self, addr: u16) -> u8 {
        if addr < 0x2000 {
            self.ram.read((addr & 0x07FF) as usize)
        } else {
            // todo: add more devices to bus
            0
        }
    }
}
