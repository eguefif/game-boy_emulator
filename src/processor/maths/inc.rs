use crate::processor::cpu::Cpu;
use crate::processor::instructions::IncTarget;

use super::add::half_overflow_u8;

impl Cpu {
    pub fn inc_dispatch(&mut self, target: IncTarget) {
        match target {
            IncTarget::A => self.registers.a = self.inc(self.registers.a),
            IncTarget::B => self.registers.b = self.inc(self.registers.b),
            IncTarget::C => self.registers.c = self.inc(self.registers.c),
            IncTarget::D => self.registers.d = self.inc(self.registers.d),
            IncTarget::E => self.registers.e = self.inc(self.registers.e),
            IncTarget::H => self.registers.h = self.inc(self.registers.h),
            IncTarget::L => self.registers.l = self.inc(self.registers.l),
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

        cpu.run();

        assert_eq!(cpu.registers.b, 0x06);
        assert!(!cpu.registers.f.is_n());
    }

    #[test]
    fn it_should_inc_register_b_z_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x04, 0);

        cpu.registers.b = 0xFF;

        cpu.run();

        assert!(cpu.registers.f.is_zero());
    }

    #[test]
    fn it_should_inc_register_b_h_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x04, 0);

        cpu.registers.b = 0xF;

        cpu.run();

        assert!(cpu.registers.f.is_half_carry());
    }

    #[test]
    fn it_should_inc_register_b_no_h_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x04, 0);

        cpu.registers.b = 0xE;

        cpu.run();

        assert!(!cpu.registers.f.is_half_carry());
    }

    #[test]
    fn it_should_inc_register_d_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x14, 0);

        cpu.registers.d = 0x4;

        cpu.run();

        assert_eq!(cpu.registers.d, 0x5);
    }

    #[test]
    fn it_should_inc_register_h_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x24, 0);

        cpu.registers.h = 0x4;

        cpu.run();

        assert_eq!(cpu.registers.h, 0x5);
    }

    #[test]
    fn it_should_inc_hl_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x34, 0);
        cpu.memory.set_byte(0x4, 0xFF);
        cpu.registers.set_hl(0xFF);

        cpu.run();

        assert_eq!(cpu.memory.memory[0xFF], 0x5);
    }

    #[test]
    fn it_should_inc_register_c_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x0C, 0);

        cpu.registers.c = 0x4;

        cpu.run();

        assert_eq!(cpu.registers.c, 0x5);
    }

    #[test]
    fn it_should_inc_register_e_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x1C, 0);

        cpu.registers.e = 0x4;

        cpu.run();

        assert_eq!(cpu.registers.e, 0x5);
    }

    #[test]
    fn it_should_inc_register_l_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x2C, 0);

        cpu.registers.l = 0x4;

        cpu.run();

        assert_eq!(cpu.registers.l, 0x5);
    }

    #[test]
    fn it_should_inc_register_a_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x3C, 0);

        cpu.registers.a = 0x4;

        cpu.run();

        assert_eq!(cpu.registers.a, 0x5);
    }

    #[test]
    fn it_should_inc_register_bc_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x03, 0);

        cpu.registers.set_bc(0x4);

        cpu.run();

        assert_eq!(cpu.registers.bc(), 0x5);
    }

    #[test]
    fn it_should_inc_register_de_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x13, 0);

        cpu.registers.set_de(0x4);

        cpu.run();

        assert_eq!(cpu.registers.de(), 0x5);
    }

    #[test]
    fn it_should_inc_register_hl_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x23, 0);

        cpu.registers.set_hl(0x4);

        cpu.run();

        assert_eq!(cpu.registers.hl(), 0x5);
    }

    #[test]
    fn it_should_inc_register_sp_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x33, 0);

        cpu.registers.sp = 0x04;

        cpu.run();

        assert_eq!(cpu.registers.sp, 0x5);
    }
}
