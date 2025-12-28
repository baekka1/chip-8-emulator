use minifb::Key;
use minifb::{Scale, Window, WindowOptions};

pub struct Display {
    pub win: Window,
    pub height: usize,
    pub width: usize,
    pub buffer: Vec<u32>,
}

impl Display {
    pub fn new(title: &str, height: usize, width: usize) -> Self {
        let mut display = Window::new(
            title,
            width,
            height,
            WindowOptions {
                scale: Scale::X8,
                ..WindowOptions::default()
            },
        )
        .expect("failed to create window");

        display.set_target_fps(60);

        let buffer = vec![0u32; width * height];

        Self {
            win: display,
            height: height,
            width: width,
            buffer: buffer,
        }
    }

    pub fn map_key(&self) -> u16 {
        let mut c = 0xFF;
        self.win.get_keys().iter().for_each(|key| match key {
            Key::Key1 => c = 0x1,
            Key::Key2 => c = 0x2,
            Key::Key3 => c = 0x3,
            Key::Key4 => c = 0xC,
            Key::Q => c = 0x4,
            Key::W => c = 0x5,
            Key::E => c = 0x6,
            Key::R => c = 0xD,
            Key::A => c = 0x7,
            Key::S => c = 0x8,
            Key::D => c = 0x9,
            Key::F => c = 0xE,
            Key::Z => c = 0xA,
            Key::X => c = 0x0,
            Key::C => c = 0xB,
            Key::V => c = 0xF,
            _ => (),
        });
        return c;
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }
}
