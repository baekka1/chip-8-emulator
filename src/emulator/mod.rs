pub mod cpu;
pub mod memory;
pub mod timers;

use cpu::Cpu;
use memory::Memory;
use timers::Timers;

pub struct Emulator {
    pub memory: Memory,
    pub cpu: Cpu,
    pub timers: Timers,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            cpu: Cpu::new(),
            timers: Timers::new(),
        }
    }
}
