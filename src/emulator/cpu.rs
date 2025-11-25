pub struct Cpu {
    pub stack: [u16; 1024],
    pub gen_registers: [u8; 16],
    pub pc: u16,
    pub i: u16,
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
}
