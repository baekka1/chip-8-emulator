use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};

pub struct Memory {
    pub data: [u8; 4096],
    pub stack: [u16; 16],
}

impl Memory {
    pub fn new() -> Self {
        let mut ram = Self {
            data: [0; 4096],
            stack: [0; 16],
        };
        ram.load_font();
        ram
    }

    pub fn load_rom(&mut self, path: &str) -> Result<()> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();

        // read file into buffer
        file.read_to_end(&mut buffer)?;
        let start = 0x200;
        let end = start + buffer.len();

        if end > self.data.len() {
            return Err(Error::new(ErrorKind::InvalidData, "ROM too large"));
        }

        self.data[start..end].copy_from_slice(&buffer);

        Ok(())
    }

    fn load_font(&mut self) {
        let font = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        self.data[0x050..0x0A0].copy_from_slice(&font);
    }
}
