#![allow(dead_code)]
use crate::processor::instructions::Instruction;
use crate::processor::memorybus::MemoryBus;
use crate::processor::registers::Registers;

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    pub memory: MemoryBus,
    pub pc: usize,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            memory: MemoryBus::new(),
            pc: 0,
        }
    }

    pub fn run(self: &mut Cpu) -> u8 {
        let opcode = self.memory.fetch_next_instruction();

        loop {
            if let Some(instruction) = Instruction::from_byte(opcode) {
                let is_over = self.execute(instruction);
                if is_over {
                    break;
                }
            } else {
                println!("Unknown opcode: {:#x}", opcode);
            }
        }
        1
    }

    fn execute(&mut self, instruction: Instruction) -> bool {
        match instruction {
            Instruction::Add(target) => self.add_dispatch(target),
            Instruction::End => return true,
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_adds_b_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x80, 1);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x01;
        cpu.registers.b = 0x02;
        cpu.run();
        assert_eq!(cpu.registers.a, 0x01 + 0x02);
    }
}
