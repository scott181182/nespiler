use crate::parser::opcodes::Opcode;

use super::{TraceContext, TraceError};




pub trait Traceable {
    fn trace(&self, ctx: &mut TraceContext) -> Result<(), TraceError>;
}

impl Traceable for Opcode {
    fn trace(&self, ctx: &mut TraceContext) -> Result<(), TraceError> {
        
        match self {
            Opcode::ADC(_) => todo!(),
            
            Opcode::AND(_) => todo!(),
            Opcode::ASL(_) => todo!(),
            Opcode::BCC(_) => todo!(),
            Opcode::BCS(_) => todo!(),
            Opcode::BEQ(_) => todo!(),
            Opcode::BIT(_) => todo!(),
            Opcode::BMI(_) => todo!(),
            Opcode::BNE(_) => todo!(),
            Opcode::BPL(_) => todo!(),
            Opcode::BRK => todo!(),
            Opcode::BVC(_) => todo!(),
            Opcode::BVS(_) => todo!(),
            Opcode::CLC => todo!(),
            Opcode::CLD => todo!(),
            Opcode::CLI => todo!(),
            Opcode::CLV => todo!(),
            Opcode::CMP(_) => todo!(),
            Opcode::CPX(_) => todo!(),
            Opcode::CPY(_) => todo!(),
            Opcode::DEC(_) => todo!(),
            Opcode::DEX => todo!(),
            Opcode::DEY => todo!(),
            Opcode::EOR(_) => todo!(),
            Opcode::INC(_) => todo!(),
            Opcode::INX => todo!(),
            Opcode::INY => todo!(),
            Opcode::JMP(_) => todo!(),
            Opcode::JSR(_) => todo!(),
            Opcode::LDA(_) => todo!(),
            Opcode::LDX(_) => todo!(),
            Opcode::LDY(_) => todo!(),
            Opcode::LSR(_) => todo!(),
            Opcode::NOP => todo!(),
            Opcode::ORA(_) => todo!(),
            Opcode::PHA => todo!(),
            Opcode::PHP => todo!(),
            Opcode::PLA => todo!(),
            Opcode::PLP => todo!(),
            Opcode::ROL(_) => todo!(),
            Opcode::ROR(_) => todo!(),
            Opcode::RTI => todo!(),
            Opcode::RTS => todo!(),
            Opcode::SBC(_) => todo!(),
            Opcode::SEC => todo!(),
            Opcode::SED => todo!(),
            Opcode::SEI => todo!(),
            Opcode::STA(_) => todo!(),
            Opcode::STX(_) => todo!(),
            Opcode::STY(_) => todo!(),
            Opcode::TAX => todo!(),
            Opcode::TAY => todo!(),
            Opcode::TSX => todo!(),
            Opcode::TXA => todo!(),
            Opcode::TXS => todo!(),
            Opcode::TYA => todo!(),
            Opcode::AHX(_) => todo!(),
            Opcode::ALR(_) => todo!(),
            Opcode::ANC(_) => todo!(),
            Opcode::ARR(_) => todo!(),
            Opcode::AXS(_) => todo!(),
            Opcode::DCP(_) => todo!(),
            Opcode::ISC(_) => todo!(),
            Opcode::LAS(_) => todo!(),
            Opcode::LAX(_) => todo!(),
            Opcode::LXA(_) => todo!(),
            Opcode::RLA(_) => todo!(),
            Opcode::RRA(_) => todo!(),
            Opcode::SAX(_) => todo!(),
            Opcode::SHX(_) => todo!(),
            Opcode::SHY(_) => todo!(),
            Opcode::SLO(_) => todo!(),
            Opcode::SRE(_) => todo!(),
            Opcode::TAS(_) => todo!(),
            Opcode::XAA(_) => todo!(),
            Opcode::STP => todo!(),

            _ => Ok(())
        }
    }
}