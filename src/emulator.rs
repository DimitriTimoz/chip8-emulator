use std::io::Read;

use crate::cpu::{CPU, ALU};
use crate::drivers::*;

pub const START_RAM_ADDRESS: usize = 0x200;

pub struct Emulator {
    cpu: CPU,
    display_driver: display::DisplayDriver,
    ram: [u8; 4096],
    stack: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    ClearScreen,
    Jump(u16),
    SetRegister(u8, u8),
    AddValueRegister(u8, u8),
    SetIndexRegister(u16),
    SetIndexFromRegister(u8),
    Draw {
        vx: u8,
        vy: u8,
        n: u8,
    },
    Nothing,
    NotYetImplemented(u16),
    CallSubroutine(u16),
    ReturnFromSubroutine,
    SkipIfEq(u8, u8),
    SkipIfNotEq(u8, u8),
    SkipIfRegEq(u8, u8),
    SkipIfRegNotEq(u8, u8),
    SetToValue(u8, u8),
    Or(u8, u8),
    And(u8, u8),
    Xor(u8, u8),
    Add(u8, u8),
    Sub(u8, u8),
    ShiftRight(u8, u8),
    ShiftLeft(u8, u8),
    StoreMemory(u8),
    LoadMemory(u8),
    DecimalConversion(u8),
}

impl Emulator {
    pub fn new(context: &sdl2::Sdl) -> Result<Emulator, String> {
        let mut display_driver = display::DisplayDriver::new(context)?;

        display_driver.init()?;
        let mut ram = [0; 4096];
        let FONTSET: [u8; 80] = [
            /* 0 */ 0xF0, 0x90, 0x90, 0x90, 0xF0,
            /* 1 */ 0x20, 0x60, 0x20, 0x20, 0x70,
            /* 2 */ 0xF0, 0x10, 0xF0, 0x80, 0xF0,
            /* 3 */ 0xF0, 0x10, 0xF0, 0x10, 0xF0,
            /* 4 */ 0x90, 0x90, 0xF0, 0x10, 0x10,
            /* 5 */ 0xF0, 0x80, 0xF0, 0x10, 0xF0,
            /* 6 */ 0xF0, 0x80, 0xF0, 0x90, 0xF0,
            /* 7 */ 0xF0, 0x10, 0x20, 0x40, 0x40,
            /* 8 */ 0xF0, 0x90, 0xF0, 0x90, 0xF0,
            /* 9 */ 0xF0, 0x90, 0xF0, 0x10, 0xF0,
            /* A */ 0xF0, 0x90, 0xF0, 0x90, 0x90,
            /* B */ 0xE0, 0x90, 0xE0, 0x90, 0xE0,
            /* C */ 0xF0, 0x80, 0x80, 0x80, 0xF0,
            /* D */ 0xE0, 0x90, 0x90, 0x90, 0xE0,
            /* E */ 0xF0, 0x80, 0xF0, 0x80, 0xF0,
            /* F */ 0xF0, 0x80, 0xF0, 0x80, 0x80,
            ];

        ram[0x00..FONTSET.len()].copy_from_slice(&FONTSET[..]);
        Ok(Emulator {
            // ...
            cpu: CPU::new(),
            display_driver,
            ram,
            stack: Vec::new(),
            // ...
        })
    }

    pub fn load_program(&mut self, path: &str) -> Result<(), String> {
        let mut file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(format!("Could not open file {}", path)),
        };
        
        file.read(&mut self.ram[START_RAM_ADDRESS..]).unwrap();
     
