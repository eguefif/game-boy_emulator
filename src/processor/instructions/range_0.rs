use crate::processor::{
    cpu::Cpu,
    registers::{set_carry, set_h, set_n_to_zero, set_zero},
};

use super::add::{add, half_overflow_u16};
use super::sub::sub;

impl Cpu {
    pub fn range_0_dispatch(self: &mut Cpu, opcode: u8) {
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
            0x4 => add(&mut self.registers.b, 1, &mut self.registers.f),
            0x5 => sub(&mut self.registers.b, 1, &mut self.registers.f),
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

    #[allow(dead_code)]
    pub fn set_flags(self: &mut Cpu, value: u16, overflow: bool, former_value: u16) {
        if value == 0 {
            self.registers.f = set_zero(self.registers.f);
        }
        if overflow {
            self.registers.f = set_carry(self.registers.f);
        }
        if half_overflow_u16(former_value, value) {
            self.registers.f = set_h(self.registers.f);
        }
        self.registers.f = set_n_to_zero(self.registers.f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_increment_with_no_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 5;

        cpu.range_0_dispatch(0x04);

        assert_eq!(cpu.registers.b, 6);
        assert_eq!(cpu.registers.f, 0b0000_0000);
    }

    #[test]
    fn it_increment_with_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0xff;

        cpu.range_0_dispatch(0x04);

        assert_eq!(cpu.registers.b, 0);
        assert_eq!(cpu.registers.f, 0b1011_0000);
    }

    #[test]
    fn it_decrement() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 5;

        cpu.range_0_dispatch(0x05);

        assert_eq!(cpu.registers.b, 4);
        assert_eq!(cpu.registers.f, 0b0100_0000);
    }

    #[test]
    fn it_decrement_to_zero() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x1;

        cpu.range_0_dispatch(0x05);

        assert_eq!(cpu.registers.b, 0);
        assert_eq!(cpu.registers.f, 0b1100_0000);
    }
}
