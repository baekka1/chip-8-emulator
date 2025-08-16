mod components;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

const FONTSET: [u8; 80] = [
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

fn load_fontset_into_memory(memory: &mut [u8; 4096]) {
    let font_start: usize = 0x050;

    for (i, byte) in FONTSET.iter().enumerate() {
        memory[font_start + i] = *byte;
    }
}

fn fetch(memory: &[u8], pc: &mut u16) -> u16 {
    //read instruction that PC is currently pointing at in memory
    // will need to read two successive bytes from mem and combine into one
    // 16 bit instruction
    // increment PC by 2
    let opcode: u16 = (memory[*pc as usize] as u16) << 8 | memory[(*pc + 1) as usize] as u16;
    *pc += 2;
    opcode
}

fn main() {
    let mut stack: Vec<u16> = Vec::new(); // the stack

    let mut memory: [u8; 4096] = [0; 4096]; // allocated on the stack
    let mut pc: u16 = 512; // program counter - starts at 512 bc that's where the program will get
    // loaded into memory
    let s_timer: u8 = 0; // sound timer
    let d_timer: u8 = 0; // delay timer

    // Load the fontset into memory
    load_fontset_into_memory(&mut memory);

    // Get the first opcode
    let opcode: u16 = fetch(&memory, &mut pc);
    println!("First opcode: {opcode}");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    println!("Creating window");
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Reached an error: {}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 1;
        }

        // unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
