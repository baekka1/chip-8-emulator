const HEIGHT: usize = 640;
const WIDTH: usize = 320;

mod emulator;
mod gui;

use emulator::Emulator;
use gui::Display;
use minifb::Key;

fn main() {
    let path = "./roms/IBM Logo.ch8";

    let mut emu = Emulator::new(HEIGHT, WIDTH);

    emu.memory.load_rom(path).expect("Failed to load ROM");

    let mut display = Display::new("CHIP-8 emulator", HEIGHT, WIDTH);

    while display.win.is_open() && !display.win.is_key_down(Key::Escape) {
        display
            .win
            .update_with_buffer(&display.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
