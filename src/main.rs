const HEIGHT: usize = 32;
const WIDTH: usize = 64;

mod emulator;
mod gui;

use emulator::Emulator;
use gui::Display;

use std::thread;
use std::time::Duration;

fn vram_to_buf(display: &mut Display, emu: &mut Emulator) {
    for i in 0..(HEIGHT * WIDTH) {
        display.buffer[i] = if emu.cpu.vram[i] != 0 {
            0x00FFFFFF
        } else {
            0x00000000
        };
    }
}

fn main() {
    let path = "./roms/Space Invaders [David Winter].ch8";
    //let sleep_duration = Duration::from_millis(1);
    let mut emu = Emulator::new(HEIGHT, WIDTH);

    emu.memory.load_rom(path).expect("Failed to load ROM");

    let mut display = Display::new("CHIP-8 emulator", HEIGHT, WIDTH);

    loop {
        emu.cpu
            .cpu_cycle(&mut emu.memory, &display, &mut emu.timers);
        vram_to_buf(&mut display, &mut emu);
        display
            .win
            .update_with_buffer(&display.buffer, WIDTH, HEIGHT)
            .unwrap();
        if emu.timers.sound > 0 {
            emu.timers.sound -= 1;
        }
        if emu.timers.delay > 0 {
            emu.timers.delay -= 1;
        }
        //thread::sleep(sleep_duration);
    }
}
