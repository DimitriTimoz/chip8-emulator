use std::time::{ Duration};

use sdl2::{keyboard::Keycode, event::Event};

pub mod cpu;
pub mod drivers;
pub mod emulator;

fn main() -> Result<(), String> {
    let context = sdl2::init()?;
    let mut event_pump = context.event_pump()?;

    let mut emulator = emulator::Emulator::new(&context)?;

    emulator.load_program("ibm_logo.ch8")?;

    'running: loop {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
      

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        
    }
    Ok(())
}



/*
    ALU:
    RAM: 4 KB 4096 bytes
    Display: 64 x 32 Monochrome
    Program Counter: PC
    16-bit Address Register: I
    Stack for 16-bit addresses
    8-bit timer 60 Hz
    8-bit sound timer 
    16 8-bit registers: V0 - VF
        VF is used as a flag
*/