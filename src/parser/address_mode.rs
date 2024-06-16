use binrw::{io, BinRead};
use subenum::subenum;




/// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
#[subenum(
    AddrModeImmediate,
    AddrModeNoZeropageY, AddrModeSimpleXAcc, AddrModeRelative, AddrModeSimple,
    AddrModeSimpleOrImm, AddrModeSimpleX, AddrModeAbsInd, AddrModeAbs,
    AddrModeSimpleYImm, AddrModeSimpleXImm, AddrModeNoZeropageYNoImm,
    AddrModeSTX, AddrModeSTY, AddrModeAHX, AddrModeLAX,
    AddrModeAbsY, AddrModeAbsX
)]
#[derive(Debug)]
pub enum AddressMode {
    /// Operand is implied Accumulator register.
    #[subenum(AddrModeSimpleXAcc)]
    Accumulator,

    /// Absolute (16-bit) address.
    #[subenum(
        AddrModeNoZeropageY, AddrModeNoZeropageYNoImm, AddrModeSimpleXAcc, AddrModeSimple,
        AddrModeSimpleOrImm, AddrModeSimpleX, AddrModeAbsInd, AddrModeAbs,
        AddrModeSimpleYImm, AddrModeSimpleXImm, AddrModeSTX, AddrModeSTY,
        AddrModeLAX
    )]
    Absolute(u16),

    /// Absolute (16-bit) address, incremented by X with carry.
    #[subenum(AddrModeNoZeropageY, AddrModeNoZeropageYNoImm, AddrModeSimpleXAcc, AddrModeSimpleX, AddrModeSimpleXImm, AddrModeAbsX)]
    AbsoluteX(u16),

    /// Absolute (16-bit) address, incremented by Y with carry.
    #[subenum(AddrModeNoZeropageY, AddrModeNoZeropageYNoImm, AddrModeSimpleYImm, AddrModeAHX, AddrModeAbsY, AddrModeLAX)]
    AbsoluteY(u16),

    /// Immediate (8-bit) value.
    #[subenum(AddrModeNoZeropageY, AddrModeSimpleOrImm, AddrModeSimpleYImm, AddrModeSimpleXImm, AddrModeImmediate)]
    Immediate(u8),

    /// Implied (empty) value.
    Implied,

    /// Absolute address, the value in memory at the given absolute address.
    #[subenum(AddrModeAbsInd)]
    Indirect(u16),

    /// Absolute address, the value in memory at the given zeropage address incremented by X (without carry).
    #[subenum(AddrModeNoZeropageY, AddrModeNoZeropageYNoImm, AddrModeLAX)]
    IndirectX(u8),

    /// Absolute address, the value in memory at the given zeropage address incremented by Y (with carry).
    #[subenum(AddrModeNoZeropageY, AddrModeNoZeropageYNoImm, AddrModeAHX, AddrModeLAX)]
    IndirectY(u8),

    /// Branch target is PC plus the signed offset byte.
    #[subenum(AddrModeRelative)]
    Relative(u8),

    /// Zeropage (8-bit) address.
    #[subenum(AddrModeNoZeropageY, AddrModeNoZeropageYNoImm, AddrModeSimpleXAcc, AddrModeSimple, AddrModeSimpleOrImm, AddrModeSimpleX, AddrModeSimpleYImm, AddrModeSimpleXImm, AddrModeSTX, AddrModeSTY, AddrModeLAX)]
    ZeroPage(u8),

    /// Zeropage (8-bit) address, incremented by X without carry.
    #[subenum(AddrModeNoZeropageY, AddrModeNoZeropageYNoImm, AddrModeSimpleXAcc, AddrModeSimpleX, AddrModeSimpleXImm, AddrModeSTY)]
    ZeroPageX(u8),

    /// Zeropage (8-bit) address, incremented by Y without carry.
    #[subenum(AddrModeSimpleYImm, AddrModeSTX, AddrModeLAX)]
    ZeroPageY(u8),
}
impl AddressMode {
    pub fn size(&self) -> usize {
        match self {
            &AddressMode::Accumulator | &AddressMode::Implied =>
                0,
            &AddressMode::Immediate(_) | &AddressMode::IndirectX(_) |
            &AddressMode::IndirectY(_) | &AddressMode::Relative(_) |
            &AddressMode::ZeroPage(_) | &AddressMode::ZeroPageX(_) |
            &AddressMode::ZeroPageY(_) =>
                1,
            &AddressMode::Absolute(_) | &AddressMode::AbsoluteX(_) |
            &AddressMode::AbsoluteY(_) | &AddressMode::Indirect(_) => 
                2,
        }
    }
}
impl std::fmt::Display for AddressMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &AddressMode::Accumulator => write!(f, "A"),
            &AddressMode::Absolute(addr) => write!(f, "${:04x}", addr),
            &AddressMode::AbsoluteX(addr) => write!(f, "${:04x},X", addr),
            &AddressMode::AbsoluteY(addr) => write!(f, "${:04x},Y", addr),
            &AddressMode::Immediate(addr) => write!(f, "#${:02x}", addr),
            &AddressMode::Implied => Ok(()),
            &AddressMode::Indirect(addr) => write!(f, "(${:04x})", addr),
            &AddressMode::IndirectX(addr) => write!(f, "(${:02x},X)", addr),
            &AddressMode::IndirectY(addr) => write!(f, "(${:02x}),Y", addr),
            &AddressMode::Relative(addr) => write!(f, "${:02x}", addr),
            &AddressMode::ZeroPage(addr) => write!(f, "${:02x}", addr),
            &AddressMode::ZeroPageX(addr) => write!(f, "${:02x},X", addr),
            &AddressMode::ZeroPageY(addr) => write!(f, "${:02x},Y", addr),
        }
    }
}
impl BinRead for AddressMode {
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
                if args == 0x20 { Ok(AddressMode::Absolute(u16::read_options(reader, endian, ())?)) }
                // Half the column is immediate
                else if args & 0x80 > 0 { Ok(AddressMode::Immediate(u8::read_options(reader, endian, ())?)) }
                else { Ok(AddressMode::Implied) },
            0x01 | 0x03 =>
                Ok(AddressMode::IndirectX(u8::read_options(reader, endian, ())?)),
            0x02 =>
                // Half the column is immediate
                if args & 0x80 > 0 {
                    Ok(AddressMode::Immediate(u8::read_options(reader, endian, ())?))
                } else {
                    Ok(AddressMode::Implied)
                },
            0x04 | 0x05 | 0x06 | 0x07 =>
                Ok(AddressMode::ZeroPage(u8::read_options(reader, endian, ())?)),

