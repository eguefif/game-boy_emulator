use crate::processor::cpu::Cpu;
use crate::processor::instructions::Target;

impl Cpu {
    pub fn loade_dispatch(&mut self, target: Target) {
        match target {
            Target::A => self.registers.e = self.registers.a,
            Target::B => self.registers.e = self.registers.b,
            Target::C => self.registers.e = self.registers.c,
            Target::D => self.registers.e = self.registers.d,
            Target::E => {}
            Target::H => self.registers.e = self.registers.h,
            Target::L => self.registers.e = self.registers.l,
            Target::HL => {
                let position = self.registers.hl() as usize;
                self.registers.e = self.memory.fetch_byte(position);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_move_b_to_e() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x13;
        cpu.memory.set_byte(0x58, 0);

        cpu.run();

        assert_eq!(cpu.registers.e, 0x13);
    }

    #[test]
    fn it_should_move_a_to_e() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x13;
        cpu.memory.set_byte(0x5F, 0);

        cpu.run();

        assert_eq!(cpu.registers.e, 0x13);
    }

    #[test]
    fn it_should_move_c_to_e() {
        let mut cpu = Cpu::new();
        cpu.registers.c = 0x13;
        cpu.memory.set_byte(0x59, 0);

        cpu.run();

        assert_eq!(cpu.registers.e, 0x13);
    }

    #[test]
    fn it_should_move_d_to_e() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x13;
        cpu.memory.set_byte(0x5A, 0);

        cpu.run();

        assert_eq!(cpu.registers.e, 0x13);
    }

    #[test]
    fn it_should_move_e_to_e() {
        let mut cpu = Cpu::new();
        cpu.registers.e = 0x13;
        cpu.memory.set_byte(0x5B, 0);

        cpu.run();

        assert_eq!(cpu.registers.e, 0x13);
    }

    #[test]
    fn it_should_move_h_to_e() {
        let mut cpu = Cpu::new();
        cpu.registers.h = 0x13;
        cpu.memory.set_byte(0x5C, 0);

        cpu.run();

        assert_eq!(cpu.registers.e, 0x13);
    }

    #[test]
    fn it_should_move_l_to_e() {
        let mut cpu = Cpu::new();
        cpu.registers.l = 0x13;
        cpu.memory.set_byte(0x5D, 0);

        cpu.run();

        assert_eq!(cpu.registers.e, 0x13);
    }

    #[test]
    fn it_should_move_hl_to_e() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x5E, 0);

        cpu.registers.set_hl(0x2);
        cpu.memory.set_byte(0xFF, 0x2);

        cpu.run();

        assert_eq!(cpu.registers.e, 0xFF);
    }
}
