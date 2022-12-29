use std::io::Read;

use crate::{emulator::{START_RAM_ADDRESS, FONT_OFFSET}, drivers::display::{WIDTH, HEIGHT, DisplayDriver}};



struct Timer {
    counter: u8,
}

pub(crate) struct CPU {
    pc: u16,
    registers: [u8; 16],
    I: u16,
    ram: [u8; 4096],
    vram: [[bool; WIDTH as usize]; HEIGHT as usize],
    vram_changed: bool,
    stack: Vec<u8>,
    timer: Timer,
    sound_timer: Timer,
}

enum PCIncrement {
    Increment,
    DontIncrement,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: START_RAM_ADDRESS as u16,
            registers: [0; 16],
            timer: Timer { counter: 0 },
            sound_timer: Timer { counter: 0 },
            I: 0,
            ram: [0; 4096],
            vram: [[false; WIDTH as usize]; HEIGHT as usize],
            vram_changed: false,
            stack: Vec::new(),
        }
    }

    fn clear_vram(&mut self) -> PCIncrement {
        self.vram = [[false; WIDTH as usize]; HEIGHT as usize];
        self.vram_changed = true;
        PCIncrement::Increment
    }

    fn op_0x00EE(&mut self) -> PCIncrement{
        let address = self.stack.pop().unwrap();
        self.pc = address as u16;
        PCIncrement::DontIncrement
    }

    fn op_0xDXYN(&mut self, vx: u8, vy: u8, n: u8) -> PCIncrement {
        let x = self.registers[vx as usize] % 64;
        let y = self.registers[vy as usize] % 32;
        self.vram_changed = true;
        self.registers[0xF] = 0;
        for i in 0..n {
            let byte = self.ram[self.I as usize + i as usize];
            let y = y + i;
            if y >= 32 {
                break;
            }
            for j in 0..8 {
                let x = x + j;
                if x >= 64 {
                    break;
                }
                if byte & (0b10000000 >> j) != 0 {
                    if self.vram[y as usize][x as usize] {
                        self.registers[0xF] = 1;
                        self.vram[y as usize][x as usize] = false;
                    } else if !self.vram[y as usize][x as usize]  {
                        self.vram[y as usize][x as usize] = true;
                    }
                }
            }
        }
        PCIncrement::Increment
    }

    pub fn next_instruction(&mut self, opcode: u16) -> Result<(), String> {
        println!("-: {:X}", opcode);
        let operation = (opcode & 0xF000) >> 12;
        let next = match operation {
            // Clear VRAM
            0x0 => {
                let opcode = opcode & 0x00FF;
                match opcode {
                    0xE0 => {
                        self.clear_vram()
                    },
                    // Return from subroutine
                    0xEE => {
                        self.op_0x00EE()
                    },
                    _ => {
                        println!("Unknown opcode: {:X}", opcode);
                        PCIncrement::Increment
                    }
                }
            },
            // Jump to address
            0x1 => {
                self.pc = (opcode & 0x0FFF) as u16;
                PCIncrement::DontIncrement
            },
            // Call subroutine
            0x2 => {
                let address = opcode & 0x0FFF;
                self.stack.push((self.pc & 0xFF) as u8);
                self.stack.push(((self.pc >> 8) & 0xFF) as u8);
                self.pc = address as u16;
                PCIncrement::DontIncrement
            },
            // Skip if equal
            0x3 => {
                let register = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;
                if value == self.registers[register as usize] {
                    self.pc += 2;
                }
                PCIncrement::Increment
            },
            // Skip if not equal
            0x4 => {
                let register = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;
                if value != self.registers[register as usize] {
                    self.pc += 2;
                }
                PCIncrement::Increment
            },
            // Skip if register equal
            0x5 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let vy = ((opcode & 0x00F0) >> 4) as u8;
                if self.registers[vx as usize] == self.registers[vy as usize] {
                    self.pc += 2;
                }
                PCIncrement::Increment
            },
            // Set register to value
            0x6 => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let nn = (opcode & 0x00FF) as u8;
                self.registers[x as usize] = nn;
                PCIncrement::Increment
            },
            // Add value to register
            0x7 => {
                let x  = ((opcode & 0x0F00) >> 8) as u8;
                let nn = (opcode & 0x00FF) as u8;
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(nn);
                PCIncrement::Increment
            },
            // ALU
            0x8 => {
                let operation = opcode & 0x000F;
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let vy = ((opcode & 0x00F0) >> 4) as u8;
                match operation {
                    0x0 => self.registers[vx as usize] = self.registers[vy as usize],
                    0x1 => self.registers[vx as usize] |= self.registers[vy as usize],
                    0x2 => self.registers[vx as usize] &= self.registers[vy as usize],
                    0x3 => self.registers[vx as usize] ^= self.registers[vy as usize],
                    0x4 => {
                        let (result, overflow) = self.registers[vx as usize].overflowing_add(self.registers[vy as usize]);
                        self.registers[vx as usize] = result;
                        self.registers[0xF] =  overflow as u8;
                    },
                    0x5 => {
                        let (result, overflow) = self.registers[vx as usize].overflowing_sub(self.registers[vy as usize]);
                        self.registers[vx as usize] = result;
                        self.registers[0xF] =  !overflow as u8;
                    },
                    0x6 => {
                        self.registers[0xF] = self.registers[vx as usize] & 0x1;
                        self.registers[vx as usize] >>= 1;
                    },
                    0x7 => {
                        let (result, overflow) = self.registers[vy as usize].overflowing_sub(self.registers[vx as usize]);
                        self.registers[vx as usize] = result;
                        self.registers[0xF] =  !overflow as u8;
                    },
                    0xE => {
                        self.registers[0xF] = (self.registers[vx as usize] & 0x80) >> 7;
                        self.registers[vx as usize] <<= 1;
                    },
                    _ => return Err(format!("Unknown operation: {:X}", operation))
                }
                PCIncrement::Increment
            },
            0x9 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let vy = ((opcode & 0x00F0) >> 4) as u8;
                if self.registers[vx as usize] != self.registers[vy as usize] {
                    self.pc += 2;
                }
                PCIncrement::Increment
            }
            0xA => {
                let value = (opcode & 0x0FFF) as u16;
                self.I= value;
                PCIncrement::Increment
            },
            0xB => {
                let value = (opcode & 0x0FFF) as u16;
                self.pc = value + self.registers[0] as u16;
                PCIncrement::DontIncrement
            },
            0xC => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let nn = (opcode & 0x00FF) as u8;
                let random = rand::random::<u8>();
                self.registers[x as usize] = random & nn;
                PCIncrement::Increment
            }
            0xD => {
                let n = (opcode & 0x000F) as u8;
                let vy = ((opcode & 0x0F0) >> 4) as u8;
                let vx = ((opcode & 0xF00) >> 8) as u8;
                self.op_0xDXYN(vx, vy, n)
            },
            // Keyboard
            0xE => {
                let opcode = opcode & 0x00FF;
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let key = self.registers[x as usize];
                match opcode {
                    0x9E => {},
                    0xA1 => {},
                    _ => return Err(format!("Unknown operation: {:X}", opcode))
                }
                PCIncrement::Increment
            }
            0xF => {
                let operation = opcode & 0x00FF;
                let x = ((opcode & 0x0F00) >> 8) as u8;
                match operation {
                    0x1E => {
                        self.I += self.registers[x as usize] as u16;
                    },
                    0x29 => self.I = FONT_OFFSET as u16 + self.registers[x as usize] as u16,
                    0x33 => {
                        let value = self.registers[x as usize];
                        self.ram[self.I as usize] = value / 100;
                        self.ram[self.I as usize + 1] = (value / 10) % 10;
                        self.ram[self.I as usize + 2] = (value % 100) % 10;
                    },
                    0x55 => {
                        for i in 0..=x {
                            self.ram[self.I as usize + i as usize] = self.registers[i as usize];
                        }
                    },
                    0x65 => {
                        for i in 0..=x {
                            self.registers[i as usize] = self.ram[self.I as usize + i as usize];
                        }
                    }
                    _ => {
                        eprintln!("Unknown opcode: {:X}", opcode);
                    }
                }
                PCIncrement::Increment
            },
            0x0000 => PCIncrement::Increment,
            _ => { 
                eprintln!("Unknown opcode: {:X}", opcode);
                PCIncrement::Increment
            },
        };
        if self.pc >= 0xFFF {
            self.pc %= 0xFFF;
        }
        match next {
            PCIncrement::DontIncrement => (),
            PCIncrement::Increment => self.pc += 2,
        }
        Ok(())
    }

    pub fn fetch_opcode(&self) -> u16 {
        let opcode = (self.ram[self.pc as usize] as u16) << 8 | self.ram[self.pc as usize + 1] as u16;
        opcode
    }

    pub fn cycle(&mut self, driver: &mut DisplayDriver) -> Result<(), String> {
        let opcode = self.fetch_opcode();
        self.next_instruction(opcode)?;
        if self.vram_changed {
            driver.draw(&self.vram)?;
            self.vram_changed = false;
        }
        Ok(())
    }

    pub fn load_ram(&mut self, path: &str) -> Result<(), String> {
        let mut file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(format!("Could not open file {}", path)),
        };
        
        file.read(&mut self.ram[START_RAM_ADDRESS..]).unwrap();
        Ok(())
    }
}
