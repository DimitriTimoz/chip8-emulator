
pub struct KeyboardDriver {
    keys: [bool; 16],
}

impl KeyboardDriver {
    pub fn new() -> Self {
        Self {
            keys: [false; 16],
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn set_key(&mut self, key: u8, pressed: bool) {
        self.keys[key as usize] = pressed;
    }

    pub fn clear(&mut self) {
        for i in 0x0..self.keys.len() {
            self.keys[i] = false;
        }
    }
}