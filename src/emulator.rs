use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::cpu::{CPU};
use crate::drivers::*;

pub const START_RAM_ADDRESS: usize = 0x200;
pub const FONT_OFFSET: usize = 0x50;
pub struct Emulator {
    cpu: CPU,
    context: sdl2::Sdl,
    display_driver: display::DisplayDriver,
    keyboard_driver: keyboard::KeyboardDriver,
}

impl Emulator {
    pub fn new() -> Result<Emulator, String> {
        let context = sdl2::init()?;
        let mut display_driver = display::DisplayDriver::new(&context)?;

        display_driver.init()?;
        let mut ram = [0; 4096];
        let FONTSET: [u8; 80] = [
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
                0xF0, 0x80, 0xF0, 0x80, 0x80  // F
                ];

        ram[FONT_OFFSET..(FONT_OFFSET+FONTSET.len())].copy_from_slice(&FONTSET);

        Ok(Emulator {
            cpu: CPU::new(),
            display_driver,
            context,
            keyboard_driver: keyboard::KeyboardDriver::new(),
        })
    }

    pub fn load_program(&mut self, path: &str) -> Result<(), String> {
        self.cpu.load_ram(path)
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut event_pump = self.context.event_pump()?;
    
        'running: loop {
            ::std::thread::sleep(Duration::from_millis(10));
          
            self.cpu.cycle(&mut self.display_driver)?;
            self.keyboard_driver.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
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
                            self.keyboard_driver.set_key(code, true);
                        }
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }
}