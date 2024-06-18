use binrw::BinRead;

use nespile_macros::{parse_byte_with, OpcodeArgs, VariantNames};

use crate::parser::address_mode::*;



#[derive(Debug, VariantNames, OpcodeArgs)]
#[parse_byte_with(
    BRK, ORA, STP, SLO, NOP, ORA, ASL, SLO, PHP, ORA, ASL, ANC, NOP, ORA, ASL, SLO, 
    BPL, ORA, STP, SLO, NOP, ORA, ASL, SLO, CLC, ORA, NOP, SLO, NOP, ORA, ASL, SLO, 
    JSR, AND, STP, RLA, BIT, AND, ROL, RLA, PLP, AND, ROL, ANC, BIT, AND, ROL, RLA, 
    BMI, AND, STP, RLA, NOP, AND, ROL, RLA, SEC, AND, NOP, RLA, NOP, AND, ROL, RLA, 
    RTI, EOR, STP, SRE, NOP, EOR, LSR, SRE, PHA, EOR, LSR, ALR, JMP, EOR, LSR, SRE, 
    BVC, EOR, STP, SRE, NOP, EOR, LSR, SRE, CLI, EOR, NOP, SRE, NOP, EOR, LSR, SRE, 
    RTS, ADC, STP, RRA, NOP, ADC, ROR, RRA, PLA, ADC, ROR, ARR, JMP, ADC, ROR, RRA, 
    BVS, ADC, STP, RRA, NOP, ADC, ROR, RRA, SEI, ADC, NOP, RRA, NOP, ADC, ROR, RRA, 

    NOP, STA, NOP, SAX, STY, STA, STX, SAX, DEY, NOP, TXA, XAA, STY, STA, STX, SAX, 
    BCC, STA, STP, AHX, STY, STA, STX, SAX, TYA, STA, TXS, TAS, SHY, STA, SHX, AHX, 
    LDY, LDA, LDX, LAX, LDY, LDA, LDX, LAX, TAY, LDA, TAX, LXA, LDY, LDA, LDX, LAX, 
    BCS, LDA, STP, LAX, LDY, LDA, LDX, LAX, CLV, LDA, TSX, LAS, LDY, LDA, LDX, LAX, 
    CPY, CMP, NOP, DCP, CPY, CMP, DEC, DCP, INY, CMP, DEX, AXS, CPY, CMP, DEC, DCP, 
    BNE, CMP, STP, DCP, NOP, CMP, DEC, DCP, CLD, CMP, NOP, DCP, NOP, CMP, DEC, DCP, 
    CPX, SBC, NOP, ISC, CPX, SBC, INC, ISC, INX, SBC, NOP, SBC, CPX, SBC, INC, ISC, 
    BEQ, SBC, STP, ISC, NOP, SBC, INC, ISC, SED, SBC, NOP, ISC, NOP, SBC, INC, ISC, 
)]
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
    /// LAX = LDA + LDX
    LAX(AddrModeLAX),
    /// (A OR CONST) AND oper -> A -> X
    LXA(AddrModeImmediate),
    /// RLA = ROL + AND
    RLA(AddrModeNoZeropageYNoImm),
    /// RRA = ROR + ADC
    RRA(AddrModeNoZeropageYNoImm),
    /// a.k.a. AXS, AAX
    /// 
    /// A AND X -> M
    SAX(AddrModeSAX),
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



impl Opcode {
    pub fn to_source_string(&self) -> String {
        if let Some(addr_mode) = self.argument() {
            format!("{:?}   {}", self.variant_name(), addr_mode)
        } else {
            format!("{:?}", self.variant_name())
        }
    }
}
