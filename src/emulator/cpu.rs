use crate::{HEIGHT, WIDTH, emulator::Memory};

pub struct Cpu {
    pub stack: [u16; 1024],
    pub gen_registers: [u8; 16],
    pub pc: u16,
    pub i: u16,
    pub vram: Vec<u8>,
    pub draw_dirty: bool,
}

struct Opcode {
    first_nibble: u8,
    x: u8,
    y: u8,
    n: u8,
    nn: u8,
    nnn: u16,
    opcode: u16,
}

impl Cpu {
    pub fn new(height: usize, width: usize) -> Self {
        let vram = vec![0u8; height * width];

        Self {
            stack: [0; 1024],
            gen_registers: [0; 16],
            pc: 0x200,
            i: 0,
            vram: vram,
            draw_dirty: false,
        }
    }

    pub fn cpu_cycle(&mut self, memory: &Memory) {
        let h_opcode = self.fetch(memory);
        let opcode = self.decode(h_opcode);
        self.execute(opcode, memory);
    }

    fn fetch(&mut self, memory: &Memory) -> u16 {
        let first_byte = memory.data[self.pc as usize] as u16;
        let second_byte = memory.data[(self.pc + 1) as usize] as u16;

        let opcode = (first_byte << 8) | second_byte;

        self.pc += 2;

        return opcode;
    }

    fn decode(&mut self, opcode: u16) -> Opcode {
        let first_nibble = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF as u16;

        Opcode {
            first_nibble: first_nibble,
            x: x,
            y: y,
            n: n,
            nn: nn,
            nnn: nnn,
            opcode: opcode,
        }
    }

    fn execute(&mut self, opcode: Opcode, memory: &Memory) {
        // println!("opcode: {:#X}", opcode.opcode);
        match opcode.first_nibble {
            0x0 => match opcode.opcode {
                0x00E0 => {
                    self.vram.fill(0);
                    self.draw_dirty = true;
                }
                _ => {
                    //println!("improper opcode: {:#X}", opcode.opcode);
                }
            },
            0x1 => {
                //the jump instruction
                self.pc = opcode.nnn;
            }
            0x6 => {
                // set register VX
                self.gen_registers[opcode.x as usize] = opcode.nn;
            }
            0x7 => {
                // add value to register VX
                // for this instruction, nothing happens if there is overflow
                self.gen_registers[opcode.x as usize] += opcode.nn;
            }
            0xA => {
                // set index register I
                self.i = opcode.nnn;
            }
            0xD => {
                // draw
                let x_cord = (self.gen_registers[opcode.x as usize] & 63) as usize;
                println!("x coordinate: {:X}", x_cord);
                let mut y_cord = (self.gen_registers[opcode.y as usize] & 31) as usize;

                self.gen_registers[0xF] = 0;

                for n in 0..opcode.n {
                    if y_cord >= HEIGHT {
                        break;
                    }
                    let mut x = x_cord;
                    let sprite: u8 = memory.data[self.i as usize + n as usize];
                    for bit_index in (0..8).rev() {
                        if x == WIDTH {
                            break;
                        }
                        // shifts the bit_index'th bit to the right-most position, then does a mask
                        // with 1
                        let pixel = (sprite >> bit_index) & 1;
                        if pixel == 1 {
                            let pos = y_cord * WIDTH + x;
                            if self.vram[pos] == 1 {
                                self.vram[pos] = 0;
                                self.gen_registers[0xF] = 1;
                            } else {
                                self.vram[pos] = 1;
                            }
                        }
                        x += 1;
                    }
                    y_cord += 1;
                }
            }
            _ => {
                //println!("improper opcode: {:#X}", opcode.opcode);
            }
        }
    }
}
