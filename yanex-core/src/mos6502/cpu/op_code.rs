use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum OpCode {
    Todo0 = 0x00,
    OraIndX = 0x01,
    Todo2 = 0x02,
    Todo3 = 0x03,
    Todo4 = 0x04,
    OraZp = 0x05,
    AslZp = 0x06,
    Todo7 = 0x07,
    PhpImp = 0x08,
    OraImm = 0x09,
    AslAcc = 0x0A,
    Todo11 = 0x0B,
    Todo12 = 0x0C,
    OraAbs = 0x0D,
    AslAbs = 0x0E,
    Todo15 = 0x0F,
    Todo16 = 0x10,
    OraIndY = 0x11,
    Todo18 = 0x12,
    Todo19 = 0x13,
    Todo20 = 0x14,
    OraZpX = 0x15,
    AslZpX = 0x16,
    Todo23 = 0x17,
    Todo24 = 0x18,
    OraAbsY = 0x19,
    Todo26 = 0x1A,
    Todo27 = 0x1B,
    Todo28 = 0x1C,
    OraAbsX = 0x1D,
    AslAbsX = 0x1E,
    Todo31 = 0x1F,
    Todo32 = 0x20,
    AndIndX = 0x21,
    Todo34 = 0x22,
    Todo35 = 0x23,
    BitZp = 0x24,
    AndZp = 0x25,
    RolZp = 0x26,
    Todo39 = 0x27,
    PlpImp = 0x28,
    AndImm = 0x29,
    RolAcc = 0x2A,
    Todo43 = 0x2B,
    BitAbs = 0x2C,
    AndAbs = 0x2D,
    RolAbs = 0x2E,
    Todo47 = 0x2F,
    Todo48 = 0x30,
    AndIndY = 0x31,
    Todo50 = 0x32,
    Todo51 = 0x33,
    Todo52 = 0x34,
    AndZpX = 0x35,
    RolZpX = 0x36,
    Todo55 = 0x37,
    Todo56 = 0x38,
    AndAbsY = 0x39,
    Todo58 = 0x3A,
    Todo59 = 0x3B,
    Todo60 = 0x3C,
    AndAbsX = 0x3D,
    RolAbsX = 0x3E,
    Todo63 = 0x3F,
    Todo64 = 0x40,
    EorIndX = 0x41,
    Todo66 = 0x42,
    Todo67 = 0x43,
    Todo68 = 0x44,
    EorZp = 0x45,
    LsrZp = 0x46,
    Todo71 = 0x47,
    PhaImp = 0x48,
    EorImm = 0x49,
    LsrAcc = 0x4A,
    Todo75 = 0x4B,
    JmpAbs = 0x4C,
    EorAbs = 0x4D,
    LsrAbs = 0x4E,
    Todo79 = 0x4F,
    Todo80 = 0x50,
    EorIndY = 0x51,
    Todo82 = 0x52,
    Todo83 = 0x53,
    Todo84 = 0x54,
    EorZpX = 0x55,
    LsrZpX = 0x56,
    Todo87 = 0x57,
    Todo88 = 0x58,
    EorAbsY = 0x59,
    Todo90 = 0x5A,
    Todo91 = 0x5B,
    Todo92 = 0x5C,
    EorAbsX = 0x5D,
    LsrAbsX = 0x5E,
    Todo95 = 0x5F,
    Todo96 = 0x60,
    Todo97 = 0x61,
    Todo98 = 0x62,
    Todo99 = 0x63,
    Todo100 = 0x64,
    Todo101 = 0x65,
    RorZp = 0x66,
    Todo103 = 0x67,
    PlaImp = 0x68,
    Todo105 = 0x69,
    RorAcc = 0x6A,
    Todo107 = 0x6B,
    JmpInd = 0x6C,
    Todo109 = 0x6D,
    RorAbs = 0x6E,
    Todo111 = 0x6F,
    Todo112 = 0x70,
    Todo113 = 0x71,
    Todo114 = 0x72,
    Todo115 = 0x73,
    Todo116 = 0x74,
    Todo117 = 0x75,
    RorZpX = 0x76,
    Todo119 = 0x77,
    Todo120 = 0x78,
    Todo121 = 0x79,
    Todo122 = 0x7A,
    Todo123 = 0x7B,
    Todo124 = 0x7C,
    Todo125 = 0x7D,
    RorAbsX = 0x7E,
    Todo127 = 0x7F,
    Todo128 = 0x80,
    StaIndX = 0x81,
    Todo130 = 0x82,
    Todo131 = 0x83,
    StyZp = 0x84,
    StaZp = 0x85,
    StxZp = 0x86,
    Todo135 = 0x87,
    Todo136 = 0x88,
    Todo137 = 0x89,
    TxaImp = 0x8A,
    Todo139 = 0x8B,
    StyAbs = 0x8C,
    StaAbs = 0x8D,
    StxAbs = 0x8E,
    Todo143 = 0x8F,
    Todo144 = 0x90,
    StaIndY = 0x91,
    Todo146 = 0x92,
    Todo147 = 0x93,
    StyZpX = 0x94,
    StaZpX = 0x95,
    StxZpY = 0x96,
    Todo151 = 0x97,
    TyaImp = 0x98,
    StaAbsY = 0x99,
    TxsImp = 0x9A,
    Todo155 = 0x9B,
    Todo156 = 0x9C,
    StaAbsX = 0x9D,
    Todo158 = 0x9E,
    Todo159 = 0x9F,
    LdyImm = 0xA0,
    LdaIndX = 0xA1,
    LdxImm = 0xA2,
    Todo163 = 0xA3,
    LdyZp = 0xA4,
    LdaZp = 0xA5,
    LdxZp = 0xA6,
    Todo167 = 0xA7,
    TayImp = 0xA8,
    LdaImm = 0xA9,
    TaxImp = 0xAA,
    Todo171 = 0xAB,
    LdyAbs = 0xAC,
    LdaAbs = 0xAD,
    LdxAbs = 0xAE,
    Todo175 = 0xAF,
    Todo176 = 0xB0,
    LdaIndY = 0xB1,
    Todo178 = 0xB2,
    Todo179 = 0xB3,
    LdyZpX = 0xB4,
    LdaZpX = 0xB5,
    LdxZpY = 0xB6,
    Todo183 = 0xB7,
    Todo184 = 0xB8,
    LdaAbsY = 0xB9,
    TsxImp = 0xBA,
    Todo187 = 0xBB,
    LdyAbsX = 0xBC,
    LdaAbsX = 0xBD,
    LdxAbsY = 0xBE,
    Todo191 = 0xBF,
    Todo192 = 0xC0,
    Todo193 = 0xC1,
    Todo194 = 0xC2,
    Todo195 = 0xC3,
    Todo196 = 0xC4,
    Todo197 = 0xC5,
    Todo198 = 0xC6,
    Todo199 = 0xC7,
    Todo200 = 0xC8,
    Todo201 = 0xC9,
    Todo202 = 0xCA,
    Todo203 = 0xCB,
    Todo204 = 0xCC,
    Todo205 = 0xCD,
    Todo206 = 0xCE,
    Todo207 = 0xCF,
    Todo208 = 0xD0,
    Todo209 = 0xD1,
    Todo210 = 0xD2,
    Todo211 = 0xD3,
    Todo212 = 0xD4,
    Todo213 = 0xD5,
    Todo214 = 0xD6,
    Todo215 = 0xD7,
    Todo216 = 0xD8,
    Todo217 = 0xD9,
    Todo218 = 0xDA,
    Todo219 = 0xDB,
    Todo220 = 0xDC,
    Todo221 = 0xDD,
    Todo222 = 0xDE,
    Todo223 = 0xDF,
    Todo224 = 0xE0,
    Todo225 = 0xE1,
    Todo226 = 0xE2,
    Todo227 = 0xE3,
    Todo228 = 0xE4,
    Todo229 = 0xE5,
    Todo230 = 0xE6,
    Todo231 = 0xE7,
    Todo232 = 0xE8,
    Todo233 = 0xE9,
    NopImp = 0xEA,
    Todo235 = 0xEB,
    Todo236 = 0xEC,
    Todo237 = 0xED,
    Todo238 = 0xEE,
    Todo239 = 0xEF,
    Todo240 = 0xF0,
    Todo241 = 0xF1,
    Todo242 = 0xF2,
    Todo243 = 0xF3,
    Todo244 = 0xF4,
    Todo245 = 0xF5,
    Todo246 = 0xF6,
    Todo247 = 0xF7,
    Todo248 = 0xF8,
    Todo249 = 0xF9,
    Todo250 = 0xFA,
    Todo251 = 0xFB,
    Todo252 = 0xFC,
    Todo253 = 0xFD,
    Todo254 = 0xFE,
    Todo255 = 0xFF,
}
