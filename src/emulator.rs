use std::time::Duration;

use crate::cpu::CPU;
use crate::drivers::{*, self};

pub const START_RAM_ADDRESS: usize = 0x200;
pub const FONT_OFFSET: usize = 0x00;
pub struct Emulator {
    cpu: CPU,
    context: sdl2::Sdl,
    display_driver: display::DisplayDriver,
    keyboard_driver: keyboard::KeyboardDriver,
    sound_driver: sound::SoundDriver,
}

impl Emulator {
    pub fn new() -> Result<Emulator, String> {
        let context = sdl2::init()?;
        let display_driver = display::DisplayDriver::new(&context)?;
        
        Ok(Emulator {
            cpu: CPU::default(),
            display_driver,
            sound_driver: sound::SoundDriver::new(&context),
            context,
            keyboard_driver: keyboard::KeyboardDriver::new(),
        })
    }

    pub fn load_program(&mut self, path: &str) -> Result<(), String> {
        self.cpu.load_ram(path)
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut event_pump = self.context.event_pump()?;
    
         loop {
            ::std::thread::sleep(Duration::from_micros(600));
            self.cpu.timer.update();
            self.cpu.sound_timer.update();
            if !self.cpu.sound_timer.is_zero() {
                self.sound_driver.start();
            } else {
                self.sound_driver.stop();
            }
            if self.keyboard_driver.keys_pressed(&mut event_pump, &mut  self.cpu.key_buffer) == drivers::keyboard::Result::Quit {
                break;
            }
            self.cpu.cycle(&mut self.display_driver)?;
        }
        Ok(())
    }
}