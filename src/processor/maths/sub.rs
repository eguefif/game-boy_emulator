use crate::processor::cpu::Cpu;
use crate::processor::instructions::ArithmeticTarget;

impl Cpu {
    pub fn sub_dispatch(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {}
            ArithmeticTarget::B => self.registers.a = self.sub_u8(self.registers.b, false),
            ArithmeticTarget::C => self.registers.a = self.sub_u8(self.registers.c, false),
            ArithmeticTarget::D => self.registers.a = self.sub_u8(self.registers.d, false),
            ArithmeticTarget::E => self.registers.a = self.sub_u8(self.registers.e, false),
            ArithmeticTarget::L => self.registers.a = self.sub_u8(self.registers.l, false),
            ArithmeticTarget::H => self.registers.a = self.sub_u8(self.registers.h, false),
            ArithmeticTarget::HL => {
                let position = self.registers.hl() as usize;
                let addend = self.memory.fetch_byte(position);
                self.registers.a = self.sub_u8(addend, false)
            }
        }
    }

    pub fn subc_dispatch(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {}
            ArithmeticTarget::B => self.registers.a = self.sub_u8(self.registers.b, true),
            ArithmeticTarget::C => self.registers.a = self.sub_u8(self.registers.c, true),
            ArithmeticTarget::D => self.registers.a = self.sub_u8(self.registers.d, true),
            ArithmeticTarget::E => self.registers.a = self.sub_u8(self.registers.e, true),
            ArithmeticTarget::L => self.registers.a = self.sub_u8(self.registers.l, true),
            ArithmeticTarget::H => self.registers.a = self.sub_u8(self.registers.h, true),
            ArithmeticTarget::HL => {
                let position = self.registers.hl() as usize;
                let addend = self.memory.fetch_byte(position);
                self.registers.a = self.sub_u8(addend, true)
            }
        }
    }

    pub fn sub_u8(&mut self, to_sub: u8, carry: bool) -> u8 {
        let carry_value = if carry && self.registers.f.is_carry() {
            1
        } else {
            0
        };
        let result = self
            .registers
            .a
            .wrapping_sub(carry_value)
            .wrapping_sub(to_sub);
        self.set_flags_u8_sub(result, self.registers.a, to_sub);
        result
    }

    fn set_flags_u8_sub(&mut self, result: u8, v1: u8, v2: u8) {
        self.registers.f.set_n();
        if result == 0 {
            self.registers.f.set_zero();
        } else {
            self.registers.f.unset_zero();
        }
        if underflow_u8(v1, v2, self.registers.f.is_carry()) {
            self.registers.f.set_carry();
        } else {
            self.registers.f.unset_carry();
        }
        if half_underflow_u8(v1, v2, self.registers.f.is_carry()) {
            self.registers.f.set_h();
        } else {
            self.registers.f.unset_h();
        }
    }
}

fn underflow_u8(v1: u8, v2: u8, carry: bool) -> bool {
    (v1 as u16) < (v2 as u16 - carry as u16)
}

fn half_underflow_u8(v1: u8, v2: u8, carry: bool) -> bool {
    let first_nibble_v1 = v1 & 0xF;
    let first_nibble_v2 = v2 & 0xF;
    first_nibble_v1 < (first_nibble_v2 - carry as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    // sub no carry
    #[test]
    fn it_subs_b_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x90, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x03;
        cpu.registers.b = 0x02;
        cpu.run();
        assert_eq!(cpu.registers.a, 0x03 - 0x02);
    }

    // sub with carry
    #[test]
    fn it_subs_b_to_a_with_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x98, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.f.set_carry();
        cpu.registers.a = 0x05;
        cpu.registers.b = 0x02;
        cpu.run();
        assert_eq!(cpu.registers.a, 0x05 - 0x02 - 1);
    }

    // sub with carry
    #[test]
    fn it_subs_b_to_a_and_set_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x98, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x0;
        cpu.registers.b = 0x1;
        cpu.run();
        assert!(cpu.registers.f.is_carry());
    }
    // sub with half carry
    #[test]
    fn it_subs_b_to_a_and_set_half_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x98, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x10;
        cpu.registers.b = 0x01;
        cpu.run();
        assert!(cpu.registers.f.is_half_carry());
    }

    // Test sub from memory
    #[test]
    fn it_subs_hl_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x96, 0);
        cpu.memory.set_byte(0x00, 1);

        cpu.memory.set_byte(0x03, 0x2);
        cpu.registers.set_hl(0x02);
        cpu.registers.a = 0x04;
        cpu.run();
        assert_eq!(cpu.registers.a, 0x04 - 0x03);
    }
}
