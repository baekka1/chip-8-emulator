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

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }
}
