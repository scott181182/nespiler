use std::io;

use binrw::{binread, BinRead};



/// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
#[derive(Debug)]
pub enum OpcodeArg {
    /// Operand is implied Accumulator register.
    Accumulator,

    /// Absolute (16-bit) address.
    Absolute(u16),

    /// Absolute (16-bit) address, incremented by X with carry.
    AbsoluteX(u16),

    /// Absolute (16-bit) address, incremented by Y with carry.
    AbsoluteY(u16),

    /// Immediate (8-bit) value.
    Immediate(u8),

    /// Implied (empty) value.
    Implied,

    /// Absolute address, the value in memory at the given absolute address.
    Indirect(u16),

    /// Absolute address, the value in memory at the given zeropage address incremented by X (without carry).
    IndirectX(u8),

    /// Absolute address, the value in memory at the given zeropage address incremented by Y (with carry).
    IndirectY(u8),

    /// Branch target is PC plus the signed offset byte.
    Relative(u8),

    /// Zeropage (8-bit) address.
    ZeroPage(u8),

    /// Zeropage (8-bit) address, incremented by X without carry.
    ZeroPageX(u8),

    /// Zeropage (8-bit) address, incremented by Y without carry.
    ZeroPageY(u8),
}
impl OpcodeArg {
    pub fn size(&self) -> usize {
        match self {
            &OpcodeArg::Accumulator | &OpcodeArg::Implied =>
                0,
            &OpcodeArg::Immediate(_) | &OpcodeArg::IndirectX(_) |
            &OpcodeArg::IndirectY(_) | &OpcodeArg::Relative(_) |
            &OpcodeArg::ZeroPage(_) | &OpcodeArg::ZeroPageX(_) |
            &OpcodeArg::ZeroPageY(_) =>
                1,
            &OpcodeArg::Absolute(_) | &OpcodeArg::AbsoluteX(_) |
            &OpcodeArg::AbsoluteY(_) | &OpcodeArg::Indirect(_) => 
                2,
        }
    }
}
impl std::fmt::Display for OpcodeArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &OpcodeArg::Accumulator => write!(f, "A"),
            &OpcodeArg::Absolute(addr) => write!(f, "${:04x}", addr),
            &OpcodeArg::AbsoluteX(addr) => write!(f, "${:04x},X", addr),
            &OpcodeArg::AbsoluteY(addr) => write!(f, "${:04x},Y", addr),
            &OpcodeArg::Immediate(addr) => write!(f, "#${:02x}", addr),
            &OpcodeArg::Implied => Ok(()),
            &OpcodeArg::Indirect(addr) => write!(f, "(${:04x})", addr),
            &OpcodeArg::IndirectX(addr) => write!(f, "(${:02x},X)", addr),
            &OpcodeArg::IndirectY(addr) => write!(f, "(${:02x}),Y", addr),
            &OpcodeArg::Relative(addr) => write!(f, "${:02x}", addr),
            &OpcodeArg::ZeroPage(addr) => write!(f, "${:02x}", addr),
            &OpcodeArg::ZeroPageX(addr) => write!(f, "${:02x},X", addr),
            &OpcodeArg::ZeroPageY(addr) => write!(f, "${:02x},Y", addr),
        }
    }
}
impl BinRead for OpcodeArg {
    // The opcode, for determining address mode.
    type Args<'a> = u8;

