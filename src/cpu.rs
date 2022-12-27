use crate::emulator::{START_RAM_ADDRESS, Instruction};



struct Timer {
    counter: u8,
}

pub(crate) struct CPU {
    pub pc: u16,
    pub registers: [u8; 16],
    pub I: u16,
    timer: Timer,
    sound_timer: Timer,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: START_RAM_ADDRESS as u16,
            registers: [0; 16],
            timer: Timer { counter: 0 },
            sound_timer: Timer { counter: 0 },
            I: 0,
        }
    }
}

pub trait ALU {
    fn execute(&mut self, instruction: Instruction) -> Result<(), String>;
}

impl ALU for CPU {
    fn execute(&mut self, instruction: Instruction) -> Result<(), String> {
        match instruction {
            
            _ => println!("Instruction not implemented for the ALU: {:?}", instruction)
        }

        Ok(())
    }
}