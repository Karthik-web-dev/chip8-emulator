use crate::core::memory::Memory;

pub struct Display {
    pixels: [u32; 64 * 32]
}

impl Display {
    pub fn new() -> Self {
        Display {
            pixels: std::array::from_fn(|_| 0)
        }
    }
}

pub struct Chip8 {
    v: [u8; 16],
    memory: Memory,
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

    pub fn execute(opcode: u16) {
        match opcode & 0xF000 {
            0x0000 => match opcode {
                0xE000 => {

                }, 

                0x00E0 => {

                },
                _ => {
                    
                }
            },

            0x1000 => {},
            0x2000 => {},
            0x3000 => {},
            0x4000 => {},
            0x5000 => {},
            0x6000 => {},
            0x7000 => {},
            0x8000 => match opcode & 0x000F {
                0x0 => {},
                0x1 => {},
                0x2 => {},
                0x3 => {},
                0x4 => {},
                0x5 => {},
                0x6 =>  {},
                0x7 => {},
                0xE => {},
                _ => unreachable!(),
            }

            0x9000 => {},
            0xA000 => {},
            0xB000 => {},
            0xC000 => {},
            0xD000 => {},

            0xE000 => match opcode & 0x00FF{
                0x91 => {},
                0xA1 => {},
                _ => unreachable!(),
            }

            0xF000 => match opcode & 0x00FF{
                0x7A => {},
                0x0A => {},
                0x15 => {},
                0x18 => {},
                0x1E => {},
                0x29 => {},
                0x33 => {},
                0x55 => {},
                0x65 => {},
                _ => unreachable!(),
            }
            _ => panic!("Incorrect Opcode"),
        }
    }

    pub fn tick(&mut self) {
        //FETCH
        let instr = self.memory.read_u16(self.pc);
        self.pc += 2; //Increment PC
        
        //DECODE
        
    }
}


