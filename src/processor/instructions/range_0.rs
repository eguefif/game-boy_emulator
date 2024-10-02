use crate::processor::{
    cpu::Cpu,
    registers::{set_carry, set_h, set_n, set_zero},
};

use super::add::half_overflow_u16;

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
            0x4 => {
                let former_value = self.registers.bc();
                let (value, overflow) = former_value.overflowing_add(1);
                self.set_flags(value, overflow, former_value);
                self.registers.set_bc(value);
            }
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

    pub fn set_flags(self: &mut Cpu, value: u16, overflow: bool, former_value: u16) {
        let flags = self.registers.f;

        if value == 0 {
            self.registers.f = set_zero(flags);
        }
        if overflow {
            self.registers.f = set_carry(flags);
        }
        if half_overflow_u16(former_value, value) {
            self.registers.f = set_h(flags);
        }
        self.registers.f = set_n(flags);
    }
}
