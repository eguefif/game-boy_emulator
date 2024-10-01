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
            0x0 => self.range_0_op(opcode),
            0x4 => self.load_b_c_dispatch(opcode),
            0x5 => self.load_d_e_dispatch(opcode),
            0x6 => self.load_h_l_dispatch(opcode),
            0x7 => self.load_memory_reg_a_dispatch(opcode),
            0x8 => self.add_dispatch(opcode),
            _ => eprintln!("Opcode {:x?} not implemented.", opcode),
        }
        1
    }

    pub fn fetch_word(self: &mut Cpu) -> u16 {
        let value = self.memory[self.registers.pc] as u16;
        let value_2 = self.memory[self.registers.pc + 1] as u16;
        self.registers.pc += 2;
        (value_2 << 8) | value
    }

    pub fn set_word(self: &mut Cpu, value: u16) {
        let value_1: u8 = (value & 0xF) as u8;
        let value_2: u8 = (value >> 8) as u8 & 0xF;
        self.memory[self.registers.pc] = value_1;
        self.memory[self.registers.pc + 1] = value_2;
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

        assert_eq!(cpu.memory[cpu.registers.pc], 0x0a);
        assert_eq!(cpu.memory[cpu.registers.pc + 1], 0xf8);
    }
}
