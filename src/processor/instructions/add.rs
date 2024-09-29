use crate::processor::{
    cpu::Cpu,
    registers::{set_carry, set_h, set_zero},
};

impl Cpu {
    pub fn add_dispatch(self: &mut Cpu, opcode: u8) {
        let low_opcode = opcode & 0xf;
        match low_opcode {
            0 => add(
                &mut self.registers.a,
                self.registers.b,
                &mut self.registers.f,
            ),
            1 => add(
                &mut self.registers.a,
                self.registers.c,
                &mut self.registers.f,
            ),
            2 => add(
                &mut self.registers.a,
                self.registers.d,
                &mut self.registers.f,
            ),
            3 => add(
                &mut self.registers.a,
                self.registers.e,
                &mut self.registers.f,
            ),
            4 => add(
                &mut self.registers.a,
                self.registers.h,
                &mut self.registers.f,
            ),
            5 => add(
                &mut self.registers.a,
                self.registers.l,
                &mut self.registers.f,
            ),
            6 => {
                let hl_low_nibble = (self.registers.hl() & 0xF) as u8;
                add(&mut self.registers.a, hl_low_nibble, &mut self.registers.f);
            }
            7 => {
                let a_value = self.registers.a;
                add(&mut self.registers.a, a_value, &mut self.registers.f);
            }
            _ => eprintln!("Opcode {:x?} not implemented.", opcode),
        }
    }
}
fn add(target: &mut u8, value: u8, flags: &mut u8) {
    let (value, overflow) = target.overflowing_add(value);
    *target = value;
    if value == 0 {
        *flags = set_zero(*flags);
    }
    if overflow {
        *flags = set_carry(*flags);
    }
    if half_overflow(*target, value) {
        *flags = set_h(*flags);
    }
}

fn half_overflow(target: u8, value: u8) -> bool {
    let low_target_nibble = target & 0xF0;
    let low_value_nibble = value & 0xF0;
    low_target_nibble + low_value_nibble > 0xF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_add_with_no_overflow_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x1;
        cpu.registers.b = 0x1;
        cpu.add_dispatch(0x80);

        assert_eq!(cpu.registers.a, 0x1 + 0x1);
        assert_eq!(cpu.registers.f, 0x0);
    }

    #[test]
    fn it_add_with_overflow_and_set_carry_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        cpu.registers.b = 0x1;
        cpu.add_dispatch(0x80);

        assert_eq!(cpu.registers.a, 0x0);
        assert_eq!(cpu.registers.f, 0b1001000);
    }
    #[test]
    fn it_add_with_overflow_and_set_carry_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xFF;
        cpu.registers.b = 0x2;
        cpu.add_dispatch(0x80);

        assert_eq!(cpu.registers.a, 0x1);
        assert_eq!(cpu.registers.f, 0b0001000);
    }

    #[test]
    fn it_add_with_low_overflow_and_set_half_carry_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0xF;
        cpu.registers.b = 0x1;
        cpu.add_dispatch(0x80);

        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.registers.f, 0b0010000);
    }
    #[test]
    fn it_add_nothing_and_set_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x0;
        cpu.registers.b = 0x0;
        cpu.add_dispatch(0x80);

        assert_eq!(cpu.registers.a, 0x0);
        assert_eq!(cpu.registers.f, 0b1000000);
    }
    #[test]
    fn it_add_hl_to_a() {
        let mut cpu = Cpu::new();
        cpu.registers.a = 0x1;
        cpu.registers.b = 0x0;
        cpu.registers.h = 0x5;
        cpu.registers.l = 0x8;
        cpu.add_dispatch(0x86);

        assert_eq!(cpu.registers.a, 0xf9);
        assert_eq!(cpu.registers.f, 0b0000000);
    }
}
