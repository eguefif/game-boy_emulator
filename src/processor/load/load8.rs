use crate::processor::cpu::Cpu;
use crate::processor::instructions::Target;

impl Cpu {
    pub fn load8_dispatch(&mut self, target: Target) {
        match target {
            Target::A => {}
            Target::B => self.registers.b = self.memory.fetch_byte(0),
            Target::C => {}
            Target::D => self.registers.d = self.memory.fetch_byte(0),
            Target::E => {}
            Target::H => self.registers.h = self.memory.fetch_byte(0),
            Target::L => {}
            Target::HL => {
                let position = self.registers.hl() as usize;
                let value = self.memory.fetch_byte(0);
                self.memory.set_byte(value, position);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_move_d8_to_b() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x06, 0);
        cpu.memory.set_byte(0x13, 1);

        cpu.run();
        assert_eq!(cpu.registers.b, 0x13);
    }

    #[test]
    fn it_should_move_d8_to_d() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x16, 0);
        cpu.memory.set_byte(0x13, 1);

        cpu.run();
        assert_eq!(cpu.registers.d, 0x13);
    }

    #[test]
    fn it_should_move_d8_to_h() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x26, 0);
        cpu.memory.set_byte(0x13, 1);

        cpu.run();
        assert_eq!(cpu.registers.h, 0x13);
    }

    #[test]
    fn it_should_move_d8_to_hl() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x36, 0);
        cpu.memory.set_byte(0x13, 1);
        cpu.registers.set_hl(0x5);

        cpu.run();
        assert_eq!(cpu.memory.fetch_byte(0x5), 0x13);
    }
}
