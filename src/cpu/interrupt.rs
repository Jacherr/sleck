use super::Cpu;

/// 2A03 Interrupts
#[repr(C)]
pub enum InterruptType {
    NMI,
    RESET,
    IRQ,
}

impl Cpu {
    pub fn interrupt(&mut self, interrupt: InterruptType) {
        match interrupt {
            InterruptType::RESET => {
                // Reserved flag 1 << 5 always 1, also disable interrupts on reset/boot
                self.p = 0b00100100;
                let low = self.bus.read(0xfffc);
                let high = self.bus.read(0xfffd);
                self.pc = u16::from_le_bytes([low, high]);
            },
            // todo: handle other interrupts
            _ => {},
        }
    }
}
