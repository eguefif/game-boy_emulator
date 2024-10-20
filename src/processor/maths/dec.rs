use crate::processor::cpu::Cpu;
use crate::processor::instructions::IncTarget;

use super::sub::half_underflow_u8;

impl Cpu {
    pub fn dec_dispatch(&mut self, target: IncTarget) {
        match target {
            IncTarget::A => self.registers.a = self.dec(self.registers.a),
            IncTarget::B => self.registers.b = self.dec(self.registers.b),
            IncTarget::C => self.registers.c = self.dec(self.registers.c),
            IncTarget::D => self.registers.d = self.dec(self.registers.d),
            IncTarget::E => self.registers.e = self.dec(self.registers.e),
            IncTarget::H => self.registers.h = self.dec(self.registers.h),
            IncTarget::L => self.registers.l = self.dec(self.registers.l),
            IncTarget::BC => {
                let value = self.registers.bc();
                self.registers.set_bc(value.wrapping_sub(1));
            }
            IncTarget::DE => {
                let value = self.registers.de();
                self.registers.set_de(value.wrapping_sub(1));
            }
            IncTarget::SP => {
                let value = self.registers.sp;
                self.registers.sp = value.wrapping_sub(1);
            }
            IncTarget::HL => {
                let value = self.registers.hl();
                self.registers.set_hl(value.wrapping_sub(1));
            }
            IncTarget::HLFlags => {
                let position = self.registers.hl() as usize;
                let value = self.memory.fetch_byte_at(position);
                let result = self.dec(value);
                self.memory.set_byte(result, position);
            }
        }
    }

    fn dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.set_dec_flags(result, value);
        result
    }

    fn set_dec_flags(&mut self, result: u8, value: u8) {
        if result == 0 {
            self.registers.f.set_zero();
        } else {
            self.registers.f.unset_zero();
        }
        self.registers.f.set_n();
        if half_underflow_u8(value, 1, false) {
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
    fn it_should_dec_registers_b_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x05, 0);

        cpu.registers.b = 0x05;

        cpu.run();

        assert_eq!(cpu.registers.b, 0x04);
        assert!(cpu.registers.f.is_n());
    }

    #[test]
    fn it_should_dec_registers_b_z_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x05, 0);

        cpu.registers.b = 0x1;

        cpu.run();

        assert!(cpu.registers.f.is_zero());
    }

    #[test]
    fn it_should_dec_registers_b_h_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x05, 0);

        cpu.registers.b = 0x10;

        cpu.run();

        assert!(cpu.registers.f.is_half_carry());
    }

    #[test]
    fn it_should_dec_registers_b_no_h_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x05, 0);

        cpu.registers.b = 0xF;

        cpu.run();

        assert!(!cpu.registers.f.is_half_carry());
    }

    #[test]
    fn it_should_dec_registers_d_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x15, 0);

        cpu.registers.d = 0x4;

        cpu.run();

        assert_eq!(cpu.registers.d, 0x3);
    }

    #[test]
    fn it_should_dec_registers_h_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x25, 0);

        cpu.registers.h = 0x4;

        cpu.run();

        assert_eq!(cpu.registers.h, 0x3);
    }

    #[test]
    fn it_should_dec_hl_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x35, 0);
        cpu.memory.set_byte(0x4, 0xFF);
        cpu.registers.set_hl(0xFF);

        cpu.run();

        assert_eq!(cpu.memory.memory[0xFF], 0x3);
    }

    #[test]
    fn it_should_dec_registers_c_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x0D, 0);

        cpu.registers.c = 0x4;

        cpu.run();

        assert_eq!(cpu.registers.c, 0x3);
    }

    #[test]
    fn it_should_dec_registers_e_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x1D, 0);

        cpu.registers.e = 0x4;

        cpu.run();

        assert_eq!(cpu.registers.e, 0x3);
    }

    #[test]
    fn it_should_dec_registers_l_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x2D, 0);

        cpu.registers.l = 0x4;

        cpu.run();

        assert_eq!(cpu.registers.l, 0x3);
    }

    #[test]
    fn it_should_dec_registers_a_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x3D, 0);

        cpu.registers.a = 0x4;

        cpu.run();

        assert_eq!(cpu.registers.a, 0x3);
    }

    #[test]
    fn it_should_dec_registers_bc_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x0B, 0);

        cpu.registers.set_bc(0x4);

        cpu.run();

        assert_eq!(cpu.registers.bc(), 0x3);
    }

    #[test]
    fn it_should_dec_registers_de_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x1B, 0);

        cpu.registers.set_de(0x4);

        cpu.run();

        assert_eq!(cpu.registers.de(), 0x3);
    }

    #[test]
    fn it_should_dec_registers_hl_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x2B, 0);

        cpu.registers.set_hl(0x4);

        cpu.run();

        assert_eq!(cpu.registers.hl(), 0x3);
    }

    #[test]
    fn it_should_dec_registers_sp_no_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x3B, 0);

        cpu.registers.sp = 0x04;

        cpu.run();

        assert_eq!(cpu.registers.sp, 0x3);
    }
}
