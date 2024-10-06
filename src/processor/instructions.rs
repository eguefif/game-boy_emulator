pub enum Instruction {
    Add(ArithmeticTarget),
    AddC(ArithmeticTarget),
    LoadB(Target),
    LoadC(Target),
    End,
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

pub enum Target {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::End),
            0x40 => Some(Instruction::LoadB(Target::B)),
            0x41 => Some(Instruction::LoadB(Target::C)),
            0x42 => Some(Instruction::LoadB(Target::D)),
            0x43 => Some(Instruction::LoadB(Target::E)),
            0x44 => Some(Instruction::LoadB(Target::H)),
            0x45 => Some(Instruction::LoadB(Target::L)),
            0x46 => Some(Instruction::LoadB(Target::HL)),
            0x47 => Some(Instruction::LoadB(Target::A)),
            0x48 => Some(Instruction::LoadC(Target::B)),
            0x49 => Some(Instruction::LoadC(Target::C)),
            0x4A => Some(Instruction::LoadC(Target::D)),
            0x4B => Some(Instruction::LoadC(Target::E)),
            0x4C => Some(Instruction::LoadC(Target::H)),
            0x4D => Some(Instruction::LoadC(Target::L)),
            0x4E => Some(Instruction::LoadC(Target::HL)),
            0x4F => Some(Instruction::LoadC(Target::A)),
            0x80 => Some(Instruction::Add(ArithmeticTarget::B)),
            0x81 => Some(Instruction::Add(ArithmeticTarget::C)),
            0x82 => Some(Instruction::Add(ArithmeticTarget::D)),
            0x83 => Some(Instruction::Add(ArithmeticTarget::E)),
            0x84 => Some(Instruction::Add(ArithmeticTarget::H)),
            0x85 => Some(Instruction::Add(ArithmeticTarget::L)),
            0x86 => Some(Instruction::Add(ArithmeticTarget::HL)),
            0x87 => Some(Instruction::Add(ArithmeticTarget::A)),
            0x88 => Some(Instruction::AddC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::AddC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::AddC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::AddC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::AddC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::AddC(ArithmeticTarget::L)),
            0x8E => Some(Instruction::AddC(ArithmeticTarget::HL)),
            0x8F => Some(Instruction::AddC(ArithmeticTarget::A)),
            _ => None,
        }
    }
}
