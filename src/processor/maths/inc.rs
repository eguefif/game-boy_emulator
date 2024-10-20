use crate::processor::cpu::Cpu;
use crate::processor::instructions::IncTarget;

use super::add::half_overflow_u8;

impl Cpu {
    pub fn inc_dispatch(&mut self, target: IncTarget) {
        match target {
            IncTarget::A => self.registers.a = self.inc(self.registers.a),
            IncTarget::B => self.registers.a = self.inc(self.registers.a),
            IncTarget::C => self.registers.a = self.inc(self.registers.a),
            IncTarget::D => self.registers.a = self.inc(self.registers.a),
            IncTarget::E => self.registers.a = self.inc(self.registers.a),
            IncTarget::H => self.registers.a = self.inc(self.registers.a),
            IncTarget::L => self.registers.a = self.inc(self.registers.a),
            IncTarget::BC => {
                let value = self.registers.bc();
                self.registers.set_bc(value.wrapping_add(1));
            }
            IncTarget::DE => {
                let value = self.registers.de();
                self.registers.set_de(value.wrapping_add(1));
            }
            IncTarget::SP => {
                let value = self.registers.sp;
                self.registers.sp = value.wrapping_add(1);
            }
            IncTarget::HL => {
                let value = self.registers.hl();
                self.registers.set_hl(value.wrapping_add(1));
            }
            IncTarget::HLFlags => {
                let position = self.registers.hl() as usize;
                let value = self.memory.fetch_byte_at(position);
                let result = self.inc(value);
                self.memory.set_byte(result, position);
            }
        }
    }

    fn inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.set_flags(result, value);
        result
    }

    fn set_flags(&mut self, result: u8, value: u8) {
        if result == 0 {
            self.registers.f.set_zero();
        } else {
            self.registers.f.unset_zero();
        }
        self.registers.f.unset_n();
        if half_overflow_u8(value, 1) {
            self.registers.f.set_h();
        } else {
            self.registers.f.unset_h();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_inc_register_b_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x04, 0);

        cpu.registers.b = 0x05;

        assert_eq!(cpu.registers.b, 0x05);
    }
}
