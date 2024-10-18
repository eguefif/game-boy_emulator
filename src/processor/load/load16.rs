use crate::processor::cpu::Cpu;
use crate::processor::instructions::Load16Target;

impl Cpu {
    pub fn load16_dispatch(&mut self, target: Load16Target) {
        match target {
            Load16Target::BC => self.registers.set_bc(self.memory.fetch_next_word()),
            Load16Target::DE => {}
            Load16Target::HL => {}
            Load16Target::SP => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_load_16_from_memory_to_bc() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x01, 0);
        cpu.memory.set_byte(0xaf, 0x1);
        cpu.memory.set_byte(0x23, 0x2);

        cpu.run();

        assert_eq!(cpu.registers.bc(), 0x23af)
    }
}
