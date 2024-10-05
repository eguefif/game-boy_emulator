pub enum Instruction {
    Add(ArithmeticTarget),
    End,
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
            0x00 => Some(Instruction::End),
            0x80 => Some(Instruction::Add(ArithmeticTarget::B)),
            _ => None,
        }
    }
}
