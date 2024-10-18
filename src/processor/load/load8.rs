use crate::processor::cpu::Cpu;
use crate::processor::instructions::TargetLd8;

impl Cpu {
    pub fn load8_dispatch(&mut self, target: TargetLd8) {
        match target {
            TargetLd8::A => self.registers.a = self.memory.fetch_byte(0),
            TargetLd8::B => self.registers.b = self.memory.fetch_byte(0),
            TargetLd8::C => self.registers.c = self.memory.fetch_byte(0),
            TargetLd8::D => self.registers.d = self.memory.fetch_byte(0),
            TargetLd8::E => self.registers.e = self.memory.fetch_byte(0),
            TargetLd8::H => self.registers.h = self.memory.fetch_byte(0),
            TargetLd8::L => self.registers.l = self.memory.fetch_byte(0),
            TargetLd8::HL => {
                let position = self.registers.hl() as usize;
                let value = self.memory.fetch_byte(0);
                self.memory.set_byte(value, position);
            }
            TargetLd8::AHLp => {
                let position = self.registers.hl() as usize;
                let value = self.memory.fetch_byte(position);
                self.registers.set_hl((position + 1) as u16);
                self.registers.a = value;
            }
            TargetLd8::AHLm => {
                let position = self.registers.hl() as usize;
                let value = self.memory.fetch_byte(position);
                self.registers.set_hl((position - 1) as u16);
                self.registers.a = value;
            }
            TargetLd8::Abc => {
                let position = self.registers.bc() as usize;
                let value = self.memory.fetch_byte(position);
                self.registers.a = value;
            }
            TargetLd8::Ade => {
                let position = self.registers.de() as usize;
                let value = self.memory.fetch_byte(position);
                self.registers.a = value;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_move_bc_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x0A, 0);
        cpu.registers.set_bc(0x5);
        cpu.memory.set_byte(0x13, 5);

        cpu.run();
        assert_eq!(cpu.registers.a, 0x13);
    }

    #[test]
    fn it_should_move_de_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x1A, 0);
        cpu.registers.set_de(0x5);
        cpu.memory.set_byte(0x13, 5);

        cpu.run();
        assert_eq!(cpu.registers.a, 0x13);
    }

    #[test]
    fn it_should_move_hlp_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x2A, 0);
        cpu.registers.set_hl(0x5);
        cpu.memory.set_byte(0x13, 5);

        cpu.run();
        assert_eq!(cpu.registers.a, 0x13);
        assert_eq!(cpu.registers.hl(), 0x6);
    }

    #[test]
    fn it_should_move_hlm_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x3A, 0);
        cpu.registers.set_hl(0x5);
        cpu.memory.set_byte(0x13, 5);

        cpu.run();
        assert_eq!(cpu.registers.a, 0x13);
        assert_eq!(cpu.registers.hl(), 0x4);
    }

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
    fn it_should_move_d8_to_c() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x0E, 0);
        cpu.memory.set_byte(0x13, 1);

        cpu.run();
        assert_eq!(cpu.registers.c, 0x13);
    }

    #[test]
    fn it_should_move_d8_to_e() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x1E, 0);
        cpu.memory.set_byte(0x13, 1);

        cpu.run();
        assert_eq!(cpu.registers.e, 0x13);
    }

    #[test]
    fn it_should_move_d8_to_l() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x2E, 0);
        cpu.memory.set_byte(0x13, 1);

        cpu.run();
        assert_eq!(cpu.registers.l, 0x13);
    }

    #[test]
    fn it_should_move_d8_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x3E, 0);
        cpu.memory.set_byte(0x13, 1);

        cpu.run();
        assert_eq!(cpu.registers.a, 0x13);
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
