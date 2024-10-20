use crate::processor::cpu::Cpu;
use crate::processor::instructions::ArithmeticTarget;

impl Cpu {
    pub fn add_dispatch(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {}
            ArithmeticTarget::B => self.registers.a = self.add_u8(self.registers.b, false),
            ArithmeticTarget::C => self.registers.a = self.add_u8(self.registers.c, false),
            ArithmeticTarget::D => self.registers.a = self.add_u8(self.registers.d, false),
            ArithmeticTarget::E => self.registers.a = self.add_u8(self.registers.e, false),
            ArithmeticTarget::L => self.registers.a = self.add_u8(self.registers.l, false),
            ArithmeticTarget::H => self.registers.a = self.add_u8(self.registers.h, false),
            ArithmeticTarget::HL => {
                let position = self.registers.hl() as usize;
                let addend = self.memory.fetch_byte_at(position);
                self.registers.a = self.add_u8(addend, false)
            }
        }
    }

    pub fn addc_dispatch(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {}
            ArithmeticTarget::B => self.registers.a = self.add_u8(self.registers.b, true),
            ArithmeticTarget::C => self.registers.a = self.add_u8(self.registers.c, true),
            ArithmeticTarget::D => self.registers.a = self.add_u8(self.registers.d, true),
            ArithmeticTarget::E => self.registers.a = self.add_u8(self.registers.e, true),
            ArithmeticTarget::L => self.registers.a = self.add_u8(self.registers.l, true),
            ArithmeticTarget::H => self.registers.a = self.add_u8(self.registers.h, true),
            ArithmeticTarget::HL => {
                let position = self.registers.hl() as usize;
                let addend = self.memory.fetch_byte_at(position);
                self.registers.a = self.add_u8(addend, true)
            }
        }
    }

    pub fn add_u8(&mut self, to_add: u8, carry: bool) -> u8 {
        let carry_value = if carry && self.registers.f.is_carry() {
            1
        } else {
            0
        };
        let (tmp, overflow1) = self.registers.a.overflowing_add(carry_value);
        let (result, overflow2) = tmp.overflowing_add(to_add);
        self.set_flags_u8(result, self.registers.a, to_add, overflow1 || overflow2);
        result
    }

    fn set_flags_u8(&mut self, result: u8, v1: u8, v2: u8, overflow: bool) {
        self.registers.f.unset_n();
        if result == 0 {
            self.registers.f.set_zero();
        } else {
            self.registers.f.unset_zero();
        }
        if overflow {
            self.registers.f.set_carry();
        } else {
            self.registers.f.unset_carry();
        }
        if half_overflow_u8(v1, v2) {
            self.registers.f.set_h();
        } else {
            self.registers.f.unset_h();
        }
    }
}

pub fn half_overflow_u8(target: u8, value: u8) -> bool {
    let low_target_nibble = target & 0xF;
    let low_value_nibble = value & 0xF;
    ((low_target_nibble + low_value_nibble) & 0x10) == 0x10
}

#[allow(dead_code)]
pub fn half_overflow_u16(target: u16, value: u16) -> bool {
    let low_target_nibble = target & 0xFFF;
    let low_value_nibble = value & 0xFFF;
    ((low_target_nibble + low_value_nibble) & 0x1000) == 0x1000
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_check_half_overflow_u8_true() {
        let a = 0xf;
        let b = 0x1;

        let res = half_overflow_u8(a, b);
        assert!(res);
    }

    #[test]
    fn it_check_half_overflow_u8_false() {
        let a = 0xe;
        let b = 0x1;

        let res = half_overflow_u8(a, b);
        assert!(!res);
    }

    #[test]
    fn it_check_half_overflow_u16_true() {
        let a = 0xfff;
        let b = 0x1;

        let res = half_overflow_u16(a, b);
        assert!(res);
    }

    #[test]
    fn it_check_half_overflow_u16_false() {
        let a = 0xffe;
        let b = 0x1;

        let res = half_overflow_u16(a, b);
        assert!(!res);
    }

    // Addc u8 test
    #[test]
    fn it_addc_u8() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x1;
        cpu.registers.f.set_carry();

        let res = cpu.add_u8(0x2, true);
        assert_eq!(res, 0x1 + 0x2 + 1);
    }

    // Add u8 tests
    #[test]
    fn it_add_u8_with_no_overflow_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x1;
        let res = cpu.add_u8(0x2, false);

        assert_eq!(res, 0x1 + 0x2);
        assert!(!cpu.registers.f.is_flag());
        assert!(!cpu.registers.f.is_n());
    }

    #[test]
    fn it_add_u8_with_overflow_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        let res = cpu.add_u8(0x1, false);

        assert_eq!(res, 0x0);
        assert!(cpu.registers.f.is_carry());
        assert!(cpu.registers.f.is_zero());
    }

    #[test]
    fn it_add_u8_with_half_overflow_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xF;
        let res = cpu.add_u8(0x1, false);

        assert_eq!(res, 0xF + 0x1);
        assert!(cpu.registers.f.is_half_carry());
    }

    #[test]
    fn it_add_u8_with_no_half_overflow_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xE;
        cpu.add_u8(0x1, false);

        assert!(!cpu.registers.f.is_half_carry());
    }

    // opcode addition tests: ADD
    #[test]
    fn it_adds_b_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x80, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.b = 0x02;
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x02);
    }

    #[test]
    fn it_adds_c_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x81, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.c = 0x02;
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x02);
    }

    #[test]
    fn it_adds_d_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x82, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.d = 0x02;
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x02);
    }

    #[test]
    fn it_adds_e_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x83, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.e = 0x02;
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x02);
    }

    #[test]
    fn it_adds_h_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x84, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.h = 0x02;
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x02);
    }

    #[test]
    fn it_adds_l_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x85, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.l = 0x02;
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x02);
    }

    // opcode addition tests: ADDC
    #[test]
    fn it_adds_b_to_a_with_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x88, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.b = 0x02;
        cpu.registers.f.set_carry();
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x02 + 0x1);
    }

    #[test]
    fn it_adds_c_to_a_with_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x89, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.c = 0x02;
        cpu.registers.f.set_carry();
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x02 + 0x1);
    }

    #[test]
    fn it_adds_d_to_a_with_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x8A, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.d = 0x02;
        cpu.registers.f.set_carry();
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x02 + 0x1);
    }

    #[test]
    fn it_adds_e_to_a_with_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x8B, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.e = 0x02;
        cpu.registers.f.set_carry();
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x02 + 0x1);
    }

    #[test]
    fn it_adds_h_to_a_with_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x8C, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.h = 0x02;
        cpu.registers.f.set_carry();
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x02 + 0x1);
    }

    #[test]
    fn it_adds_l_to_a_with_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x8D, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.l = 0x02;
        cpu.registers.f.set_carry();
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x02 + 0x1);
    }

    // Test add from memory
    #[test]
    fn it_adds_hl_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x86, 0);
        cpu.memory.set_byte(0x00, 1);

        cpu.memory.set_byte(0x04, 0x2);
        cpu.registers.set_hl(0x02);
        cpu.registers.a = 0x03;
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x04);
    }

    #[test]
    fn it_adds_hl_to_a_with_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x8E, 0);
        cpu.memory.set_byte(0x04, 0x2);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.set_hl(0x02);
        cpu.registers.a = 0x03;
        cpu.registers.f.set_carry();
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 + 0x04 + 0x1);
    }
}
