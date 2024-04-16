pub struct Ram([u8; 0x800]);
impl Ram {
    pub fn new() -> Self {
        Self([0; 0x800])
    }

    #[inline(always)]
    #[cfg(not(debug_assertions))]
    pub unsafe fn write(&mut self, addr: usize, data: u8) {
        unsafe {
            *self.0.get_unchecked_mut(addr) = data;
        }
    }

    #[inline(always)]
    #[cfg(debug_assertions)]
    pub fn write(&mut self, addr: usize, data: u8) {
        self.0[addr] = data;
    }

    #[inline(always)]
    #[cfg(not(debug_assertions))]
    pub unsafe fn read(&self, addr: usize) -> u8 {
        unsafe { *self.0.get_unchecked(addr) }
    }

    #[inline(always)]
    #[cfg(debug_assertions)]
    pub fn read(&self, addr: usize) -> u8 {
        self.0[addr]
    }
}
