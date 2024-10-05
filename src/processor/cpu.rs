#![allow(dead_code)]
use crate::processor::instructions::Instruction;
use crate::processor::registers::Registers;

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    pub memory: [u8; 0xFFFF],
    pub pc: usize,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            memory: [0; 0xFFFF],
            pc: 0,
        }
    }

    pub fn run(self: &mut Cpu) -> u8 {
        let opcode = self.memory[self.pc];
        self.pc += 1;

        if let Some(instruction) = Instruction::from_byte(opcode) {
            self.execute(instruction);
        } else {
            println!("Unknown opcode: {:#x}", opcode);
        }
        1
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(target) => self.add_dispatch(target),
        }
    }

    fn fetch_word(self: &mut Cpu) -> u16 {
        let value = self.memory[self.registers.pc] as u16;
        let value_2 = self.memory[self.registers.pc + 1] as u16;
        self.registers.pc += 2;
        (value_2 << 8) | value
    }

    fn set_word(self: &mut Cpu, value: u16) {
        let value_1: u8 = (value >> 8) as u8;
        let value_2: u8 = (value & 0xFF) as u8;
        self.memory[self.registers.pc] = value_1;
        self.memory[self.registers.pc + 1] = value_2;
    }

    fn fetch_byte(self: &mut Cpu) -> u8 {
        let value = self.memory[self.registers.pc];
        self.registers.pc += 1;
        value
    }

    fn set_byte(self: &mut Cpu, value: u8) {
        self.memory[self.registers.pc] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_fetch_word_from_memory() {
        let mut cpu = Cpu::new();
        cpu.registers.pc = 0xFF;
        let save_pc = cpu.registers.pc;
        cpu.memory[0xFF] = 0xab;
        cpu.memory[0xFF + 1] = 0xa0;
        let res = cpu.fetch_word();

        assert_eq!(res, 0xa0ab);
        assert_eq!(cpu.registers.pc, save_pc + 2);
    }

    #[test]
    fn it_set_word_from_memory() {
        let mut cpu = Cpu::new();
        cpu.registers.pc = 0xFF;
        cpu.set_word(0xf80a);

        assert_eq!(cpu.memory[cpu.registers.pc], 0xf8);
        assert_eq!(cpu.memory[cpu.registers.pc + 1], 0x0a);
    }

    #[test]
    fn it_fetch_byte_from_memory() {
        let mut cpu = Cpu::new();
        cpu.registers.pc = 0xFF;
        let save_pc = cpu.registers.pc;
        cpu.memory[0xFF] = 0xab;
        let res = cpu.fetch_byte();

        assert_eq!(res, 0xab);
        assert_eq!(cpu.registers.pc, save_pc + 1);
    }

    #[test]
    fn it_set_byte_from_memory() {
        let mut cpu = Cpu::new();
        cpu.registers.pc = 0xFF;
        cpu.set_byte(0xf8);

        assert_eq!(cpu.memory[cpu.registers.pc], 0xf8);
    }
}
