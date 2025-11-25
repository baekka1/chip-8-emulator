use minifb::{Key, Window, WindowOptions};

pub struct Display {
    pub win: Window,
    pub height: usize,
    pub width: usize,
}

impl Display {
    pub fn new(title: &str, height: usize, width: usize) -> Self {
        let display = Window::new(title, height, width, WindowOptions::default())
            .expect("failed to create window");

        Self {
            win: display,
            height: height,
            width: width,
        }
    }
}
