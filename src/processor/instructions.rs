pub enum Instruction {
    Add(ArithmeticTarget),
}

#[allow(dead_code)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x80 => Some(Instruction::Add(ArithmeticTarget::B)),
            _ => None,
        }
    }
}
