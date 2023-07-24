use num_enum::{FromPrimitive, IntoPrimitive};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, IntoPrimitive, FromPrimitive)]
pub enum Opcode {
    Todo00 = 0x00,
    Todo01 = 0x01,
    Todo02 = 0x02,
    Todo03 = 0x03,
    Todo04 = 0x04,
    Todo05 = 0x05,
    Todo06 = 0x06,
    Todo07 = 0x07,
    Todo08 = 0x08,
    Todo09 = 0x09,
    Todo0A = 0x0A,
    Todo0B = 0x0B,
    Todo0C = 0x0C,
    Todo0D = 0x0D,
    Todo0E = 0x0E,
    Todo0F = 0x0F,
    Todo10 = 0x10,
    Todo11 = 0x11,
    Todo12 = 0x12,
    Todo13 = 0x13,
    Todo14 = 0x14,
    Todo15 = 0x15,
    Todo16 = 0x16,
    Todo17 = 0x17,
    Todo18 = 0x18,
    Todo19 = 0x19,
    Todo1A = 0x1A,
    Todo1B = 0x1B,
    Todo1C = 0x1C,
    Todo1D = 0x1D,
    Todo1E = 0x1E,
    Todo1F = 0x1F,
    Todo20 = 0x20,
    Todo21 = 0x21,
    Todo22 = 0x22,
    Todo23 = 0x23,
    Todo24 = 0x24,
    Todo25 = 0x25,
    Todo26 = 0x26,
    Todo27 = 0x27,
    Todo28 = 0x28,
    Todo29 = 0x29,
    Todo2A = 0x2A,
    Todo2B = 0x2B,
    Todo2C = 0x2C,
    Todo2D = 0x2D,
    Todo2E = 0x2E,
    Todo2F = 0x2F,
    Todo30 = 0x30,
    Todo31 = 0x31,
    Todo32 = 0x32,
    Todo33 = 0x33,
    Todo34 = 0x34,
    Todo35 = 0x35,
    Todo36 = 0x36,
    Todo37 = 0x37,
    Todo38 = 0x38,
    Todo39 = 0x39,
    Todo3A = 0x3A,
    Todo3B = 0x3B,
    Todo3C = 0x3C,
    Todo3D = 0x3D,
    Todo3E = 0x3E,
    Todo3F = 0x3F,
    Todo40 = 0x40,
    Todo41 = 0x41,
    Todo42 = 0x42,
    Todo43 = 0x43,
    Todo44 = 0x44,
    Todo45 = 0x45,
    Todo46 = 0x46,
    Todo47 = 0x47,
    Todo48 = 0x48,
    Todo49 = 0x49,
    Todo4A = 0x4A,
    Todo4B = 0x4B,
    Todo4C = 0x4C,
    Todo4D = 0x4D,
    Todo4E = 0x4E,
    Todo4F = 0x4F,
    Todo50 = 0x50,
    Todo51 = 0x51,
    Todo52 = 0x52,
    Todo53 = 0x53,
    Todo54 = 0x54,
    Todo55 = 0x55,
    Todo56 = 0x56,
    Todo57 = 0x57,
    Todo58 = 0x58,
    Todo59 = 0x59,
    Todo5A = 0x5A,
    Todo5B = 0x5B,
    Todo5C = 0x5C,
    Todo5D = 0x5D,
    Todo5E = 0x5E,
    Todo5F = 0x5F,
    Todo60 = 0x60,
    Todo61 = 0x61,
    Todo62 = 0x62,
    Todo63 = 0x63,
    Todo64 = 0x64,
    Todo65 = 0x65,
    Todo66 = 0x66,
    Todo67 = 0x67,
    Todo68 = 0x68,
    Todo69 = 0x69,
    Todo6A = 0x6A,
    Todo6B = 0x6B,
    Todo6C = 0x6C,
    Todo6D = 0x6D,
    Todo6E = 0x6E,
    Todo6F = 0x6F,
    Todo70 = 0x70,
    Todo71 = 0x71,
    Todo72 = 0x72,
    Todo73 = 0x73,
    Todo74 = 0x74,
    Todo75 = 0x75,
    Todo76 = 0x76,
    Todo77 = 0x77,
    Todo78 = 0x78,
    Todo79 = 0x79,
    Todo7A = 0x7A,
    Todo7B = 0x7B,
    Todo7C = 0x7C,
    Todo7D = 0x7D,
    Todo7E = 0x7E,
    Todo7F = 0x7F,
    Todo80 = 0x80,
    StaIndX = 0x81,
    Todo82 = 0x82,
    Todo83 = 0x83,
    Todo84 = 0x84,
    StaZp = 0x85,
    Todo86 = 0x86,
    Todo87 = 0x87,
    Todo88 = 0x88,
    Todo89 = 0x89,
    Todo8A = 0x8A,
    Todo8B = 0x8B,
    Todo8C = 0x8C,
    StaAbs = 0x8D,
    Todo8E = 0x8E,
    Todo8F = 0x8F,
    Todo90 = 0x90,
    StaIndY = 0x91,
    Todo92 = 0x92,
    Todo93 = 0x93,
    Todo94 = 0x94,
    StaZpX = 0x95,
    Todo96 = 0x96,
    Todo97 = 0x97,
    Todo98 = 0x98,
    StaAbsY = 0x99,
    Todo9A = 0x9A,
    Todo9B = 0x9B,
    Todo9C = 0x9C,
    StaAbsX = 0x9D,
    Todo9E = 0x9E,
    Todo9F = 0x9F,
    TodoA0 = 0xA0,
    LdaIndX = 0xA1,
    TodoA2 = 0xA2,
    TodoA3 = 0xA3,
    TodoA4 = 0xA4,
    LdaZp = 0xA5,
    TodoA6 = 0xA6,
    TodoA7 = 0xA7,
    TodoA8 = 0xA8,
    LdaImm = 0xA9,
    TodoAA = 0xAA,
    TodoAB = 0xAB,
    TodoAC = 0xAC,
    LdaAbs = 0xAD,
    TodoAE = 0xAE,
    TodoAF = 0xAF,
    TodoB0 = 0xB0,
    LdaIndY = 0xB1,
    TodoB2 = 0xB2,
    TodoB3 = 0xB3,
    TodoB4 = 0xB4,
    LdaZpX = 0xB5,
    TodoB6 = 0xB6,
    TodoB7 = 0xB7,
    TodoB8 = 0xB8,
    LdaAbsY = 0xB9,
    TodoBA = 0xBA,
    TodoBB = 0xBB,
    TodoBC = 0xBC,
    LdaAbsX = 0xBD,
    TodoBE = 0xBE,
    TodoBF = 0xBF,
    TodoC0 = 0xC0,
    TodoC1 = 0xC1,
    TodoC2 = 0xC2,
    TodoC3 = 0xC3,
    TodoC4 = 0xC4,
    TodoC5 = 0xC5,
    TodoC6 = 0xC6,
    TodoC7 = 0xC7,
    TodoC8 = 0xC8,
    TodoC9 = 0xC9,
    TodoCA = 0xCA,
    TodoCB = 0xCB,
    TodoCC = 0xCC,
    TodoCD = 0xCD,
    TodoCE = 0xCE,
    TodoCF = 0xCF,
    TodoD0 = 0xD0,
    TodoD1 = 0xD1,
    TodoD2 = 0xD2,
    TodoD3 = 0xD3,
    TodoD4 = 0xD4,
    TodoD5 = 0xD5,
    TodoD6 = 0xD6,
    TodoD7 = 0xD7,
    TodoD8 = 0xD8,
    TodoD9 = 0xD9,
    TodoDA = 0xDA,
    TodoDB = 0xDB,
    TodoDC = 0xDC,
    TodoDD = 0xDD,
    TodoDE = 0xDE,
    TodoDF = 0xDF,
    TodoE0 = 0xE0,
    TodoE1 = 0xE1,
    TodoE2 = 0xE2,
    TodoE3 = 0xE3,
    TodoE4 = 0xE4,
    TodoE5 = 0xE5,
    TodoE6 = 0xE6,
    TodoE7 = 0xE7,
    TodoE8 = 0xE8,
    TodoE9 = 0xE9,
    TodoEA = 0xEA,
    TodoEB = 0xEB,
    TodoEC = 0xEC,
    TodoED = 0xED,
    TodoEE = 0xEE,
    TodoEF = 0xEF,
    TodoF0 = 0xF0,
    TodoF1 = 0xF1,
    TodoF2 = 0xF2,
    TodoF3 = 0xF3,
    TodoF4 = 0xF4,
    TodoF5 = 0xF5,
    TodoF6 = 0xF6,
    TodoF7 = 0xF7,
    TodoF8 = 0xF8,
    TodoF9 = 0xF9,
    TodoFA = 0xFA,
    TodoFB = 0xFB,
    TodoFC = 0xFC,
    TodoFD = 0xFD,
    TodoFE = 0xFE,
    TodoFF = 0xFF,
}

