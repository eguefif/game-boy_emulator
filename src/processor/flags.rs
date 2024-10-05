#![allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Flags {
    pub f: u8,
}

impl Flags {
    pub fn new() -> Flags {
        Flags { f: 0 }
    }

    pub fn set_zero(self: &mut Flags) {
        self.f |= 0b1000_0000
    }

    pub fn unset_zero(self: &mut Flags) {
        self.f &= 0b0111_1111;
    }

    pub fn set_carry(self: &mut Flags) {
        self.f |= 0b0001_0000;
    }

    pub fn unset_carry(self: &mut Flags) {
        self.f &= 0b1110_1111;
    }

    pub fn set_n(self: &mut Flags) {
        self.f |= 0b0100_0000;
    }

    pub fn unset_n(self: &mut Flags) {
        self.f &= 0b1011_1111;
    }

    pub fn set_h(self: &mut Flags) {
        self.f |= 0b0010_0000;
    }

    pub fn unset_h(self: &mut Flags) {
        self.f &= 0b1101_1111;
    }

    pub fn is_carry(self: &mut Flags) -> bool {
        (self.f & 0b0001_0000) == 0b0001_0000
    }

    pub fn is_zero(self: &mut Flags) -> bool {
        (self.f & 0b1000_0000) == 0b1000_0000
    }

    pub fn is_half_carry(self: &mut Flags) -> bool {
        (self.f & 0b0010_0000) == 0b0010_0000
    }

    pub fn is_n(self: &mut Flags) -> bool {
        (self.f & 0b0100_0000) == 0b0100_0000
    }

    pub fn is_flag(self: &mut Flags) -> bool {
        (self.f & 0b1111_0000) != 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_zero_flag() {
        let mut flags = Flags::new();
        flags.set_zero();

        assert_eq!(0b1000_0000, flags.f);
    }

    #[test]
    fn unset_zero_flag() {
        let mut flags = Flags::new();
        flags.set_zero();
        flags.unset_zero();

        assert_eq!(0b0000_0000, flags.f);
    }

    #[test]
    fn is_zero_flag() {
        let mut flags = Flags::new();
        flags.set_zero();

        assert!(flags.is_zero());
    }
}
