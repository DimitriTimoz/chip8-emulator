use sdl2::event::Event;

use crate::cpu::CPU;
use crate::drivers::*;
pub struct Emulator {
    cpu: CPU,
    display_driver: display::DisplayDriver,
}

impl Emulator {
    pub fn new(context: &sdl2::Sdl) -> Result<Emulator, String> {
        let mut display_driver = display::DisplayDriver::new(context)?;

        display_driver.init()?;
        Ok(Emulator {
            // ...
            cpu: CPU::new(),
            display_driver,
            // ...
        })
    }

}