use crate::processor::cpu::Cpu;
use crate::processor::instructions::ByteTarget;

impl Cpu {
    pub fn loadbytea_dispatch(&mut self, target: ByteTarget) {
        let position: usize;
        match target {
            ByteTarget::BC => position = self.registers.bc() as usize,
            ByteTarget::DE => position = self.registers.de() as usize,
            ByteTarget::HLp => {
                position = self.registers.hl() as usize;
                self.registers.set_hl((position + 1) as u16);
            }
            ByteTarget::HLm => {
                position = self.registers.hl() as usize;
                self.registers.set_hl((position - 1) as u16);
            }
        }
        let value = self.registers.a;

        self.memory.set_byte(value, position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_load_a_to_bc() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x02, 0);
        cpu.registers.set_bc(0x12);
        cpu.registers.a = 0x13;

        cpu.run();
        assert_eq!(cpu.memory.fetch_byte_at(0x12), 0x13);
    }
    #[test]
    fn it_should_load_a_to_de() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x12, 0);
        cpu.registers.set_de(0x12);
        cpu.registers.a = 0x13;

        cpu.run();
        assert_eq!(cpu.memory.fetch_byte_at(0x12), 0x13);
    }

    #[test]
    fn it_should_load_a_to_hl_and_increment_hl() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x22, 0);
        cpu.registers.set_hl(0x12);
        cpu.registers.a = 0x13;

        cpu.run();
        assert_eq!(cpu.memory.fetch_byte_at(0x12), 0x13);
        assert_eq!(cpu.registers.hl(), 0x12 + 1);
    }

    #[test]
    fn it_should_load_a_to_hl_and_decrement_hl() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x32, 0);
        cpu.registers.set_hl(0x12);
        cpu.registers.a = 0x13;

        cpu.run();
        assert_eq!(cpu.memory.fetch_byte_at(0x12), 0x13);
        assert_eq!(cpu.registers.hl(), 0x12 - 0x1);
    }
}
