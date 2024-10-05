/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_increment_with_no_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 5;

        cpu.range_0_dispatch(0x04);

        assert_eq!(cpu.registers.b, 6);
        assert_eq!(cpu.registers.f, 0b0000_0000);
    }

    #[test]
    fn it_increment_with_carry() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0xff;

        cpu.range_0_dispatch(0x04);

        assert_eq!(cpu.registers.b, 0);
        assert_eq!(cpu.registers.f, 0b1011_0000);
    }

    #[test]
    fn it_decrement() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 5;

        cpu.range_0_dispatch(0x05);

        assert_eq!(cpu.registers.b, 4);
        assert_eq!(cpu.registers.f, 0b0100_0000);
    }

    #[test]
    fn it_decrement_to_zero() {
        let mut cpu = Cpu::new();
        cpu.registers.b = 0x1;

        cpu.range_0_dispatch(0x05);

        assert_eq!(cpu.registers.b, 0);
        assert_eq!(cpu.registers.f, 0b1100_0000);
    }
}
*/
