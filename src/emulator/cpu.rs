use crate::{HEIGHT, WIDTH, emulator::Memory};
use rand::Rng;

pub struct Cpu {
    pub gen_registers: [u8; 16],
    pub pc: u16,
    pub sp: u16,
    pub i: u16,
    pub vram: Vec<u8>,
    pub draw_dirty: bool,
}
// add a stack pointer

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
            gen_registers: [0; 16],
            pc: 0x200,
            sp: 0,
            i: 0,
            vram: vram,
            draw_dirty: false,
        }
    }

    pub fn cpu_cycle(&mut self, memory: &mut Memory) {
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

    fn execute(&mut self, opcode: Opcode, memory: &mut Memory) {
        // println!("opcode: {:#X}", opcode.opcode);
        let vx = self.gen_registers[opcode.x as usize];
        let vy = self.gen_registers[opcode.y as usize];
        match opcode.first_nibble {
            0x0 => match opcode.opcode {
                0x00E0 => {
                    self.vram.fill(0);
                    self.draw_dirty = true;
                }
                0x00EE => {
                    self.pc = memory.stack[(self.sp - 1) as usize];
                    self.sp -= 1;
                }
                _ => {
                    //println!("improper opcode: {:#X}", opcode.opcode);
                }
            },
            0x1 => {
                //the jump instruction
                self.pc = opcode.nnn;
            }
            0x2 => {
                // calls subroutine
                memory.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode.nnn;
            }
            0x3 => {
                if vx == opcode.nn {
                    self.pc += 2;
                }
            }
            0x4 => {
                if vx != opcode.nn {
                    self.pc += 2;
                }
            }
            0x5 => {
                if vx == vy {
                    self.pc += 2;
                }
            }
            0x6 => {
                // set register VX
                self.gen_registers[opcode.x as usize] = opcode.nn;
            }
            0x7 => {
                // add value to register VX
                // for this instruction, nothing happens if there is overflow
                let x = opcode.x as usize;
                self.gen_registers[x] = self.gen_registers[x].wrapping_add(opcode.nn);
            }
            0x8 => match opcode.n {
                0 => {
                    self.gen_registers[opcode.x as usize] = vy;
                }
                1 => {
                    self.gen_registers[opcode.x as usize] = vx | vy;
                }
                2 => {
                    self.gen_registers[opcode.x as usize] = vx & vy;
                }
                3 => {
                    self.gen_registers[opcode.x as usize] = vx ^ vy;
                }
                4 => {
                    let (sum, carry) = vx.overflowing_add(vy);
                    self.gen_registers[opcode.x as usize] = sum;
                    self.gen_registers[0xF] = if carry { 1 } else { 0 };
                }
                5 => {
                    self.gen_registers[0xF] = if vx > vy { 1 } else { 0 };
                    self.gen_registers[opcode.x as usize] = vx.wrapping_sub(vy);
                }
                6 => {
                    // CONFIG LATER
                    self.gen_registers[opcode.y as usize] = vx;
                    self.gen_registers[0xF] = vx & 1;
                    self.gen_registers[opcode.x as usize] = vx >> 1;
                }
                7 => {
                    self.gen_registers[0xF] = if vy > vx { 1 } else { 0 };
                    self.gen_registers[opcode.x as usize] = vy.wrapping_sub(vx);
                }
                0xE => {
                    // CONFIG LATER
                    self.gen_registers[opcode.y as usize] = vx;
                    self.gen_registers[0xF] = vx & 128;
                    self.gen_registers[opcode.x as usize] = vx << 1;
                }
                _ => {
                    println!("not implemented yet");
                }
            },
            0x9 => {
                if vx != vy {
                    self.pc += 2;
                }
            }
            0xA => {
                // set index register I
                self.i = opcode.nnn;
            }
            0xB => {
                // jump with offset
                // CONFIG LATER (if necessary)
                let v0 = self.gen_registers[0x0];
                // double check to make sure that the reg. should only hold 8 bit numbers
                self.pc = opcode.nnn + v0 as u16;
            }
            0xC => {
                let mut rng = rand::rng();
                let x: u8 = rng.random();
                self.gen_registers[opcode.x as usize] = opcode.nn & x;
            }
            0xD => {
                // draw
                let x_cord = (self.gen_registers[opcode.x as usize] & 63) as usize;
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
