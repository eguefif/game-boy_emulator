use crate::processor::registers::Registers;

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
        }
    }

    pub fn run(self: &mut Cpu) -> u8 {
        println!("{:?}", self);
        println!("{:?}", self.registers);
        1
    }
}
