use crate::bus::Bus;
use crate::ram::Ram;

pub mod instruction;
pub mod interrupt;

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
/// Unused on 2A03, always 1
pub static FLAG_UNUSED: u8 = 1 << 5;
/// Overflow Flag V
pub static FLAG_OVERFLOW: u8 = 1 << 6;
/// Negative Flag N
pub static FLAG_NEGATIVE: u8 = 1 << 7;

/// NES 2A03 Emulator
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
    pub sp: u8,
    /// Program counter
    pub pc: u16,
    /// Whether CPU is halted (all execution stopped)
    pub halt: bool,
    /// System bus
    pub bus: Bus,
    /// Current cpu cycle number (0 on reset)
    pub cycle: usize,
}
impl Cpu {
    /// Initialise CPU
    pub fn new() -> Self {
        let ram = Ram::new();
        let bus = Bus::new(ram);

        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            p: 0b00100100,
            sp: 0x00FD,
            halt: false,
            bus,
            cycle: 0,
        }
    }

    /// Set processor status flag
    pub fn p_set(&mut self, flag: u8) {
        self.p |= flag;
    }

    /// Unset (zero) processor status flag
    pub fn p_unset(&mut self, flag: u8) {
        self.p &= !flag;
    }

    /// Get processor status flag state
    pub fn p_get(&self, flag: u8) -> bool {
        (self.p & flag) != 0
    }

    /// Push value to NES stack
    pub fn stack_push(&mut self, data: u8) {
        // align with stack
        self.bus.write(self.sp as u16 | (0x01 << 8), data);
        self.sp -= 1;
    }

    /// Pop value from NES stack
    pub fn stack_pop(&mut self) -> u8 {
        // align with stack
        self.sp += 1;
        self.bus.read(self.sp as u16 | (0x01 << 8))
    }
}
