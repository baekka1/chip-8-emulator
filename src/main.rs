const HEIGHT: usize = 64;
const WIDTH: usize = 32;

mod emulator;
mod gui;

use emulator::Emulator;
use gui::Display;

fn main() {
    let path = "./roms/IBM Logo.ch8";

    let mut emu = Emulator::new();

    emu.memory.load_rom(path).expect("Failed to load ROM");

    let mut window = Display::new("CHIP-8 emulator", HEIGHT, WIDTH);
}
