use crate::processor::cpu::Cpu;

impl Cpu {
    pub fn scf(&mut self) {
        self.registers.f.set_carry();
        self.registers.f.unset_h();
        self.registers.f.unset_n();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_set_carry_unset_h_n() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x37, 0);

        cpu.run();

        assert!(cpu.registers.f.is_carry());
        assert!(!cpu.registers.f.is_half_carry());
        assert!(!cpu.registers.f.is_n());
    }
}
