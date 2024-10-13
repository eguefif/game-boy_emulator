use crate::processor::cpu::Cpu;
use crate::processor::instructions::ArithmeticTarget;

impl Cpu {
    pub fn comp_dispatch(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {}
            ArithmeticTarget::B => _ = self.sub_u8(self.registers.b, false),
            ArithmeticTarget::C => _ = self.sub_u8(self.registers.c, false),
            ArithmeticTarget::D => _ = self.sub_u8(self.registers.d, false),
            ArithmeticTarget::E => _ = self.sub_u8(self.registers.e, false),
            ArithmeticTarget::L => _ = self.sub_u8(self.registers.l, false),
            ArithmeticTarget::H => _ = self.sub_u8(self.registers.h, false),
            ArithmeticTarget::HL => {
                let position = self.registers.hl() as usize;
                let addend = self.memory.fetch_byte(position);
                _ = self.sub_u8(addend, false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_not_set_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0xB8, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x1;
        cpu.registers.b = 0x2;
        cpu.run();
        assert!(!cpu.registers.f.is_zero());
    }

    #[test]
    fn it_should_set_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0xB8, 0);
        cpu.memory.set_byte(0x00, 1);
        cpu.registers.a = 0x1;
        cpu.registers.b = 0x1;
        cpu.run();
        assert!(cpu.registers.f.is_zero());
    }
}
