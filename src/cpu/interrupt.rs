use super::{Cpu, FLAG_INTERRUPT_DISABLE};

/// 2A03 Interrupts
pub enum InterruptType {
    /// Interrupt Request (BRK)
    IRQ,
    /// Non-Maskable Interrupt (V-BLANK)
    NMI,
    /// System reset
    RESET,
}

impl Cpu {
    /// Handle 6502 interrupt. This function should be called when CPU detects interrupt has
    /// happened.
    pub fn interrupt(&mut self, interrupt: InterruptType) {
        match interrupt {
            InterruptType::IRQ => {
                if self.p_get(FLAG_INTERRUPT_DISABLE) {
                    return;
                }

                // save context to stack
                self.stack_push(self.p);
                let [low, high] = u16::to_le_bytes(self.pc);
                self.stack_push(low);
                self.stack_push(high);

                // load interrupt service routine from [0xfffe, 0xffff]
                // todo: handle return from interrupt (RTI) instruction to restore former state
                let low = self.bus.read(0xfffe);
                let high = self.bus.read(0xffff);
                self.pc = u16::from_le_bytes([low, high]);

                return;
            },

            InterruptType::NMI => {
                // save context to stack
                self.stack_push(self.p);
                let [low, high] = u16::to_le_bytes(self.pc);
                self.stack_push(low);
                self.stack_push(high);

                // load interrupt service routine from [0xfffa, 0xfffb]
                // todo: handle return from interrupt (RTI) instruction to restore former state
                let low = self.bus.read(0xfffa);
                let high = self.bus.read(0xfffb);
                self.pc = u16::from_le_bytes([low, high]);

                return;
            },

            InterruptType::RESET => {
                // Reserved flag 1 << 5 always 1, also disable interrupts on reset/boot
                self.p = 0b00100100;
                let low = self.bus.read(0xfffc);
                let high = self.bus.read(0xfffd);
                self.pc = u16::from_le_bytes([low, high]);
            },
            _ => {},
        }
    }
}
