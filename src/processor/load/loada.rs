use crate::processor::cpu::Cpu;
use crate::processor::instructions::Target;

impl Cpu {
    pub fn loada_dispatch(&mut self, target: Target) {
        match target {
            Target::A => {}
            Target::B => self.registers.a = self.registers.b,
            Target::C => self.registers.a = self.registers.c,
            Target::D => self.registers.a = self.registers.d,
            Target::E => self.registers.a = self.registers.e,
            Target::H => self.registers.a = self.registers.h,
            Target::L => self.registers.a = self.registers.l,
            Target::HL => {
                let position = self.registers.hl() as usize;
                self.registers.a = self.memory.fetch_byte_at(position);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_move_b_to_a() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x13;
        cpu.memory.set_byte(0x78, 0);

        cpu.run();

        assert_eq!(cpu.registers.a, 0x13);
    }

    #[test]
    fn it_should_move_a_to_a() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x13;
        cpu.memory.set_byte(0x7F, 0);

        cpu.run();

        assert_eq!(cpu.registers.a, 0x13);
    }

    #[test]
    fn it_should_move_c_to_a() {
        let mut cpu = Cpu::new();
        cpu.registers.c = 0x13;
        cpu.memory.set_byte(0x79, 0);

        cpu.run();

        assert_eq!(cpu.registers.a, 0x13);
    }

    #[test]
    fn it_should_move_d_to_a() {
        let mut cpu = Cpu::new();
        cpu.registers.d = 0x13;
        cpu.memory.set_byte(0x7A, 0);

        cpu.run();

        assert_eq!(cpu.registers.a, 0x13);
    }

    #[test]
    fn it_should_move_e_to_a() {
        let mut cpu = Cpu::new();
        cpu.registers.e = 0x13;
        cpu.memory.set_byte(0x7B, 0);

        cpu.run();

        assert_eq!(cpu.registers.a, 0x13);
    }

    #[test]
    fn it_should_move_h_to_a() {
        let mut cpu = Cpu::new();
        cpu.registers.h = 0x13;
        cpu.memory.set_byte(0x7C, 0);

        cpu.run();

        assert_eq!(cpu.registers.a, 0x13);
    }

    #[test]
    fn it_should_move_l_to_a() {
        let mut cpu = Cpu::new();
        cpu.registers.l = 0x13;
        cpu.memory.set_byte(0x7D, 0);

        cpu.run();

        assert_eq!(cpu.registers.a, 0x13);
    }

    #[test]
    fn it_should_move_hl_to_a() {
        let mut cpu = Cpu::new();
        cpu.memory.set_byte(0x7E, 0);

        cpu.registers.set_hl(0x2);
        cpu.memory.set_byte(0xFF, 0x2);

        cpu.run();

        assert_eq!(cpu.registers.a, 0xFF);
    }
}
