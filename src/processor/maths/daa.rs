use crate::processor::cpu::Cpu;

impl Cpu {
    pub fn daa(&mut self) {
        let a = self.registers.a;
        let mut adjust = 0;
        adjust += handle_units(a, self.registers.f.is_half_carry());
        adjust += handle_tens(a, self.registers.f.is_carry());
        self.registers.a = a.wrapping_add(adjust);
    }
}

fn handle_units(a: u8, half_carry: bool) -> u8 {
    let mut retval = 0;
    if (a & 0xF) > 9 || half_carry {
        retval = 0x6;
    }
    retval
}

fn handle_tens(a: u8, carry: bool) -> u8 {
    let mut retval = 0;
    if a > 99 || carry {
        retval = 0x60;
    }
    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_adjust_unit_when_greater_than_9() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x27, 0x0);
        cpu.registers.a = 0x2E;

        cpu.run();
        assert_eq!(cpu.registers.a, 0x2E + 0x6)
    }

    #[test]
    fn it_should_adjust_unit_when_half_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x27, 0x0);
        cpu.registers.a = 0x22;
        cpu.registers.f.set_h();

        cpu.run();
        assert_eq!(cpu.registers.a, 0x22 + 0x6)
    }

    #[test]
    fn it_should_adjust_tens_when_greater_than_99() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x27, 0x0);
        cpu.registers.a = 0xA4;

        cpu.run();
        assert_eq!(cpu.registers.a, 0x4)
    }

    #[test]
    fn it_should_adjust_tens_when_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x27, 0x0);
        cpu.registers.a = 0x22;
        cpu.registers.f.set_carry();

        cpu.run();
        assert_eq!(cpu.registers.a, 0x22 + 0x60)
    }

    #[test]
    fn it_should_adjust_tens_and_unit_when_carry_and_half_carry() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x27, 0x0);
        cpu.registers.a = 0x22;
        cpu.registers.f.set_carry();
        cpu.registers.f.set_h();

        cpu.run();
        assert_eq!(cpu.registers.a, 0x22 + 0x66)
    }

    #[test]
    fn it_should_adjust_tens_and_unit_when_higher_than_99() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x27, 0x0);
        cpu.registers.a = 0xAA;

        cpu.run();
        assert_eq!(cpu.registers.a, 0x10)
    }
}
