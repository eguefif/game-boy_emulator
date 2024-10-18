use crate::processor::cpu::Cpu;
use crate::processor::instructions::Load16Target;

impl Cpu {
    pub fn load16_dispatch(&mut self, target: Load16Target) {
        match target {
            Load16Target::BC => self.registers.set_bc(self.memory.fetch_next_word()),
            Load16Target::DE => self.registers.set_de(self.memory.fetch_next_word()),
            Load16Target::HL => self.registers.set_hl(self.memory.fetch_next_word()),
            Load16Target::SP => self.registers.set_sp(self.memory.fetch_next_word()),
            Load16Target::A16A =>{
                let position = self.memory.fetch_next_word();
                let value: u8 = self.registers.a;
                self.memory.set_byte(value, position as usize);
            }
            Load16Target::AA16 =>{
                let position = self.memory.fetch_next_word();
                let value: u8 = self.memory.fetch_byte(position as usize);
                self.registers.a = value;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_load_16_from_memory_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0xFA, 0);
        cpu.memory.set_byte(0xFF, 0x1);
        cpu.memory.set_byte(0x55, 0x2);
        cpu.memory.set_byte(0x13, 0x55FF);

        cpu.run();

        assert_eq!(cpu.registers.a, 0x13)
    }
    #[test]
    fn it_should_load_a_from_memory_to_16() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0xEA, 0);
        cpu.memory.set_byte(0x55, 0x1);
        cpu.memory.set_byte(0xFF, 0x2);
        cpu.registers.a = 0x13;

        cpu.run();

        assert_eq!(cpu.memory.fetch_byte(0xFF55), 0x13)
    }

    #[test]
    fn it_should_load_16_from_memory_to_bc() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x01, 0);
        cpu.memory.set_byte(0xaf, 0x1);
        cpu.memory.set_byte(0x23, 0x2);

        cpu.run();

        assert_eq!(cpu.registers.bc(), 0x23af)
    }

    #[test]
    fn it_should_load_16_from_memory_to_de() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x11, 0);
        cpu.memory.set_byte(0xaf, 0x1);
        cpu.memory.set_byte(0x23, 0x2);

        cpu.run();

        assert_eq!(cpu.registers.de(), 0x23af)
    }

    #[test]
    fn it_should_load_16_from_memory_to_hl() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x21, 0);
        cpu.memory.set_byte(0xaf, 0x1);
        cpu.memory.set_byte(0x23, 0x2);

        cpu.run();

        assert_eq!(cpu.registers.hl(), 0x23af)
    }

    #[test]
    fn it_should_load_16_from_memory_to_sp() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x31, 0);
        cpu.memory.set_byte(0xaf, 0x1);
        cpu.memory.set_byte(0x23, 0x2);

        cpu.run();

        assert_eq!(cpu.registers.sp, 0x23af);
    }
}
