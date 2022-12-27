use std::io::Read;

use crate::cpu::CPU;
use crate::drivers::*;

pub const START_RAM_ADDRESS: usize = 0x200;

pub struct Emulator {
    cpu: CPU,
    display_driver: display::DisplayDriver,
    RAM: [u8; 4096],
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    ClearScreen,
    Jump(u16),
    SetRegister(u8, u8),
    AddValueRegister(u8, u8),
    SetIndexRegister(u16),
    Draw {
        vx: u8,
        vy: u8,
        n: u8,
    },
    Nothing,
    NotYetImplemented(u16),
}

impl Emulator {
    pub fn new(context: &sdl2::Sdl) -> Result<Emulator, String> {
        let mut display_driver = display::DisplayDriver::new(context)?;

        display_driver.init()?;
        Ok(Emulator {
            // ...
            cpu: CPU::new(),
            display_driver,
            RAM: [0; 4096],
            // ...
        })
    }

    pub fn load_program(&mut self, path: &str) -> Result<(), String> {
        let mut file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(format!("Could not open file {}", path)),
        };
        
        file.read(&mut self.RAM[START_RAM_ADDRESS..]).unwrap();
     
        Ok(())
    }

    pub fn next_instruction(&mut self) -> Result<(), String> {
        let pc = self.cpu.pc as usize;
        let opcode = (self.RAM[pc] as u16) << 8 | self.RAM[pc + 1] as u16;
        let instruction = Instruction::from(opcode);
        println!("{} - {:?}", pc, instruction);
        match instruction {
            Instruction::ClearScreen => {
                self.display_driver.clear_screen()?;
            },
            Instruction::Jump(address) => {
                self.cpu.pc = address;
                return Ok(());
            },
            Instruction::SetRegister(register, value) => {
                self.cpu.registers[register as usize] = value;
            },
            Instruction::AddValueRegister(register, value) => {
                self.cpu.registers[register as usize] += value;
            },
            Instruction::SetIndexRegister(value) => {
                self.cpu.I = value;
            },
            Instruction::Draw { vx, vy, n } => {
                let mut x = self.cpu.registers[vx as usize] % 64;
                let mut y = self.cpu.registers[vy as usize] % 32;
                self.cpu.registers[15] = 0;
                for i in 0..n {
                    let byte = self.RAM[self.cpu.I as usize + i as usize];
                    for j in 0..8 {
                        if byte & (0b1 >> j) == 0b1 {
                            if self.display_driver.get_pixel(x as usize, y as usize)  {
                                self.cpu.registers[15] = 1;
                                self.display_driver.set_pixel(x as usize, y as usize, false);
                            } else if !self.display_driver.get_pixel(x as usize, y as usize)  {
                                self.display_driver.set_pixel(x as usize, y as usize, true);
                            }
                        }
                        x += 1;
                        if x >= 64 {
                            x = 63;
                            break;
                        }
                    }
                    y += 1;
                    if y >= 32 {
                        y = 31;
                        break;
                    }
                }
                self.display_driver.draw()?;
            },
            Instruction::Nothing => {},
            Instruction::NotYetImplemented(opcode) => {
                println!("Not yet implemented: {:#X}", opcode);
            },
        }
        if self.cpu.pc <= 0xFFF {
            self.cpu.pc += 2;
        }
        Ok(())
        
    }

}

impl Instruction {
    pub fn from(opcode: u16) -> Self {
        match opcode {
            0x00E0 => {
                Instruction::ClearScreen
            },
            _ if 0xF000 & opcode == 0x1000 => {
                let address = opcode & 0x0FFF;
                Instruction::Jump(address)
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
            }
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

        // Failure
        let opcode = 0x2FFF;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::NotYetImplemented(0x2FFF));
    }
    
    #[test]
    fn test_decode_set_register() {
        // Success
        let opcode = 0x6F2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::SetRegister(0xF, 0x2F));

        // Failure
        let opcode = 0x8F2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::NotYetImplemented(0x8F2F));
    }

    #[test]
    fn test_decode_add_register() {
        // Success
        let opcode = 0x7F2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::AddValueRegister(0xF, 0x2F));

        // Failure
        let opcode = 0x9F2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::NotYetImplemented(0x9F2F));
    }

    #[test]
    fn test_decode_set_index_register() {
        // Success
        let opcode = 0xAF2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::SetIndexRegister(0xF2F));

        // Failure
        let opcode = 0xBF2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::NotYetImplemented(0xBF2F));
    }

    #[test]
    fn test_draw() {
        // Success
        let opcode = 0xDF2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::Draw { vx: 0xF, vy: 0x2, n: 0xF});

        // Failure
        let opcode = 0xEF2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::NotYetImplemented(0xEF2F));
    }
    
}