            0x08 | 0x0a | 0x12 | 0x18 | 0x1a =>
                Ok(AddressMode::Implied),
            0x09 | 0x0b =>
                Ok(AddressMode::Immediate(u8::read_options(reader, endian, ())?)),
            0x0c | 0x0d | 0x0e | 0x0f =>
                // Just one exception to this block.
                if args == 0x6c {
                    Ok(AddressMode::Indirect(u16::read_options(reader, endian, ())?))
                } else {
                    Ok(AddressMode::Absolute(u16::read_options(reader, endian, ())?))
                },

            0x10 => Ok(AddressMode::Relative(u8::read_options(reader, endian, ())?)),
            0x11 | 0x13 =>
                Ok(AddressMode::IndirectY(u8::read_options(reader, endian, ())?)),
            0x14 | 0x15 | 0x16 | 0x17 =>
                // Just a little block breaking the trend.
                if args & 0xde == 0xa6 {
                    Ok(AddressMode::ZeroPageY(u8::read_options(reader, endian, ())?))
                } else {
                    Ok(AddressMode::ZeroPageX(u8::read_options(reader, endian, ())?))
                }

            0x19 | 0x1b =>
                Ok(AddressMode::AbsoluteY(u16::read_options(reader, endian, ())?)),
            0x1c | 0x1d | 0x1e | 0x1f =>
                // Just a little block breaking the trend.
                if args & 0xde == 0xae {
                    Ok(AddressMode::AbsoluteY(u16::read_options(reader, endian, ())?))
                } else {
                    Ok(AddressMode::AbsoluteX(u16::read_options(reader, endian, ())?))
                }

            _ => unreachable!()
        }
    }
}