        Ok(())
    }

    pub fn execute(&mut self, instruction: &Instruction) -> Result<(), String> {
        match instruction {
            Instruction::ClearScreen => {
                self.display_driver.clear_screen()?;
            },
            Instruction::Jump(address) => {
                self.cpu.pc = *address;
                return Ok(());
            },
            Instruction::SetRegister(register, value) => {
                self.cpu.registers[*register as usize] = *value;
            },
            Instruction::AddValueRegister(register, value) => {
                let (v, _) = self.cpu.registers[*register as usize].overflowing_add(*value);
                self.cpu.registers[*register as usize] = v;
            },
            Instruction::SetIndexRegister(value) => {
                self.cpu.I = *value;
            },
            Instruction::Draw { vx, vy, n } => {
                let x = self.cpu.registers[*vx as usize] % 64;
                let y = self.cpu.registers[*vy as usize] % 32;
                self.cpu.registers[15] = 0;
                for i in 0..*n {
                    let byte = self.ram[self.cpu.I as usize + i as usize];
                    let y = y + i;
                    for j in 0..8 {
                        let x = x + j;
                        if byte & (0b10000000 >> j) != 0 {
                            if self.display_driver.get_pixel(x as usize, y as usize)  {
                                self.cpu.registers[15] = 1;
                                self.display_driver.set_pixel(x as usize, y as usize, false);
                            } else if !self.display_driver.get_pixel(x as usize, y as usize)  {
                                self.display_driver.set_pixel(x as usize, y as usize, true);
                            }
                        }
                        if x >= 64 {
                            break;
                        }
                    }
                    if y >= 32 {
                        break;
                    }
                }
                self.display_driver.draw()?;
            },
            Instruction::CallSubroutine(address) => {
                self.stack.push(((self.cpu.pc >> 8) & 0xFF) as u8);
                self.stack.push((self.cpu.pc & 0xFF) as u8);
                self.cpu.pc = *address;
                return Ok(());
            },
            Instruction::ReturnFromSubroutine => {
                if self.stack.len() > 1 {
                    let b1 = self.stack.pop().unwrap();
                    let b2 = self.stack.pop().unwrap();
                    self.cpu.pc = (b1 as u16 | ((b2 as u16) << 8)) + 2;
                    return Ok(());
                } else {
                    println!("Stack doesn't contain PC to return from the subroutine");
                }
            },
            Instruction::SkipIfEq(vx, n) => {
                let value = self.cpu.registers[*vx as usize];
                if value == *n {
                    self.cpu.pc += 2;
                }
            },
            Instruction::SkipIfNotEq(vx, n) => {
                let value = self.cpu.registers[*vx as usize];
                if value != *n {
                    self.cpu.pc += 2;
                }
            },
            Instruction::SkipIfRegEq(vx, vy) => {
                let vx = self.cpu.registers[*vx as usize];
                let vy = self.cpu.registers[*vy as usize];
                if vx == vy {
                    self.cpu.pc += 2;
                }
            },
            Instruction::SkipIfRegNotEq(vx, vy) => {
                let vx = self.cpu.registers[*vx as usize];
                let vy = self.cpu.registers[*vy as usize];
                if vx != vy {
                    self.cpu.pc += 2;
                }
            },
            Instruction::Nothing => {},
            Instruction::NotYetImplemented(opcode) => {
                println!("Not yet implemented: {:#X}", opcode);
            },
            Instruction::StoreMemory(x) => {
                for i in 0..=*x {
                    self.ram[self.cpu.I as usize + i as usize] = self.cpu.registers[i as usize];
                }
            },
            Instruction::LoadMemory(x) => {
                for i in 0..=*x {
                    self.cpu.registers[i as usize] = self.ram[self.cpu.I as usize + i as usize];
                }
            },
            Instruction::DecimalConversion(vx) => {
                let vx = self.cpu.registers[*vx as usize];
                self.ram[self.cpu.I as usize] = vx / 100;
                self.ram[self.cpu.I as usize + 1] = (vx % 100) / 10;;
                self.ram[self.cpu.I as usize + 2] = vx % 10;
            },
            Instruction::SetIndexFromRegister(vx) => {
                self.cpu.I = self.cpu.registers[*vx as usize] as u16;
            },
            _ => {
                self.cpu.execute(instruction)?
            }

        }

        if self.cpu.pc <= 0xFFF {
            self.cpu.pc += 2;
        }

        Ok(())
    }
    
    pub fn next_instruction(&mut self) -> Result<(), String> {
        let pc = self.cpu.pc as usize;
        let opcode = (self.ram[pc] as u16) << 8 | self.ram[pc + 1] as u16;
        let instruction = Instruction::from(opcode);
        println!("{} {:#X} - {:?} ", pc, opcode, instruction);
        self.execute(&instruction)?;
       
        Ok(())
    }
 

}