use crate::cpu::operation::Operation;

impl From<Opcode> for Operation {
    fn from(opcode: Opcode) -> Self {
        use crate::cpu::operation::addressing_mode::AddressingMode::*;
        use crate::cpu::operation::opcode::Opcode::*;
        use crate::cpu::operation::operations::load::*;
        use crate::cpu::operation::Operation::*;

        match opcode {
            LdaImm => LoadAccumulator(LoadAccumulatorState::Decoded(Immediate)),
            LdaAbs => LoadAccumulator(LoadAccumulatorState::Decoded(Absolute)),
            // LdaAbsX => LoadAccumulator(LoadAccumulatorState::Decoded(AbsoluteX)),
            // LdaAbsY => LoadAccumulator(LoadAccumulatorState::Decoded(AbsoluteY)),
            // LdaZp => LoadAccumulator(LoadAccumulatorState::Decoded(ZeroPage)),
            // LdaZpX => LoadAccumulator(LoadAccumulatorState::Decoded(ZeroPageX)),
            // LdaIndX => LoadAccumulator(LoadAccumulatorState::Decoded(IndirectX)),
            // LdaIndY => LoadAccumulator(LoadAccumulatorState::Decoded(IndirectY)),
            _ => todo!("Unsupported opcode {:?}", opcode),
        }
    }
}
