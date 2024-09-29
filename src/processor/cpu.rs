use crate::processor::registers::Registers;

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    pub memory: [u8; 0xFFFF],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            memory: [0; 0xFFFF],
        }
    }

    pub fn run(self: &mut Cpu) -> u8 {
        let opcode: u8 = 0x80;
        let high_opcode: u8 = (opcode >> 4) & 0xF;
        match high_opcode {
            0x8 => self.add_dispatch(opcode),
            _ => eprintln!("Opcode {:x?} not implemented.", opcode),
        }
        1
    }
}