impl Instruction {
    pub fn from(opcode: u16) -> Self {
        match opcode {
            0x00E0 => {
                Instruction::ClearScreen
            },
            0x00EE => {
                Instruction::ReturnFromSubroutine
            },
            _ if 0xF000 & opcode == 0x8000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let vy = ((opcode & 0x00F0) >> 4) as u8;
                match opcode & 0x000F {
                    0x0 => Instruction::SetToValue(vx, vy),
                    0x1 => Instruction::Or(vx, vy),
                    0x2 => Instruction::And(vx, vy),
                    0x3 => Instruction::Xor(vx, vy),
                    0x4 => Instruction::Add(vx, vy),
                    0x5 => Instruction::Sub(vx, vy),
                    0x6 => Instruction::ShiftRight(vx, vy),
                    0x7 => Instruction::Sub(vx, vy),
                    0xE => Instruction::ShiftLeft(vx, vy),
                    _ => Instruction::NotYetImplemented(opcode),
                }
            },
            _ if 0xF000 & opcode == 0x1000 => {
                let address = opcode & 0x0FFF;
                Instruction::Jump(address)
            },
            _ if 0xF000 & opcode == 0x3000 => {
                let register = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;
                Instruction::SkipIfEq(register, value)
            },
            _ if 0xF000 & opcode == 0x4000 => {
                let register = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;
                Instruction::SkipIfNotEq(register, value)
            },
            _ if 0xF000 & opcode == 0x6000 => {
                let register = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;
                Instruction::SetRegister(register, value)
            },
            _ if 0xF000 & opcode == 0x7000 => {
                let register  = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;
                Instruction::AddValueRegister(register, value)
            },
            _ if 0xF000 & opcode == 0xA000 => {
                let value = (opcode & 0x0FFF) as u16;
                Instruction::SetIndexRegister(value)
            },
            _ if 0xF000 & opcode == 0xD000 => {
                let n = (opcode & 0x000F) as u8;
                let vy = ((opcode & 0x0F0) >> 4) as u8;
                let vx = ((opcode & 0xF00) >> 8) as u8;
                Instruction::Draw { vx, vy, n}
            },
            _ if 0xF000 & opcode == 0x2000 => {
                let address = opcode & 0x0FFF;
                Instruction::CallSubroutine(address)
            },
            _ if 0xF000 & opcode == 0x5000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let vy = ((opcode & 0x00F0) >> 4) as u8;
                Instruction::SkipIfRegEq(vx, vy)
            },
            _ if 0xF000 & opcode == 0x9000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let vy = ((opcode & 0x00FF) >> 4) as u8;
                Instruction::SkipIfRegNotEq(vx, vy)
            },
            _ if 0xF0FF & opcode == 0xF055 => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                Instruction::StoreMemory(x)
            },
            _ if 0xF0FF & opcode == 0xF065 => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                Instruction::LoadMemory(x)
            },
            _ if 0xF0FF & opcode == 0xF033 => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                Instruction::DecimalConversion(x)
            },
            _ if 0xF0FF & opcode == 0xF029 => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                Instruction::SetIndexFromRegister(x)
            },
            0x0000 => Instruction::Nothing,
            _ => Instruction::NotYetImplemented(opcode),
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_clear_screen() {
        let opcode = 0x00E0;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::ClearScreen);
    }

    #[test]
    fn test_decode_jump() {
        // Success
        let opcode = 0x1FFF;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::Jump(0xFFF));

        let opcode = 0x1F2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::Jump(0xF2F));
    }
    
    #[test]
    fn test_decode_set_register() {
        // Success
        let opcode = 0x6F2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::SetRegister(0xF, 0x2F));
    }

    #[test]
    fn test_decode_add_register() {
        // Success
        let opcode = 0x7F2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::AddValueRegister(0xF, 0x2F));

    }

    #[test]
    fn test_decode_set_index_register() {
        // Success
        let opcode = 0xAF2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::SetIndexRegister(0xF2F));
    }

    #[test]
    fn test_draw() {
        // Success
        let opcode = 0xDF2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::Draw { vx: 0xF, vy: 0x2, n: 0xF});
    }

    #[test]
    fn test_subroutine() {
        // Success
        let opcode = 0x2F2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::CallSubroutine(0xF2F));

        // Return
        let opcode = 0x00EE;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::ReturnFromSubroutine);
    }

    #[test]
    fn test_skip_if_eq_and_neq() {
        // Success
        let opcode = 0x3F2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::SkipIfEq(0xF, 0x2F));

        let opcode = 0x4F2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::SkipIfNotEq(0xF, 0x2F));
    }

    #[test]
    fn test_skip_if_reg_eq_and_neq() {
        // Success
        let opcode = 0x5F2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::SkipIfRegEq(0xF, 0x2));

        let opcode = 0x9F2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::SkipIfRegNotEq(0xF, 0x2));
    }

    #[test]
    fn test_alu_instructions() {
        // Success
        let opcode = 0x8F20;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::SetToValue(0xF, 0x2));

        let opcode = 0x8F21;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::Or(0xF, 0x2));

        let opcode = 0x8F22;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::And(0xF, 0x2));

        let opcode = 0x8F23;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::Xor(0xF, 0x2));

        let opcode = 0x8F24;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::Add(0xF, 0x2));

        let opcode = 0x8F25;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::Sub(0xF, 0x2));

        let opcode = 0x8F26;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::ShiftRight(0xF, 0x2));
        
        let opcode = 0x8F27;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::Sub(0xF, 0x2));

        let opcode = 0x8F2E;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::ShiftLeft(0xF, 0x2));
    }

    #[test]
    fn test_store_and_load_memory() {
        // Success
        let opcode = 0xF155;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::StoreMemory(0x1));

        let opcode = 0xF165;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::LoadMemory(0x1));
    }

    #[test]
    fn test_decimal_conversion() {
        // Success
        let opcode = 0xF233;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::DecimalConversion(0x2));
    }

    #[test]
    fn test_index_from_register() {
        // Success
        let opcode = 0xF229;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::SetIndexFromRegister(0x2));
    }
    
}
