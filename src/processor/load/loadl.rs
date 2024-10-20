use crate::processor::cpu::Cpu;
use crate::processor::instructions::Target;

impl Cpu {
    pub fn loadl_dispatch(&mut self, target: Target) {
        match target {
            Target::A => self.registers.l = self.registers.a,
            Target::B => self.registers.l = self.registers.b,
            Target::C => self.registers.l = self.registers.c,
            Target::D => self.registers.l = self.registers.d,
            Target::E => self.registers.l = self.registers.e,
            Target::H => self.registers.l = self.registers.h,
            Target::L => {}
            Target::HL => {
                let position = self.registers.hl() as usize;
                self.registers.l = self.memory.fetch_byte_at(position);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_move_b_to_l() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x13;
        cpu.memory.set_byte(0x68, 0);

        cpu.run();

        assert_eq!(cpu.registers.l, 0x13);
    }

    #[test]
    fn it_should_move_a_to_l() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x13;
        cpu.memory.set_byte(0x6F, 0);

        cpu.run();

        assert_eq!(cpu.registers.l, 0x13);
    }

    #[test]
    fn it_should_move_c_to_l() {
        let mut cpu = Cpu::new();
        cpu.registers.c = 0x13;
        cpu.memory.set_byte(0x69, 0);

        cpu.run();

        assert_eq!(cpu.registers.l, 0x13);
    }

    #[test]
    fn it_should_move_d_to_l() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x13;
        cpu.memory.set_byte(0x6A, 0);

        cpu.run();

        assert_eq!(cpu.registers.l, 0x13);
    }

    #[test]
    fn it_should_move_e_to_l() {
        let mut cpu = Cpu::new();
        cpu.registers.e = 0x13;
        cpu.memory.set_byte(0x6B, 0);

        cpu.run();

        assert_eq!(cpu.registers.l, 0x13);
    }

    #[test]
    fn it_should_move_h_to_l() {
        let mut cpu = Cpu::new();
        cpu.registers.h = 0x13;
        cpu.memory.set_byte(0x6C, 0);

        cpu.run();

        assert_eq!(cpu.registers.l, 0x13);
    }

    #[test]
    fn it_should_move_l_to_l() {
        let mut cpu = Cpu::new();
        cpu.registers.l = 0x13;
        cpu.memory.set_byte(0x6D, 0);

        cpu.run();

        assert_eq!(cpu.registers.l, 0x13);
    }

    #[test]
    fn it_should_move_hl_to_l() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x6E, 0);

        cpu.registers.set_hl(0x2);
        cpu.memory.set_byte(0xFF, 0x2);

        cpu.run();

        assert_eq!(cpu.registers.l, 0xFF);
    }
}
