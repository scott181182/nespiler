use thiserror::Error;

use crate::parser::{address_mode::{AddrModeRelative, AddressMode}, opcodes::Opcode};

use super::context::EmulationContext;



#[derive(Error, Debug)]
pub enum OpcodeArgumentError {
    #[error("Unexpected address mode: {0}")]
    InvalidAddressMode(String),
    #[error("Cannot get argument value from implied address mode")]
    ImpliedAddressMode,
}



struct ArithmeticResult {
    result: u8,
    carry: Option<bool>,
    overflow: Option<bool>,
}
impl ArithmeticResult {
    pub fn new(result: u8) -> Self {
        ArithmeticResult{ result, carry: None, overflow: None}
    }
    pub fn new_with_flags(result: u8, carry: Option<bool>, overflow: Option<bool>) -> Self {
        ArithmeticResult{ result, carry, overflow }
    }

    pub fn apply_flags(self, ctx: &mut dyn EmulationContext) -> u8 {
        if let Some(carry) = self.carry { ctx.write_flag_carry(carry); }
        if let Some(overflow) = self.overflow { ctx.write_flag_overflow(overflow); }
        ctx.write_flag_zero(self.result == 0);
        ctx.write_flag_negative(self.result & 0x80 > 0);

        self.result
    }



    pub fn carrying_add(lhs: u8, rhs: u8, carry: bool) -> ArithmeticResult {
        let to_add = if carry { rhs.wrapping_add(1u8) } else { rhs };
        let res = lhs.wrapping_add(to_add);
        let cout = res < lhs || res < rhs;
        let overflow = (lhs ^ res) & (to_add ^ lhs) & 0x80 > 0;
    
        ArithmeticResult::new_with_flags(res, Some(cout), Some(overflow))
    }
    pub fn logical_and(lhs: u8, rhs: u8) -> ArithmeticResult {
        let res = lhs & rhs;
        ArithmeticResult::new(res)
    }
    pub fn arithmetic_shift_left(lhs: u8) -> Self {
        let carry = lhs & 0x80 > 0;
        ArithmeticResult::new_with_flags(lhs << 1, Some(carry), None)
    }
}

fn unwrap_address_mode_argument<T: Into<AddressMode> + Copy>(addr: &T, ctx: &dyn EmulationContext) -> u8 {
    let addr_mode: AddressMode = (*addr).into();
    addr_mode.data_argument(ctx).unwrap()
}


pub trait Emulate {
    fn emulate(&self, ctx: &mut dyn EmulationContext) -> ();
}

impl Emulate for Opcode {
    fn emulate(&self, ctx: &mut dyn EmulationContext) {
        match self {
            Opcode::ADC(addr) => {
                let res = ArithmeticResult::carrying_add(
                    ctx.read_reg_a(),
                    unwrap_address_mode_argument(addr, ctx),
                    ctx.read_flag_carry()
                ).apply_flags(ctx);
                ctx.write_reg_a(res);
            },
            Opcode::AND(addr) => {
                let res = ArithmeticResult::logical_and(
                    ctx.read_reg_a(),
                    unwrap_address_mode_argument(addr, ctx)
                ).apply_flags(ctx);
                ctx.write_reg_a(res);
            },
            Opcode::ASL(addr) => {
                let res = ArithmeticResult::arithmetic_shift_left(
                    unwrap_address_mode_argument(addr, ctx)
                ).apply_flags(ctx);
                ctx.write_reg_a(res);
            },
            Opcode::BCC(AddrModeRelative::Relative(rel)) =>
                if !ctx.read_flag_carry() {
                    ctx.write_reg_pc(ctx.read_reg_pc().wrapping_add_signed(*rel as i16))
                },
            Opcode::BCS(AddrModeRelative::Relative(rel)) =>
                if ctx.read_flag_carry() {
                    ctx.write_reg_pc(ctx.read_reg_pc().wrapping_add_signed(*rel as i16))
                },
            Opcode::BEQ(AddrModeRelative::Relative(rel)) =>
                if ctx.read_flag_zero() {
                    ctx.write_reg_pc(ctx.read_reg_pc().wrapping_add_signed(*rel as i16))
                },
            Opcode::BIT(addr) => {
                let res = ArithmeticResult::logical_and(
                    ctx.read_reg_a(),
                    unwrap_address_mode_argument(addr, ctx)
                ).apply_flags(ctx);
                ctx.write_flag_overflow(res & 0x40 > 0);
            },
            Opcode::BMI(AddrModeRelative::Relative(rel)) =>
                if ctx.read_flag_negative() {
                    ctx.write_reg_pc(ctx.read_reg_pc().wrapping_add_signed(*rel as i16))
                },
            Opcode::BNE(AddrModeRelative::Relative(rel)) =>
                if !ctx.read_flag_zero() {
                    ctx.write_reg_pc(ctx.read_reg_pc().wrapping_add_signed(*rel as i16))
                },
            Opcode::BPL(AddrModeRelative::Relative(rel)) =>
                if !ctx.read_flag_negative() {
                    ctx.write_reg_pc(ctx.read_reg_pc().wrapping_add_signed(*rel as i16))
                },
            Opcode::BRK => {
                // Interrupt!
                
            },
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
        }
    }
}