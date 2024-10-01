use crate::processor::cpu::Cpu;

impl Cpu {
    pub fn range_0_op(self: &mut Cpu, opcode: u8) {
        let low_opcode = opcode & 0xF;
        match low_opcode {
            0x0 => {}
            0x1 => {
                let value = self.fetch_word();
                self.registers.set_bc(value);
            }
            0x2 => self.memory[self.registers.bc() as usize] = self.registers.a,
            0x3 => {
                let bc_value = self.registers.bc() + 1;
                self.registers.set_bc(bc_value);
            }
            0x4 => eprintln!("Opcode {:x?} not implemented.", opcode),
            0x5 => eprintln!("Opcode {:x?} not implemented.", opcode),
            0x6 => eprintln!("Opcode {:x?} not implemented.", opcode),
            0x7 => eprintln!("Opcode {:x?} not implemented.", opcode),
            0x8 => {
                let value = self.registers.bc();
                self.set_word(value);
            }
            0x9 => eprintln!("Opcode {:x?} not implemented.", opcode),
            0xa => self.registers.a = self.memory[self.registers.bc() as usize],
            0xb => eprintln!("Opcode {:x?} not implemented.", opcode),
            0xc => eprintln!("Opcode {:x?} not implemented.", opcode),
            0xe => eprintln!("Opcode {:x?} not implemented.", opcode),
            0xf => eprintln!("Opcode {:x?} not implemented.", opcode),
            _ => eprintln!("Opcode {:x?} not implemented.", opcode),
        }
    }
}

// TODO: test set_word and fetch_word
