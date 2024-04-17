use crate::ram::Ram;

/// NES CPU Bus
pub struct Bus {
    pub ram: Ram,
}
impl Bus {
    pub fn new(ram: Ram) -> Self {
        Self { ram }
    }

    /// Write to bus
    pub fn write(&mut self, addr: u16, data: u8) {
        if addr < 0x2000 {
            self.ram.write((addr & 0x07FF) as usize, data)
        }

        // todo: add more devices to bus
    }
}