#[derive(Debug)]
pub enum Opcode {
    /// add with carry
    ADC(AddrModeNoZeropageY),
    /// and (with accumulator)
    AND(AddrModeNoZeropageY),
    /// arithmetic shift left
    ASL(AddrModeSimpleXAcc),
    /// branch on carry clear
    BCC(AddrModeRelative),
    /// branch on carry set
    BCS(AddrModeRelative),
    /// branch on equal (zero set)
    BEQ(AddrModeRelative),
    /// bit test
    BIT(AddrModeSimple),
    /// branch on minus (negative set)
    BMI(AddrModeRelative),
    /// branch on not equal (zero clear)
    BNE(AddrModeRelative),
    /// branch on plus (negative clear)
    BPL(AddrModeRelative),
    /// break / interrupt
    BRK,
    /// branch on overflow clear
    BVC(AddrModeRelative),
    /// branch on overflow set
    BVS(AddrModeRelative),
    /// clear carry
    CLC,
    /// clear decimal
    CLD,
    /// clear interrupt disable
    CLI,
    /// clear overflow
    CLV,
    /// compare (with accumulator)
    CMP(AddrModeNoZeropageY),
    /// compare with X
    CPX(AddrModeSimpleOrImm),
    /// compare with Y
    CPY(AddrModeSimpleOrImm),
    /// decrement
    DEC(AddrModeSimpleX),
    /// decrement X
    DEX,
    /// decrement Y
    DEY,
    /// exclusive or (with accumulator)
    EOR(AddrModeNoZeropageY),
    /// increment
    INC(AddrModeSimpleX),
    /// increment X
    INX,
    /// increment Y
    INY,
    /// jump
    JMP(AddrModeAbsInd),
    /// jump subroutine
    JSR(AddrModeAbs),
    /// load accumulator
    LDA(AddrModeNoZeropageY),
    /// load X
    LDX(AddrModeSimpleYImm),
    /// load Y
    LDY(AddrModeSimpleXImm),
    /// logical shift right
    LSR(AddrModeSimpleXAcc),
    /// no operation
    NOP,
    /// or with accumulator
    ORA(AddrModeNoZeropageY),
    /// push accumulator
    PHA,
    /// push processor status (SR)
    PHP,
    /// pull accumulator
    PLA,
    /// pull processor status (SR)
    PLP,
    /// rotate left
    ROL(AddrModeSimpleXAcc),
    /// rotate right
    ROR(AddrModeSimpleXAcc),
    /// return from interrupt
    RTI,
    /// return from subroutine
    RTS,
    /// subtract with carry
    SBC(AddrModeNoZeropageY),
    /// set carry
    SEC,
    /// set decimal
    SED,
    /// set interrupt disable
    SEI,
    /// store accumulator
    STA(AddrModeNoZeropageYNoImm),
    /// store X
    STX(AddrModeSTX),
    /// store Y
    STY(AddrModeSTY),
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
    AHX(AddrModeAHX),
    /// ALR = AND + LSR
    ALR(AddrModeImmediate),
    /// ANC = AND, bit(7) -> Carry
    ANC(AddrModeImmediate),
    /// ARR = AND + ROR
    ARR(AddrModeImmediate),
    /// a.k.a SBX or SAX
    /// 
    /// CMP and DEX at once, sets flags like CMP
    AXS(AddrModeImmediate),
    /// DCP = DEC + CMP
    DCP(AddrModeNoZeropageYNoImm),
    /// ISC = INC + SBC
    ISC(AddrModeNoZeropageYNoImm),
    /// LSA/TSX oper
    /// 
    /// M AND SP -> A, X, SP
    LAS(AddrModeAbsY),
    /// LAZ = LDA + LDX
    LAX(AddrModeLAX),
    /// RLA = ROL + AND
    RLA(AddrModeNoZeropageYNoImm),
    /// RRA = ROR + ADC
    RRA(AddrModeNoZeropageYNoImm),
    /// a.k.a. SBX, AXS
    /// 
    /// (A AND X) - oper -> X
    SAX(AddrModeImmediate),
    /// a.k.a A11, SXA, XAS
    /// 
    /// Stores X AND (high-byte of addr. + 1) at addr.
    SHX(AddrModeAbsY),
    /// a.k.a A11, SYA, SAY
    /// 
    /// Stores Y AND (high-byte of addr. + 1) at addr.
    SHY(AddrModeAbsX),
    /// SLO = ASL + ORA
    SLO(AddrModeNoZeropageYNoImm),
    /// SRE = LSR + EOR
    SRE(AddrModeNoZeropageYNoImm),
    /// Puts A AND X in SP and stores A AND X AND (high-byte of addr. + 1) at addr.
    TAS(AddrModeAbsY),
    /// a.k.a ANE
    /// 
    /// `(A OR CONST) AND X AND oper -> A`
    XAA(AddrModeImmediate),


    /// NES Stop (?)
    STP,
}


// impl BinRead for Opcode {
//     type Args<'a> = ();

//     fn read_options<R: io::Read + io::Seek>(
//         reader: &mut R,
//         endian: binrw::Endian,
//         args: Self::Args<'_>,
//     ) -> binrw::BinResult<Self> {
//         let mut buf = [0u8; 1];
//         reader.read_exact(&mut buf);
//         let opcode = buf[0];

        
//     }
// }
