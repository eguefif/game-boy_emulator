#![allow(dead_code)]
use crate::processor::instructions::Instruction;
use crate::processor::memorybus::MemoryBus;
use crate::processor::registers::Registers;

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    pub memory: MemoryBus,
    pub pause: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            memory: MemoryBus::new(),
            pause: false,
        }
    }

    pub fn run(self: &mut Cpu) -> u8 {
        loop {
            if !self.is_halted() {
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
        }
        1
    }

    fn execute(&mut self, instruction: Instruction) -> bool {
        match instruction {
            Instruction::Halt => self.halt(),
            Instruction::Daa() => self.daa(),
            Instruction::Scf() => self.scf(),
            Instruction::Inc(target) => self.inc_dispatch(target),
            Instruction::Add(target) => self.add_dispatch(target),
            Instruction::AddC(target) => self.addc_dispatch(target),
            Instruction::Sub(target) => self.sub_dispatch(target),
            Instruction::SubC(target) => self.subc_dispatch(target),
            Instruction::And(target) => self.and_dispatch(target),
            Instruction::Xor(target) => self.xor_dispatch(target),
            Instruction::Or(target) => self.or_dispatch(target),

            Instruction::LoadA(target) => self.loada_dispatch(target),
            Instruction::LoadB(target) => self.loadb_dispatch(target),
            Instruction::LoadC(target) => self.loadc_dispatch(target),
            Instruction::LoadD(target) => self.loadd_dispatch(target),
            Instruction::LoadE(target) => self.loade_dispatch(target),
            Instruction::LoadH(target) => self.loadh_dispatch(target),
            Instruction::LoadL(target) => self.loadl_dispatch(target),
            Instruction::LoadHL(target) => self.loadhl_dispatch(target),
            Instruction::Load16(target) => self.load16_dispatch(target),
            Instruction::Load8(target) => self.load8_dispatch(target),
            Instruction::LoadByteA(target) => self.loadbytea_dispatch(target),

            Instruction::Cp(target) => self.comp_dispatch(target),
            Instruction::Nop => self.registers.pc += 1,
            Instruction::Exit => return true,
        }
        false
    }
}
