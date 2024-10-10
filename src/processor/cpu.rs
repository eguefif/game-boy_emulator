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
        loop {
            let opcode = self.memory.fetch_next_instruction();
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
            Instruction::AddC(target) => self.addc_dispatch(target),
            Instruction::LoadA(target) => self.loada_dispatch(target),
            Instruction::LoadB(target) => self.loadb_dispatch(target),
            Instruction::LoadC(target) => self.loadc_dispatch(target),
            Instruction::LoadD(target) => self.loadd_dispatch(target),
            Instruction::LoadE(target) => self.loade_dispatch(target),
            Instruction::LoadH(target) => self.loadh_dispatch(target),
            Instruction::LoadL(target) => self.loadl_dispatch(target),
            Instruction::LoadHL(target) => self.loadhl_dispatch(target),
            Instruction::End => return true,
        }
        false
    }
}
