pub mod cpu;
pub mod drivers;
pub mod emulator;

fn main() -> Result<(), String> {
    let mut emulator = emulator::Emulator::new()?;

    emulator.load_program("rom/test-audio.ch8")?;

    emulator.run()?;
    Ok(())
}



/*
    ALU:
    ram: 4 KB 4096 bytes
    Display: 64 x 32 Monochrome
    Program Counter: PC
    16-bit Address Register: I
    Stack for 16-bit addresses
    8-bit timer 60 Hz
    8-bit sound timer 
    16 8-bit registers: V0 - VF
        VF is used as a flag
*/