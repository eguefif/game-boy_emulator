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
        (value << 8) | value_2
    }

    pub fn set_word(self: &mut Cpu, value: u16) {
        let value: u8 = (value >> 8) as u8 & 0xF;
        let value_2: u8 = value & 0xF;
        self.memory[self.registers.pc] = value;
        self.memory[self.registers.pc + 1] = value_2;
    }
}
