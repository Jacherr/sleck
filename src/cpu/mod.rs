/// Carry Flag C
pub static FLAG_CARRY: u8 = 1 << 0;
/// Zero Flag Z
pub static FLAG_ZERO: u8 = 1 << 1;
/// Interrupt Disable Flag I
pub static FLAG_INTERRUPT_DISABLE: u8 = 1 << 2;
/// Decimal Mode Flag D \
/// Unused on 2A03
pub static FLAG_DECIMAL_MODE: u8 = 1 << 3;
/// Break Command Flag B
pub static FLAG_BREAK_COMMAND: u8 = 1 << 4;
/// Unused Flag \
/// Unused on 2A03
pub static FLAG_UNUSED: u8 = 1 << 5;
/// Overflow Flag V
pub static FLAG_OVERFLOW: u8 = 1 << 6;
/// Negative Flag N
pub static FLAG_NEGATIVE: u8 = 1 << 7;

pub struct Cpu {
    /// Accumulator
    pub a: u8,
    /// X general-purpose register
    pub x: u8,
    /// Y general-purpose register
    pub y: u8,
    /// Processor status flag bits
    pub p: u8,
    /// Stack pointer
    pub s: u8,
    /// Program counter
    pub pc: u16,
    /// Whether CPU is halted (all execution stopped)
    pub halt: bool,
}
impl Cpu {
    pub fn p_set(&mut self, flag: u8) {
        self.p |= flag;
    }

    pub fn p_unset(&mut self, flag: u8) {
        self.p &= !flag;
    }

    pub fn p_get(&self, flag: u8) -> bool {
        (self.p & flag) != 0
    }
}
