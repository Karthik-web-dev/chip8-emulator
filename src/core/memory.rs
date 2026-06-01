use std::fs;

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

    pub fn load_rom(&mut self, filename: &str) {
        let rom_bytes = fs::read(filename).expect("failed to read ROM");
        
        let start = 0x200;
        for (i, byte) in rom_bytes.iter().enumerate() {
            self.ram[start + i] = *byte;
        }

    }
}
