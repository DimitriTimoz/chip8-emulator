use sdl2::{EventPump, keyboard::Keycode, event::Event};


pub struct KeyboardDriver {
}

#[derive(Debug, PartialEq)]
pub enum Result {
    Continue,
    Quit
}

impl KeyboardDriver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn keys_pressed(&self, event_pump: &mut EventPump, keys_buffer: &mut [bool; 16]) -> Result {
        self.clear_buffer(keys_buffer);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return  Result::Quit,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let code: Option<u8> = match keycode {
                        Keycode::Num1 => Some(0x1),
                        Keycode::Num2 => Some(0x2),
                        Keycode::Num3 => Some(0x3),
                        Keycode::Num4 => Some(0xc),
                        Keycode::A => Some(0x4),
                        Keycode::Z => Some(0x5),
                        Keycode::E => Some(0x6),
                        Keycode::R => Some(0xD),
                        Keycode::Q => Some(0x7),
                        Keycode::S => Some(0x8),
                        Keycode::D => Some(0x9),
                        Keycode::F => Some(0xD),
                        Keycode::W => Some(0xA),
                        Keycode::X => Some(0x0),
                        Keycode::C => Some(0xB),
                        Keycode::V => Some(0xF),
                        _ => None
                    };
                    if let Some(code) = code {
                        keys_buffer[code as usize] = true;
                    }
                },
                _ => {}
            }
        }
        Result::Continue
    }

    pub fn clear_buffer(&self, keys_buffer: &mut [bool; 16]) {
        for i in 0..keys_buffer.len() {
            keys_buffer[i] = false;
        }
    }
}