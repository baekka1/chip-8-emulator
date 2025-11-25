const HEIGHT: usize = 64;
const WIDTH: usize = 32;

mod emulator;
mod gui;

use emulator::Emulator;
use gui::Display;

fn main() {
    let mut emu = Emulator::new();
    let mut window = Display::new("CHIP-8 emulator", HEIGHT, WIDTH);
}
