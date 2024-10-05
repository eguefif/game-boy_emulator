#![allow(dead_code)]
use crate::processor::cpu::Cpu;

impl Cpu {
    pub fn add_u8(&mut self, to_add: u8) -> u8 {
        let (result, overflow) = self.registers.a.overflowing_add(to_add);
        self.set_flags_u8(result, self.registers.a, to_add, overflow, false);
        result
    }

    fn set_flags_u8(&mut self, result: u8, v1: u8, v2: u8, overflow: bool, sub: bool) {
        if result == 0 {
            self.registers.f.set_zero();
        } else {
            self.registers.f.unset_zero();
        }
        if v1 < v2 && sub {
            self.registers.f.set_n();
        } else {
            self.registers.f.unset_n();
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

    #[test]
    fn it_add_with_no_overflow_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x1;
        let res = cpu.add_u8(0x2);

        println!("in test result: {}", res);
        assert_eq!(res, 0x1 + 0x2);
        assert!(!cpu.registers.f.is_flag());
    }
}
