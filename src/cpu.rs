

struct Timer {
    counter: u8,
}

pub(crate) struct CPU {
    pc: u16,
    registers: [u8; 16],
    timer: Timer,
    sound_timer: Timer,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: 0x000,
            registers: [0; 16],
            timer: Timer { counter: 0 },
            sound_timer: Timer { counter: 0 },
        }
    }
}