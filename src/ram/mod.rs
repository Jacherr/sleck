/// NES memory locations 0x0000 to 0x07FF \
/// 0x0000 - 0x00FF = Zero Page \
/// 0x0100 - 0x01FF = Stack \
/// 0x0200 - 0x07FF = RAM
pub struct Ram([u8; 0x800]);
impl Ram {
    /// Init zeroed RAM
    pub fn new() -> Self {
        Self([0; 0x800])
    }

    /// Write byte `data` to RAM at address `addr` \
    /// `addr` must be 0x0000 - 0x07FF inclusive
    #[inline(always)]
    #[cfg(not(debug_assertions))]
    pub unsafe fn write(&mut self, addr: usize, data: u8) {
        unsafe {
            *self.0.get_unchecked_mut(addr) = data;
        }
    }

    /// Write byte `data` to RAM at address `addr` \
    /// `addr` must be 0x0000 - 0x07FF inclusive
    #[inline(always)]
    #[cfg(debug_assertions)]
    pub fn write(&mut self, addr: usize, data: u8) {
        self.0[addr] = data;
    }

    /// Read byte from RAM at address `addr` \
    /// `addr` must be 0x0000 - 0x07FF inclusive
    #[inline(always)]
    #[cfg(not(debug_assertions))]
    pub unsafe fn read(&self, addr: usize) -> u8 {
        unsafe { *self.0.get_unchecked(addr) }
    }

    /// Read byte from RAM at address `addr` \
    /// `addr` must be 0x0000 - 0x07FF inclusive
    #[inline(always)]
    #[cfg(debug_assertions)]
    pub fn read(&self, addr: usize) -> u8 {
        self.0[addr]
    }
}
