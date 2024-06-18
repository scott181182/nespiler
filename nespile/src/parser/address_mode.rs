use binrw::{io, BinRead};
use nespile_macros::BinReadAddressMode;
use subenum::subenum;



/// Address Modes per https://www.masswerk.at/6502/6502_instruction_set.html
#[subenum(
    AddrModeImmediate,
    AddrModeNoZeropageY, AddrModeSimpleXAcc, AddrModeRelative, AddrModeSimple,
    AddrModeSimpleOrImm, AddrModeSimpleX, AddrModeAbsInd, AddrModeAbs,
    AddrModeSimpleYImm, AddrModeSimpleXImm, AddrModeNoZeropageYNoImm,
    AddrModeSAX, AddrModeSTX, AddrModeSTY, AddrModeAHX, AddrModeLAX,
    AddrModeAbsY, AddrModeAbsX
)]
#[derive(Debug, Clone, Copy, BinReadAddressMode)]
pub enum AddressMode {
    /// Operand is implied Accumulator register.
    #[subenum(AddrModeSimpleXAcc)]
    Accumulator,

    /// Absolute (16-bit) address.
    #[subenum(
        AddrModeNoZeropageY, AddrModeNoZeropageYNoImm, AddrModeSimpleXAcc, AddrModeSimple,
        AddrModeSimpleOrImm, AddrModeSimpleX, AddrModeAbsInd, AddrModeAbs,
        AddrModeSimpleYImm, AddrModeSimpleXImm, AddrModeSAX, AddrModeSTX, AddrModeSTY,
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
    #[subenum(AddrModeNoZeropageY, AddrModeNoZeropageYNoImm, AddrModeSAX, AddrModeLAX)]
    IndirectX(u8),

    /// Absolute address, the value in memory at the given zeropage address incremented by Y (with carry).
    #[subenum(AddrModeNoZeropageY, AddrModeNoZeropageYNoImm, AddrModeAHX, AddrModeLAX)]
    IndirectY(u8),

    /// Branch target is PC plus the signed offset byte.
    #[subenum(AddrModeRelative)]
    Relative(u8),

    /// Zeropage (8-bit) address.
    #[subenum(AddrModeNoZeropageY, AddrModeNoZeropageYNoImm, AddrModeSimpleXAcc, AddrModeSimple, AddrModeSimpleOrImm, AddrModeSimpleX, AddrModeSimpleYImm, AddrModeSimpleXImm, AddrModeSAX, AddrModeSTX, AddrModeSTY, AddrModeLAX)]
    ZeroPage(u8),

    /// Zeropage (8-bit) address, incremented by X without carry.
    #[subenum(AddrModeNoZeropageY, AddrModeNoZeropageYNoImm, AddrModeSimpleXAcc, AddrModeSimpleX, AddrModeSimpleXImm, AddrModeSTY)]
    ZeroPageX(u8),

    /// Zeropage (8-bit) address, incremented by Y without carry.
    #[subenum(AddrModeSimpleYImm, AddrModeSAX, AddrModeSTX, AddrModeLAX)]
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