    /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
    fn read_options<R: io::Read + io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        // Strict lower 5-bit matches (columns of the table on the NESDEV wiki).
        match args & 0x1f {
            0x00 =>
                // Weird case in the middle
                if args == 0x20 { Ok(OpcodeArg::Absolute(u16::read_options(reader, endian, ())?)) }
                // Half the column is immediate
                else if args & 0x80 > 0 { Ok(OpcodeArg::Immediate(u8::read_options(reader, endian, ())?)) }
                else { Ok(OpcodeArg::Implied) },
            0x01 | 0x03 =>
                Ok(OpcodeArg::IndirectX(u8::read_options(reader, endian, ())?)),
            0x02 =>
                // Half the column is immediate
                if args & 0x80 > 0 {
                    Ok(OpcodeArg::Immediate(u8::read_options(reader, endian, ())?))
                } else {
                    Ok(OpcodeArg::Implied)
                },
            0x04 | 0x05 | 0x06 | 0x07 =>
                Ok(OpcodeArg::ZeroPage(u8::read_options(reader, endian, ())?)),

            0x08 | 0x0a | 0x12 | 0x18 | 0x1a =>
                Ok(OpcodeArg::Implied),
            0x09 | 0x0b =>
                Ok(OpcodeArg::Immediate(u8::read_options(reader, endian, ())?)),
            0x0c | 0x0d | 0x0e | 0x0f =>
                // Just one exception to this block.
                if args == 0x6c {
                    Ok(OpcodeArg::Indirect(u16::read_options(reader, endian, ())?))
                } else {
                    Ok(OpcodeArg::Absolute(u16::read_options(reader, endian, ())?))
                },

            0x10 => Ok(OpcodeArg::Relative(u8::read_options(reader, endian, ())?)),
            0x11 | 0x13 =>
                Ok(OpcodeArg::IndirectY(u8::read_options(reader, endian, ())?)),
            0x14 | 0x15 | 0x16 | 0x17 =>
                // Just a little block breaking the trend.
                if args & 0xde == 0xa6 {
                    Ok(OpcodeArg::ZeroPageY(u8::read_options(reader, endian, ())?))
                } else {
                    Ok(OpcodeArg::ZeroPageX(u8::read_options(reader, endian, ())?))
                }

            0x19 | 0x1b =>
                Ok(OpcodeArg::AbsoluteY(u16::read_options(reader, endian, ())?)),
            0x1c | 0x1d | 0x1e | 0x1f =>
                // Just a little block breaking the trend.
                if args & 0xde == 0xae {
                    Ok(OpcodeArg::AbsoluteY(u16::read_options(reader, endian, ())?))
                } else {
                    Ok(OpcodeArg::AbsoluteX(u16::read_options(reader, endian, ())?))
                }

            _ => unreachable!()
        }
    }
}



#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    /// add with carry
    ADC,
    /// and (with accumulator)
    AND,
    /// arithmetic shift left
    ASL,
    /// branch on carry clear
    BCC,
    /// branch on carry set
    BCS,
    /// branch on equal (zero set)
    BEQ,
    /// bit test
    BIT,
    /// branch on minus (negative set)
    BMI,
    /// branch on not equal (zero clear)
    BNE,
    /// branch on plus (negative clear)
    BPL,
    /// break / interrupt
    BRK,
    /// branch on overflow clear
    BVC,
    /// branch on overflow set
    BVS,
    /// clear carry
    CLC,
    /// clear decimal
    CLD,
    /// clear interrupt disable
    CLI,
    /// clear overflow
    CLV,
    /// compare (with accumulator)
    CMP,
    /// compare with X
    CPX,
    /// compare with Y
    CPY,
    /// decrement
    DEC,
    /// decrement X
    DEX,
    /// decrement Y
    DEY,
    /// exclusive or (with accumulator)
    EOR,
    /// increment
    INC,
    /// increment X
    INX,
    /// increment Y
    INY,
    /// jump
    JMP,
    /// jump subroutine
    JSR,
    /// load accumulator
    LDA,
    /// load X
    LDX,
    /// load Y
    LDY,
    /// logical shift right
    LSR,
    /// no operation
    NOP,
    /// or with accumulator
    ORA,
    /// push accumulator
    PHA,
    /// push processor status (SR)
    PHP,
    /// pull accumulator
    PLA,
    /// pull processor status (SR)
    PLP,
    /// rotate left
    ROL,
    /// rotate right
    ROR,
    /// return from interrupt
    RTI,
    /// return from subroutine
    RTS,
    /// subtract with carry
    SBC,
    /// set carry
    SEC,
    /// set decimal
    SED,
    /// set interrupt disable
    SEI,
    /// store accumulator
    STA,
    /// store X
    STX,
    /// store Y
    STY,
    /// transfer accumulator to X
    TAX,
    /// transfer accumulator to Y
    TAY,
    /// transfer stack pointer to X
    TSX,
    /// transfer X to accumulator
    TXA,
    /// transfer X to stack pointer
    TXS,
    /// transfer Y to accumulator 
    TYA,

    ///////////////////////
    // "ILLEGAL" OPCODES //
    ///////////////////////

    /// a.k.a SHA or AXA
    /// 
    /// Stores A AND X AND (high-byte of addr. + 1) at addr. 
    AHX,
    /// ALR = AND + LSR
    ALR,
    /// ANC = AND, bit(7) -> Carry
    ANC,
    /// ARR = AND + ROR
    ARR,
    /// a.k.a SBX or SAX
    /// 
    /// CMP and DEX at once, sets flags like CMP
    AXS,
    /// DCP = DEC + CMP
    DCP,
    /// ISC = INC + SBC
    ISC,
    /// LSA/TSX oper
    /// 
    /// M AND SP -> A, X, SP
    LAS,
    /// LAZ = LDA + LDX
    LAX,
    /// RLA = ROL + AND
    RLA,
    /// RRA = ROR + ADC
    RRA,
    /// a.k.a. SBX, AXS
    /// 
    /// (A AND X) - oper -> X
    SAX,
    /// a.k.a A11, SXA, XAS
    /// 
    /// Stores X AND (high-byte of addr. + 1) at addr.
    SHX,
    /// a.k.a A11, SYA, SAY
    /// 
    /// Stores Y AND (high-byte of addr. + 1) at addr.
    SHY,
    /// SLO = ASL + ORA
    SLO,
    /// SRE = LSR + EOR
    SRE,
    /// Puts A AND X in SP and stores A AND X AND (high-byte of addr. + 1) at addr.
    TAS,
    /// a.k.a ANE
    /// 
    /// `(A OR CONST) AND X AND oper -> A`
    XAA,


    /// NES Stop (?)
    STP,
}

