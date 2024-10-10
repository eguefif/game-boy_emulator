use crate::processor::cpu::Cpu;
use crate::processor::instructions::Target;

impl Cpu {
    pub fn loadh_dispatch(&mut self, target: Target) {
        match target {
            Target::A => self.registers.h = self.registers.a,
            Target::B => self.registers.h = self.registers.b,
            Target::C => self.registers.h = self.registers.c,
            Target::D => self.registers.h = self.registers.d,
            Target::E => self.registers.h = self.registers.e,
            Target::H => {}
            Target::L => self.registers.h = self.registers.l,
            Target::HL => {
                let position = self.registers.hl() as usize;
                self.registers.h = self.memory.fetch_byte(position);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_move_b_to_h() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x13;
        cpu.memory.set_byte(0x60, 0);

        cpu.run();

        assert_eq!(cpu.registers.h, 0x13);
    }

    #[test]
    fn it_should_move_a_to_h() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x13;
        cpu.memory.set_byte(0x67, 0);

        cpu.run();

        assert_eq!(cpu.registers.h, 0x13);
    }

    #[test]
    fn it_should_move_c_to_h() {
        let mut cpu = Cpu::new();
        cpu.registers.c = 0x13;
        cpu.memory.set_byte(0x61, 0);

        cpu.run();

        assert_eq!(cpu.registers.h, 0x13);
    }

    #[test]
    fn it_should_move_d_to_h() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x13;
        cpu.memory.set_byte(0x62, 0);

        cpu.run();

        assert_eq!(cpu.registers.h, 0x13);
    }

    #[test]
    fn it_should_move_e_to_h() {
        let mut cpu = Cpu::new();
        cpu.registers.e = 0x13;
        cpu.memory.set_byte(0x63, 0);

        cpu.run();

        assert_eq!(cpu.registers.h, 0x13);
    }

    #[test]
    fn it_should_move_h_to_h() {
        let mut cpu = Cpu::new();
        cpu.registers.h = 0x13;
        cpu.memory.set_byte(0x64, 0);

        cpu.run();

        assert_eq!(cpu.registers.h, 0x13);
    }

    #[test]
    fn it_should_move_l_to_h() {
        let mut cpu = Cpu::new();
        cpu.registers.l = 0x13;
        cpu.memory.set_byte(0x65, 0);

        cpu.run();

        assert_eq!(cpu.registers.h, 0x13);
    }

    #[test]
    fn it_should_move_hl_to_h() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x66, 0);

        cpu.registers.set_hl(0x2);
        cpu.memory.set_byte(0xFF, 0x2);

        cpu.run();

        assert_eq!(cpu.registers.h, 0xFF);
    }
}
