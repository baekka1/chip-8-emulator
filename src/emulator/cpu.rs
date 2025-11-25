use crate::emulator::Memory;

pub struct Cpu {
    pub stack: [u16; 1024],
    pub gen_registers: [u8; 16],
    pub pc: u16,
    pub i: u16,
}

struct Opcode {
    first_nibble: u8,
    x: u8,
    y: u8,
    n: u8,
    nn: u8,
    nnn: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            stack: [0; 1024],
            gen_registers: [0; 16],
            pc: 0,
            i: 0,
        }
    }

    pub fn fetch(&mut self, memory: &Memory) -> u16 {
        let first_byte = memory.data[self.pc as usize] as u16;
        let second_byte = memory.data[(self.pc + 1) as usize] as u16;

        let opcode = (first_byte << 8) | second_byte;

        self.pc += 2;

        return opcode;
    }

    pub fn decode(&mut self, opcode: u16) {
        let first_nibble = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF as u16;

        let opcode = Opcode {
            first_nibble: first_nibble,
            x: x,
            y: y,
            n: n,
            nn: nn,
            nnn: nnn,
        };
    }
}
