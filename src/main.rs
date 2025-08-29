mod components;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

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

struct VariableRegisters {
    v: [u8; 16],
}

impl VariableRegisters {
    fn new() -> Self {
        Self { v: [0; 16] }
    }

    fn set(&mut self, index: usize, value: u8) {
        self.v[index] = value;
    }

    fn add(&mut self, index: usize, value: u8) {
        self.v[index] += value;
    }

    fn get(&self, index: usize) -> u8 {
        self.v[index]
    }
}

struct UserWindow {
    window: Window,
    buffer: Vec<u32>,
}

struct Chip8 {
    pc: u16,
    stack: Vec<u16>,
    s_timer: u8,
    d_timer: u8,
    memory: [u8; 4096],
    var_registers: VariableRegisters,
    index_register: u16,
}

impl UserWindow {
    fn clear_screen(&mut self) {
        for i in self.buffer.iter_mut() {
            *i = 0;
        }
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

impl Chip8 {
    fn new() -> Self {
        Self {
            pc: 0x200,
            stack: Vec::new(),
            s_timer: 0,
            d_timer: 0,
            memory: [0; 4096],
            var_registers: VariableRegisters::new(),
            index_register: 0,
        }
    }

    fn load_fontset_into_memory(&mut self) {
        let font_start: usize = 0x050;

        for (i, byte) in FONTSET.iter().enumerate() {
            self.memory[font_start + i] = *byte;
        }
    }

    fn fetch(&mut self) -> u16 {
        //read instruction that PC is currently pointing at in memory
        // will need to read two successive bytes from mem and combine into one
        // 16 bit instruction
        // increment PC by 2
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8
            | self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        opcode
    }

    fn get_nibbles(opcode: u16) -> (u8, u8, u8, u8, u16) {
        // gets the nibbles, where nibbles are 4 bits
        // REMINDER: X and Y are used to look up registers
        let x: u8 = ((opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((opcode & 0x00F0) >> 4) as u8;
        let n: u8 = (opcode & 0x000F) as u8;
        let nn: u8 = (opcode & 0x00FF) as u8;
        let nnn: u16 = opcode & 0x0FFF;
        (x, y, n, nn, nnn)
    }

    fn jump(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn draw(&mut self, x: usize, y: usize) {
        let x_coord = self.var_registers.get(x) & 63;
        let y_coord = self.var_registers.get(y) & 31;
        self.var_registers.set(15, 0);
    }

    fn decode_and_execute(&mut self, window: &mut UserWindow, opcode: u16) {
        // get the nibbles
        let (x, y, n, nn, nnn) = Self::get_nibbles(opcode);

        // big ass switch statement incoming
        let first_nibble: u8 = ((opcode & 0xF000) >> 12) as u8;
        match first_nibble {
            0x00 => {
                // (00E0) clear screen
                UserWindow::clear_screen(window);
            }
            0x01 => {
                // 1NNN (jump)
                self.jump(nnn);
            }
            0x06 => {
                // 6XNN (set register VX)
                self.var_registers.set(x as usize, nn);
            }
            0x07 => {
                // 7XNN (add value to register VX)
                self.var_registers.add(x as usize, nn);
            }
            0x0A => {
                // ANNN (set index register I)
                self.index_register = nnn;
            }
            0x0D => {
                // DXYN (display/draw)
                self.draw(x as usize, y as usize);
            }
            _ => {
                // do nothing, or print error message
            }
        }
    }
}

fn main() {
    let mut chip8: Chip8 = Chip8::new();

    // Load the fontset into memory
    chip8.load_fontset_into_memory();

    // Get the first opcode
    let opcode: u16 = chip8.fetch();
    println!("First opcode: {opcode}");

    let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    println!("Creating window");
    let window = Window::new(
        "Chip8 - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: minifb::Scale::X8,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            topmost: false,
            transparency: false,
            none: false,
        },
    )
    .unwrap_or_else(|e| {
        panic!("Reached an error: {}", e);
    });

    let mut user_window = UserWindow {
        window: window,
        buffer: buffer,
    };

    // Limit to max ~60 fps update rate
    user_window.window.set_target_fps(60);

    chip8.decode_and_execute(&mut user_window, opcode);

    while user_window.window.is_open() && !user_window.window.is_key_down(Key::Escape) {
        for i in user_window.buffer.iter_mut() {
            *i = 1;
        }

        // unwrap here as we want this code to exit if it fails
        user_window
            .window
            .update_with_buffer(&user_window.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
