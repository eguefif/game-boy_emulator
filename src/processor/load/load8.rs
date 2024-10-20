use crate::processor::cpu::Cpu;
use crate::processor::instructions::TargetLd8;

impl Cpu {
    pub fn load8_dispatch(&mut self, target: TargetLd8) {
        match target {
            TargetLd8::A => self.registers.a = self.memory.fetch_next_byte(),
            TargetLd8::B => self.registers.b = self.memory.fetch_next_byte(),
            TargetLd8::C => self.registers.c = self.memory.fetch_next_byte(),
            TargetLd8::D => self.registers.d = self.memory.fetch_next_byte(),
            TargetLd8::E => self.registers.e = self.memory.fetch_next_byte(),
            TargetLd8::H => self.registers.h = self.memory.fetch_next_byte(),
            TargetLd8::L => self.registers.l = self.memory.fetch_next_byte(),
            TargetLd8::AA8 => {
                let position_a8 = self.memory.fetch_next_byte();
                let position: usize = (0xFF << 8) | position_a8 as usize;
                self.registers.a = self.memory.fetch_byte_at(position);
            }
            TargetLd8::A8A => {
                let position_a8 = self.memory.fetch_next_byte();
                let position: usize = (0xFF << 8) | position_a8 as usize;
                self.memory.set_byte(self.registers.a, position);
            }
            TargetLd8::AC8 => {
                let position: usize = (0xFF << 8) | self.registers.c as usize;
                self.registers.a = self.memory.fetch_byte_at(position);
            }
            TargetLd8::C8A => {
                let position: usize = (0xFF << 8) | self.registers.c as usize;
                self.memory.set_byte(self.registers.a, position);
            }
            TargetLd8::HL => {
                let position = self.registers.hl() as usize;
                let value = self.memory.fetch_next_byte();
                self.memory.set_byte(value, position);
            }
            TargetLd8::AHLp => {
                let position = self.registers.hl() as usize;
                let value = self.memory.fetch_byte_at(position);
                self.registers.set_hl((position + 1) as u16);
                self.registers.a = value;
            }
            TargetLd8::AHLm => {
                let position = self.registers.hl() as usize;
                let value = self.memory.fetch_byte_at(position);
                self.registers.set_hl((position - 1) as u16);
                self.registers.a = value;
            }
            TargetLd8::Abc => {
                let position = self.registers.bc() as usize;
                let value = self.memory.fetch_byte_at(position);
                self.registers.a = value;
            }
            TargetLd8::Ade => {
                let position = self.registers.de() as usize;
                let value = self.memory.fetch_byte_at(position);
                self.registers.a = value;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_move_a_to_c8() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0xE2, 0);
        cpu.registers.a = 0x13;
        cpu.registers.c = 0x5;

        cpu.run();
        assert_eq!(cpu.memory.fetch_byte_at(0xFF05), 0x13);
    }

    #[test]
    fn it_should_move_c8_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0xF2, 0);
        cpu.registers.c = 0x5;
        cpu.memory.set_byte(0x13, 0xFF05);

        cpu.run();
        assert_eq!(cpu.registers.a, 0x13);
    }

    #[test]
    fn it_should_move_a_to_a8() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0xE0, 0);
        cpu.memory.set_byte(0x5, 1);
        cpu.registers.a = 0x13;

        cpu.run();
        assert_eq!(cpu.memory.fetch_byte_at(0xFF05), 0x13);
    }

    #[test]
    fn it_should_move_a8_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0xF0, 0);
        cpu.memory.set_byte(0x5, 1);
        cpu.memory.set_byte(0x13, 0xFF05);

        cpu.run();
        assert_eq!(cpu.registers.a, 0x13);
    }

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
        assert_eq!(cpu.memory.fetch_byte_at(0x5), 0x13);
    }
}
