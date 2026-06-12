use crate::core::memory::Memory;
use rand::prelude::*;

pub struct Display {
    pub pixels: [u8; 64 * 32]
}

const SCREEN_WIDTH: u16 = 64;

impl Display {
    pub fn new() -> Self {
        Display {
            pixels: std::array::from_fn(|_| 0)
        }
    }
}

pub struct Chip8 {
    v: [u8; 16],
    pub memory: Memory,
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u16,
    pub dt: u8,
    pub st: u8,
    pub keypad: [u8; 16],
    pub display: Display,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            v: std::array::from_fn(|_| 0),
            memory: Memory::new(),
            index: 0,
            pc: 0x200, //default
            stack: std::array::from_fn(|_| 0),
            sp: 0,
            dt: 0,
            st: 0,
            keypad: std::array::from_fn(|_| 0), //TODO: Implement keys along with platform part.
            display: Display::new(),
        }
    }

    pub fn execute(&mut self, opcode: u16) {
        let nnn: u16 = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        
        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00EE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }, 

                0x00E0 => { //clear screen
                    self.display.pixels = std::array::from_fn(|_| 0);
                },
                _ => {panic!("Unknown opcode: {:#06X}", opcode);}
            },

            0x1000 => { //Jump: Do not increment PC 
                self.pc = nnn;
            },
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            },
            0x3000 => { //Skip instr (conditional)
                if self.v[x] == nn {
                    self.pc += 2;
                }
            },
            0x4000 => { //Skip instr if not equal to nn
                if self.v[x] != nn {
                    self.pc += 2;
                }
            },
            0x5000 => { //Skip instr if vx = vy
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            },
            0x6000 => { //Set
                self.v[x] = nn;
            },
            0x7000 => { //Add
                self.v[x] = self.v[x].wrapping_add(nn);
            },
            0x8000 => match opcode & 0x000F {
                0x0 => { //Set
                    self.v[x] = self.v[y];
                },
                0x1 => { //Binary OR
                    self.v[x] |= self.v[y];
                },
                0x2 => { //Binary AND
                    self.v[x] &= self.v[y];
                },
                0x3 => { //Binary XOR or Logical XOR
                    self.v[x] ^= self.v[y];
                },
                0x4 => { //Add with carry
                    let (result, carry) = self.v[x].overflowing_add(self.v[y]); 
                    self.v[x] = result;
                    self.v[0xF] = carry as u8;
                },
                0x5 => { //Sub with carry vx - vy
                    let (result, borrow) = self.v[x].overflowing_sub(self.v[y]);
                    self.v[x] = result;
                    self.v[0xF] = (!borrow) as u8;
                },
                /*0x6 =>  {
                    // WARNING: Ambigious Instruction!  
                    // TODO::implement a config system later.
                    self.v[x] = self.v[y];
                    let bit = self.v[x] & 1;
                    self.v[x] >>= 1;
                    self.v[0xF] = bit;
                }, */
                0x6 => {
                     let lsb = self.v[x] & 1;
                    self.v[x] >>= 1;
                    self.v[0xF] = lsb;
                }
                0x7 => { //Sub with carry vy - vx
                    let (result, borrow) = self.v[y].overflowing_sub(self.v[x]);
                    self.v[x] = result;
                    self.v[0xF] = (!borrow) as u8;
                },
                /*0xE => {
                    //WARNING: Ambigious Instruction!
                    self.v[x] = self.v[y];
                    let bit = self.v[x] & 1;
                    self.v[x] >>= 1;
                    self.v[0xF] = bit;
                },*/
                0xE => {
                   let msb = (self.v[x] >> 7) & 1;
                    self.v[x] <<= 1;
                    self.v[0xF] = msb;
                    }
                _ => panic!("Unknown opcode: {:#06X}", opcode),
            }

            0x9000 => { //skip instr if vx != vy
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            },
            0xA000 => {
                self.index = nnn;
            },
            0xB000 => {
                //WARNING: Ambigious Instruction
                self.pc = nnn + self.v[0x0] as u16;
            },
            0xC000 => {
                self.v[x] = rand::rng().random::<u8>() & nn;

            },
            0xD000 => { //Dxyn -> Horror
                let x = self.v[x] as u16;
                let y = self.v[y] as u16;
                self.v[0xF] = 0;

                for row_num in 0..n {
                    let addr = self.index + row_num as u16;
                    let pixels = self.memory.read_u8(addr);

                    for bit in 0..8 {
                        if (pixels & (0x80 >> bit)) != 0 {  
                            let x_cord = (x + bit as u16) % 64;
                            let y_cord = (y + row_num as u16) % 32;
                            let idx = x_cord + y_cord * SCREEN_WIDTH; 
                            if self.display.pixels[idx as usize] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.display.pixels[idx as usize] ^= 1;
                        } 
                    }
                }
            },

            0xE000 => match opcode & 0x00FF{
                0x9E => {
                    if self.keypad[self.v[x] as usize] == 1 {
                        self.pc += 2;
                    }
                },
                0xA1 => {
                    if self.keypad[self.v[x] as usize] == 0 {
                        self.pc += 2;
                    }
                },
                _ => panic!("Unknown opcode: {:#06X}", opcode),
            }

            0xF000 => match opcode & 0x00FF{
                0x07 => {
                    self.v[x] = self.dt;
                },
                0x15 => {
                    self.dt = self.v[x];
                },
                0x18 => {
                    self.st = self.v[x];
                },
                0x1E => {
                    //According to amiga chip8 spec sheet for SpaceFlight 2091!
                    let (result, carry) = self.index.overflowing_add(self.v[x] as u16);
                    self.index = result;
                    self.v[0xF] = carry as u8;
                },
                0x0A => {
                    let mut found = false;

                    for i in 0..16 {
                        if self.keypad[i as usize] != 0 {
                            self.v[x] = i as u8;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        self.pc -= 2;
                    }
                },
                0x29 => {
                    // self.index = self.memory.read_u16(self.v[x] as u16)
                    self.index = (self.v[x] as u16) * 5;
                },
                0x33 => {
                    let v: u8 = self.v[x];
                    self.memory.write_u8(self.index,     v / 100);
                    self.memory.write_u8(self.index + 1, (v / 10) % 10);
                    self.memory.write_u8(self.index + 2, v % 10); 
                },
                0x55 => {
                    //WARNING: Ambigious Instruction
                    for i in 0..=x {
                        self.memory.write_u8(self.index + i as u16, self.v[i]);
                    }
                },
                0x65 => {
                    for i in 0..=x {
                        self.v[i] = self.memory.read_u8(self.index + i as u16);
                    }
                },
                _ => panic!("Unknown opcode: {:#06X}", opcode),
            }
            _ => panic!("Incorrect Opcode"),
        }
    }

    pub fn tick(&mut self) {
        //FETCH
        let instr = self.memory.read_u16(self.pc);
        self.pc += 2; //Increment PC
        
        //DECODE
        self.execute(instr);
        
    }
}


