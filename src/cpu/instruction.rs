use super::Cpu;

pub enum AddressingMode {
    ZeroPage,
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    Absolute,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    Indirect,
    Implied,
    Accumulator,
    Immediate,
    Relative,
    /// Uses X register
    IndexedIndirectX,
    /// Uses Y register
    IndirectIndexedY,
}

impl Cpu {
    /// Fetch 8-bit instruction and increment program counter
    pub fn fetch_instruction_8(&mut self) -> u8 {
        let instr = self.bus.read(self.pc);
        self.pc += 1;
        instr
    }

    /// Fetch 16-bit instruction and increment program counter by 2
    pub fn fetch_instruction_16(&mut self) -> u16 {
        let low = self.bus.read(self.pc);
        self.pc += 1;
        let high = self.bus.read(self.pc);
        self.pc += 1;

        u16::from_le_bytes([low, high])
    }

    /// Step through next CPU instruction
    pub fn step(&mut self) {
        self.cycle += 1;
    }
}