const OPCODE_MAP: [Opcode; 256] = [
    Opcode::BRK, Opcode::ORA, Opcode::STP, Opcode::SLO, Opcode::NOP, Opcode::ORA, Opcode::ASL, Opcode::SLO, Opcode::PHP, Opcode::ORA, Opcode::ASL, Opcode::ANC, Opcode::NOP, Opcode::ORA, Opcode::ASL, Opcode::SLO, 
    Opcode::BPL, Opcode::ORA, Opcode::STP, Opcode::SLO, Opcode::NOP, Opcode::ORA, Opcode::ASL, Opcode::SLO, Opcode::CLC, Opcode::ORA, Opcode::NOP, Opcode::SLO, Opcode::NOP, Opcode::ORA, Opcode::ASL, Opcode::SLO, 
    Opcode::JSR, Opcode::AND, Opcode::STP, Opcode::RLA, Opcode::BIT, Opcode::AND, Opcode::ROL, Opcode::RLA, Opcode::PLP, Opcode::AND, Opcode::ROL, Opcode::ANC, Opcode::BIT, Opcode::AND, Opcode::ROL, Opcode::RLA, 
    Opcode::BMI, Opcode::AND, Opcode::STP, Opcode::RLA, Opcode::NOP, Opcode::AND, Opcode::ROL, Opcode::RLA, Opcode::SEC, Opcode::AND, Opcode::NOP, Opcode::RLA, Opcode::NOP, Opcode::AND, Opcode::ROL, Opcode::RLA, 
    Opcode::RTI, Opcode::EOR, Opcode::STP, Opcode::SRE, Opcode::NOP, Opcode::EOR, Opcode::LSR, Opcode::SRE, Opcode::PHA, Opcode::EOR, Opcode::LSR, Opcode::ALR, Opcode::JMP, Opcode::EOR, Opcode::LSR, Opcode::SRE, 
    Opcode::BVC, Opcode::EOR, Opcode::STP, Opcode::SRE, Opcode::NOP, Opcode::EOR, Opcode::LSR, Opcode::SRE, Opcode::CLI, Opcode::EOR, Opcode::NOP, Opcode::SRE, Opcode::NOP, Opcode::EOR, Opcode::LSR, Opcode::SRE, 
    Opcode::RTS, Opcode::ADC, Opcode::STP, Opcode::RRA, Opcode::NOP, Opcode::ADC, Opcode::ROR, Opcode::RRA, Opcode::PLA, Opcode::ADC, Opcode::ROR, Opcode::ARR, Opcode::JMP, Opcode::ADC, Opcode::ROR, Opcode::RRA, 
    Opcode::BVS, Opcode::ADC, Opcode::STP, Opcode::RRA, Opcode::NOP, Opcode::ADC, Opcode::ROR, Opcode::RRA, Opcode::SEI, Opcode::ADC, Opcode::NOP, Opcode::RRA, Opcode::NOP, Opcode::ADC, Opcode::ROR, Opcode::RRA, 

    Opcode::NOP, Opcode::STA, Opcode::NOP, Opcode::SAX, Opcode::STY, Opcode::STA, Opcode::STX, Opcode::SAX, Opcode::DEY, Opcode::NOP, Opcode::TXA, Opcode::XAA, Opcode::STY, Opcode::STA, Opcode::STX, Opcode::SAX, 
    Opcode::BCC, Opcode::STA, Opcode::STP, Opcode::AHX, Opcode::STY, Opcode::STA, Opcode::STX, Opcode::SAX, Opcode::TYA, Opcode::STA, Opcode::TXS, Opcode::TAS, Opcode::SHY, Opcode::STA, Opcode::SHX, Opcode::AHX, 
    Opcode::LDY, Opcode::LDA, Opcode::LDX, Opcode::LAX, Opcode::LDY, Opcode::LDA, Opcode::LDX, Opcode::LAX, Opcode::TAY, Opcode::LDA, Opcode::TAX, Opcode::LAX, Opcode::LDY, Opcode::LDA, Opcode::LDX, Opcode::LAX, 
    Opcode::BCS, Opcode::LDA, Opcode::STP, Opcode::LAX, Opcode::LDY, Opcode::LDA, Opcode::LDX, Opcode::LAX, Opcode::CLV, Opcode::LDA, Opcode::TSX, Opcode::LAS, Opcode::LDY, Opcode::LDA, Opcode::LDX, Opcode::LAX, 
    Opcode::CPY, Opcode::CMP, Opcode::NOP, Opcode::DCP, Opcode::CPY, Opcode::CMP, Opcode::DEC, Opcode::DCP, Opcode::INY, Opcode::CMP, Opcode::DEX, Opcode::AXS, Opcode::CPY, Opcode::CMP, Opcode::DEC, Opcode::DCP, 
    Opcode::BNE, Opcode::CMP, Opcode::STP, Opcode::DCP, Opcode::NOP, Opcode::CMP, Opcode::DEC, Opcode::DCP, Opcode::CLD, Opcode::CMP, Opcode::NOP, Opcode::DCP, Opcode::NOP, Opcode::CMP, Opcode::DEC, Opcode::DCP, 
    Opcode::CPX, Opcode::SBC, Opcode::NOP, Opcode::ISC, Opcode::CPX, Opcode::SBC, Opcode::INC, Opcode::ISC, Opcode::INX, Opcode::SBC, Opcode::NOP, Opcode::SBC, Opcode::CPX, Opcode::SBC, Opcode::INC, Opcode::ISC, 
    Opcode::BEQ, Opcode::SBC, Opcode::STP, Opcode::ISC, Opcode::NOP, Opcode::SBC, Opcode::INC, Opcode::ISC, Opcode::SED, Opcode::SBC, Opcode::NOP, Opcode::ISC, Opcode::NOP, Opcode::SBC, Opcode::INC, Opcode::ISC, 
];

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        OPCODE_MAP[value as usize]
    }
}



#[binread]
#[derive(Debug)]
#[br(little)]
pub struct Operation {
    raw_opcode: u8,

    #[br(calc = Opcode::from(raw_opcode))]
    pub opcode: Opcode,
    #[br(args_raw = raw_opcode)]
    pub arguments: OpcodeArg
}
impl Operation {
    pub fn to_source_string(&self) -> String {
        format!("{:?}   {}", self.opcode, self.arguments)
    }
}
