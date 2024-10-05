/*
use crate::Cpu;

impl Cpu {
    pub fn load_b_c_dispatch(self: &mut Cpu, opcode: u8) {
        let low_opcode = opcode & 0xF;
        match low_opcode {
            0x0 => {}
            0x1 => self.registers.b = self.registers.c,
            0x2 => self.registers.b = self.registers.d,
            0x3 => self.registers.b = self.registers.e,
            0x4 => self.registers.b = self.registers.h,
            0x5 => self.registers.b = self.registers.l,
            0x6 => self.registers.b = self.memory[self.registers.hl() as usize],
            0x7 => self.registers.b = self.registers.a,
            0x8 => self.registers.c = self.registers.b,
            0x9 => {}
            0xa => self.registers.c = self.registers.d,
            0xb => self.registers.c = self.registers.e,
            0xc => self.registers.c = self.registers.h,
            0xd => self.registers.c = self.registers.l,
            0xe => self.registers.c = self.memory[self.registers.hl() as usize],
            0xf => self.registers.c = self.registers.a,
            _ => eprintln!("Opcode {:x?} not implemented.", opcode),
        }
    }

    pub fn load_d_e_dispatch(self: &mut Cpu, opcode: u8) {
        let low_opcode = opcode & 0xF;
        match low_opcode {
            0x0 => self.registers.d = self.registers.b,
            0x1 => self.registers.d = self.registers.c,
            0x2 => {}
            0x3 => self.registers.d = self.registers.e,
            0x4 => self.registers.d = self.registers.h,
            0x5 => self.registers.d = self.registers.l,
            0x6 => self.registers.d = self.memory[self.registers.hl() as usize],
            0x7 => self.registers.d = self.registers.a,
            0x8 => self.registers.e = self.registers.b,
            0x9 => self.registers.e = self.registers.c,
            0xa => self.registers.e = self.registers.d,
            0xb => {}
            0xc => self.registers.e = self.registers.h,
            0xd => self.registers.e = self.registers.l,
            0xe => self.registers.e = self.memory[self.registers.hl() as usize],
            0xf => self.registers.e = self.registers.a,
            _ => eprintln!("Opcode {:x?} not implemented.", opcode),
        }
    }

    pub fn load_h_l_dispatch(self: &mut Cpu, opcode: u8) {
        let low_opcode = opcode & 0xF;
        match low_opcode {
            0x0 => self.registers.h = self.registers.b,
            0x1 => self.registers.h = self.registers.c,
            0x2 => self.registers.h = self.registers.d,
            0x3 => self.registers.h = self.registers.e,
            0x4 => {}
            0x5 => self.registers.h = self.registers.l,
            0x6 => self.registers.h = self.memory[self.registers.hl() as usize],
            0x7 => self.registers.h = self.registers.a,
            0x8 => self.registers.l = self.registers.b,
            0x9 => self.registers.l = self.registers.c,
            0xa => self.registers.l = self.registers.d,
            0xb => self.registers.l = self.registers.e,
            0xc => self.registers.l = self.registers.h,
            0xd => {}
            0xe => self.registers.l = self.memory[self.registers.hl() as usize],
            0xf => self.registers.l = self.registers.a,
            _ => eprintln!("Opcode {:x?} not implemented.", opcode),
        }
    }

    pub fn load_memory_reg_a_dispatch(self: &mut Cpu, opcode: u8) {
        let low_opcode = opcode & 0xF;
        let position: usize = self.registers.hl() as usize;
        match low_opcode {
            0x0 => self.memory[position] = self.registers.b,
            0x1 => self.memory[position] = self.registers.c,
            0x2 => self.memory[position] = self.registers.d,
            0x3 => self.memory[position] = self.registers.e,
            0x4 => self.memory[position] = self.registers.h,
            0x5 => self.memory[position] = self.registers.l,
            0x6 => eprintln!("Opcode {:x?} not implemented.", opcode),
            0x7 => self.memory[position] = self.registers.a,
            0x8 => self.registers.a = self.registers.b,
            0x9 => self.registers.a = self.registers.c,
            0xa => self.registers.a = self.registers.d,
            0xb => self.registers.a = self.registers.e,
            0xc => self.registers.a = self.registers.h,
            0xd => self.registers.a = self.registers.l,
            0xe => self.registers.a = self.memory[position],
            0xf => {}
            _ => eprintln!("Opcode {:x?} not implemented.", opcode),
        }
    }
}
*/
