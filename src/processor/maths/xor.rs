use crate::processor::cpu::Cpu;
use crate::processor::instructions::ArithmeticTarget;

impl Cpu {
    pub fn xor_dispatch(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {}
            ArithmeticTarget::B => self.registers.a ^= self.registers.b,
            ArithmeticTarget::C => self.registers.a ^= self.registers.c,
            ArithmeticTarget::D => self.registers.a ^= self.registers.d,
            ArithmeticTarget::E => self.registers.a ^= self.registers.e,
            ArithmeticTarget::H => self.registers.a ^= self.registers.h,
            ArithmeticTarget::L => self.registers.a ^= self.registers.l,
            ArithmeticTarget::HL => {
                let position = self.registers.hl() as usize;
                let value = self.memory.fetch_byte_at(position);
                self.registers.a ^= value;
            }
        }
        if self.registers.a == 0 {
            self.registers.f.set_zero();
        }
        self.registers.f.set_h();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_do_xor_with_b() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0xA8, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.b = 36;
        cpu.registers.a = 13;
        cpu.run();
        assert_eq!(cpu.registers.a, 41);
    }

    #[test]
    fn it_do_xor_with_hl() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0xAE, 0);
        cpu.memory.set_byte(36, 0x2);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.set_hl(0x02);
        cpu.registers.a = 13;
        cpu.run();
        assert_eq!(cpu.registers.a, 41);
    }
}
