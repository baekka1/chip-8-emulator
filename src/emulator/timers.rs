pub struct Timers {
    pub delay: u8,
    pub sound: u8,
}

impl Timers {
    pub fn new() -> Self {
        Self { delay: 0, sound: 0 }
    }
}
