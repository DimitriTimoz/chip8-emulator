use crate::cpu::CPU;
use crate::drivers::*;
pub struct Emulator {
    cpu: CPU,
    display_driver: display::DisplayDriver,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    ClearScreen,
    Jump(u16),
    SetRegister(u8, u8),
    AddValueRegister(u8, u8),
    SetIndexRegister(u16),
    Draw {
        x: u8,
        y: u8,
        n: u8,
    },
    Nothing,
    NotYetImplemented,
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
                let y = ((opcode & 0x0F0) >> 4) as u8;
                let x = ((opcode & 0xF00) >> 8) as u8;
                Instruction::Draw { x, y, n}
            }
            _ => Instruction::NotYetImplemented,
            
            
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
        assert_eq!(instruction, Instruction::NotYetImplemented);
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
        assert_eq!(instruction, Instruction::NotYetImplemented);
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
        assert_eq!(instruction, Instruction::NotYetImplemented);
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
        assert_eq!(instruction, Instruction::NotYetImplemented);
    }

    #[test]
    fn test_draw() {
        // Success
        let opcode = 0xDF2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::Draw { x: 0xF, y: 0x2, n: 0xF});

        // Failure
        let opcode = 0xEF2F;
        let instruction = Instruction::from(opcode);
        assert_eq!(instruction, Instruction::NotYetImplemented);
    }
    
}
