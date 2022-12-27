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
    fn execute(&mut self, instruction: &Instruction) -> Result<(), String>;
}

impl ALU for CPU {
    fn execute(&mut self, instruction: &Instruction) -> Result<(), String> {
        match instruction {
            Instruction::SetToValue(vx, vy) => self.registers[*vx as usize] = self.registers[*vy as usize],
            Instruction::Or(vx, vy) => self.registers[*vx as usize] |= self.registers[*vy as usize],
            Instruction::And(vx, vy) => self.registers[*vx as usize] &= self.registers[*vy as usize],
            Instruction::Xor(vx, vy) => self.registers[*vx as usize] ^= self.registers[*vy as usize],
            Instruction::Add(vx, vy) => {
                let (result, overflow) = self.registers[*vx as usize].overflowing_add(self.registers[*vy as usize]);
                self.registers[*vx as usize] = result;
                self.registers[0xF] = if overflow { 1 } else { 0 };
            },
            Instruction::Sub(vx, vy) => {
                let (result, overflow) = self.registers[*vx as usize].overflowing_sub(self.registers[*vy as usize]);
                self.registers[*vx as usize] = result;
                self.registers[0xF] = if overflow { 0 } else { 1 };
            },
            Instruction::ShiftRight(vx, vy) => {
                let vx = *vx as usize;
                self.registers[vx] = self.registers[*vy as usize];
                self.registers[0xF] = self.registers[vx] & 0x1;
                self.registers[vx] >>= 1;
            },
            Instruction::ShiftLeft(vx, vy) => {
                let vx = *vx as usize;
                self.registers[vx] = self.registers[*vy as usize];
                self.registers[0xF] = self.registers[vx] & 0x1;
                self.registers[vx] <<= 1;
            },
            _ => println!("Instruction not implemented for the ALU: {:?}", instruction)
        }

        Ok(())
    }
}