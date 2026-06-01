pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Memory { ram: std::array::from_fn(|_| 0) }
    }

    #[inline]
    pub fn read_u8(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    #[inline]
    pub fn write_u8(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }

    #[inline]
    pub fn read_u16(&self, addr: u16) -> u16 {
        ((self.ram[addr as usize] as u16) << 8) | (self.ram[(addr + 1) as usize] as u16)
    }
}
