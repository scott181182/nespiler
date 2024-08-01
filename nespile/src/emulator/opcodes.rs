use thiserror::Error;

use crate::parser::address_mode::*;
use crate::parser::opcodes::Opcode;
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
        let (x1, o1) = lhs.overflowing_add(rhs);
        let (res, o2) = x1.overflowing_add(if carry { 1 } else { 0 });

        let cout = o1 || o2;
        let overflow =
            (lhs ^ rhs) & 0x80 == 0 &&
            (lhs ^ res) & 0x80 != 0;
    
        ArithmeticResult::new_with_flags(res, Some(cout), Some(overflow))
    }
    pub fn carrying_subtract(lhs: u8, rhs: u8, carry: bool) -> ArithmeticResult {
        // Referenced from Tetanes.
        let (x1, o1) = lhs.overflowing_sub(rhs);
        let (res, o2) = x1.overflowing_sub(if !carry { 1 } else { 0 });

        let cout = !(o1 || o2);
        let overflow =
            (lhs ^ rhs) & 0x80 != 0 &&
            (lhs ^ res) & 0x80 != 0;
    
        ArithmeticResult::new_with_flags(res, Some(cout), Some(overflow))
    }

    pub fn logical_and(lhs: u8, rhs: u8) -> ArithmeticResult {
        let res = lhs & rhs;
        ArithmeticResult::new(res)
    }

    pub fn arithmetic_shift_left(lhs: u8) -> Self {
        let carry = (lhs & 0x80) > 0;
        ArithmeticResult::new_with_flags(lhs << 1, Some(carry), None)
    }
    pub fn logical_shift_right(lhs: u8) -> Self {
        let carry = (lhs & 1) > 0;
        ArithmeticResult::new_with_flags(lhs >> 1, Some(carry), None)
    }
    pub fn rotate_left(lhs: u8, carry: bool) -> Self {
        let new_carry = (lhs & 0x80) > 0;
        let rotate_in = if carry { 1 } else { 0 };
        let res = (lhs << 1) | rotate_in;
        ArithmeticResult::new_with_flags(res, Some(new_carry), None)
    }
    pub fn rotate_right(lhs: u8, carry: bool) -> Self {
        let new_carry = (lhs & 1) > 0;
        let rotate_in = if carry { 0x80 } else { 0 };
        let res = (lhs >> 1) | rotate_in;
        ArithmeticResult::new_with_flags(res, Some(new_carry), None)
    }

    pub fn compare(lhs: u8, rhs: u8) -> Self {
        let (res, overflow) = lhs.overflowing_sub(rhs);
        ArithmeticResult::new_with_flags(res, Some(overflow), None)
    }
    pub fn decrement(lhs: u8) -> Self {
        ArithmeticResult::new(lhs.wrapping_sub(1))
    }
    pub fn increment(lhs: u8) -> ArithmeticResult {
        ArithmeticResult::new(lhs.wrapping_add(1))
    }
}

fn unwrap_address_mode_address<T: Into<AddressMode> + Copy>(addr: &T, ctx: &dyn EmulationContext) -> u16 {
    let addr_mode: AddressMode = (*addr).into();
    addr_mode.address_argument(ctx).unwrap()
}
fn unwrap_address_mode_data<T: Into<AddressMode> + Copy>(addr: &T, ctx: &dyn EmulationContext) -> u8 {
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
                    unwrap_address_mode_data(addr, ctx),
                    ctx.read_flag_carry()
                ).apply_flags(ctx);
                ctx.write_reg_a(res);
            },
            Opcode::AND(addr) => {
                let res = ArithmeticResult::logical_and(
                    ctx.read_reg_a(),
                    unwrap_address_mode_data(addr, ctx)
                ).apply_flags(ctx);
                ctx.write_reg_a(res);
            },
            Opcode::ASL(addr) => {
                let res = ArithmeticResult::arithmetic_shift_left(
                    unwrap_address_mode_data(addr, ctx)
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
                    unwrap_address_mode_data(addr, ctx)
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
                println!("Interrupt!");
                ctx.interrupt();
            },
            Opcode::BVC(AddrModeRelative::Relative(rel)) =>
                if !ctx.read_flag_overflow() {
                    ctx.write_reg_pc(ctx.read_reg_pc().wrapping_add_signed(*rel as i16))
                },
            Opcode::BVS(AddrModeRelative::Relative(rel)) =>
                if ctx.read_flag_overflow() {
                    ctx.write_reg_pc(ctx.read_reg_pc().wrapping_add_signed(*rel as i16))
                },
            Opcode::CLC => ctx.write_flag_carry(false),
            Opcode::CLD => ctx.write_flag_decimal_mode(false),
            Opcode::CLI => ctx.write_flag_interrupt_disable(false),
            Opcode::CLV => ctx.write_flag_overflow(false),
            Opcode::CMP(addr) => {
                ArithmeticResult::compare(
                    ctx.read_reg_a(),
                    unwrap_address_mode_data(addr, ctx)
                ).apply_flags(ctx);
            },
            Opcode::CPX(addr) => {
                ArithmeticResult::compare(
                    ctx.read_reg_x(),
                    unwrap_address_mode_data(addr, ctx)
                ).apply_flags(ctx);
            },
            Opcode::CPY(addr) => {
                ArithmeticResult::compare(
                    ctx.read_reg_y(),
                    unwrap_address_mode_data(addr, ctx)
                ).apply_flags(ctx);
            },
            Opcode::DEC(addr) => {
                let addr = unwrap_address_mode_address(addr, ctx);
                let res = ArithmeticResult::decrement(
                    ctx.read_memory_byte(addr),
                ).apply_flags(ctx);
                ctx.write_memory_byte(addr, res);
            },
            Opcode::DEX => {
                let res = ArithmeticResult::decrement(
                    ctx.read_reg_x(),
                ).apply_flags(ctx);
                ctx.write_reg_x(res);
            },
            Opcode::DEY => {
                let res = ArithmeticResult::decrement(
                    ctx.read_reg_y(),
                ).apply_flags(ctx);
                ctx.write_reg_y(res);
            },
            Opcode::EOR(addr) => {
                let res = ArithmeticResult::new(
                    ctx.read_reg_a() ^ unwrap_address_mode_data(addr, ctx)
                ).apply_flags(ctx);
                ctx.write_reg_a(res);
            },
            Opcode::INC(addr) => {
                let addr = unwrap_address_mode_address(addr, ctx);
                let res = ArithmeticResult::increment(
                    ctx.read_memory_byte(addr),
                ).apply_flags(ctx);
                ctx.write_memory_byte(addr, res);
            },
            Opcode::INX => {
                let res = ArithmeticResult::increment(
                    ctx.read_reg_x(),
                ).apply_flags(ctx);
                ctx.write_reg_x(res);
            },
            Opcode::INY => {
                let res = ArithmeticResult::increment(
                    ctx.read_reg_y(),
                ).apply_flags(ctx);
                ctx.write_reg_y(res);
            },
            Opcode::JMP(addr) => {
                let addr = unwrap_address_mode_address(addr, ctx);
                ctx.write_reg_pc(addr);
            },
            Opcode::JSR(addr) => {
                let return_point = ctx.read_reg_pc() + (self.size() as u16);
                ctx.push_stack_word(return_point - 1);
                ctx.write_reg_pc(unwrap_address_mode_address(addr, ctx));
            },
            Opcode::LDA(addr) => {
                let res = unwrap_address_mode_data(addr, ctx);
                let res = ArithmeticResult::new(res).apply_flags(ctx);
                ctx.write_reg_a(res);
            },
            Opcode::LDX(addr) => {
                let res = unwrap_address_mode_data(addr, ctx);
                let res = ArithmeticResult::new(res).apply_flags(ctx);
                ctx.write_reg_x(res);
            },
            Opcode::LDY(addr) => {
                let res = unwrap_address_mode_data(addr, ctx);
                let res = ArithmeticResult::new(res).apply_flags(ctx);
                ctx.write_reg_y(res);
            },
            Opcode::LSR(addr) => {
                let res = ArithmeticResult::logical_shift_right(
                    unwrap_address_mode_data(addr, ctx)
                ).apply_flags(ctx);
                // TODO: make this a method on AddressMode.
                match addr {
                    AddrModeSimpleXAcc::Accumulator => 
                        ctx.write_reg_a(res),
                    am =>
                        ctx.write_memory_byte(unwrap_address_mode_address(am, ctx), res),
                }
            },
            Opcode::NOP => {},
            Opcode::ORA(addr) => {
                let res = ArithmeticResult::new(
                    ctx.read_reg_a() | unwrap_address_mode_data(addr, ctx)
                ).apply_flags(ctx);
                ctx.write_reg_a(res);
            },
            Opcode::PHA => ctx.push_stack_byte(ctx.read_reg_a()),
            Opcode::PHP => ctx.push_stack_byte(ctx.read_status()),
            Opcode::PLA => {
                let res = ctx.pop_stack_byte();
                let res = ArithmeticResult::new(res).apply_flags(ctx);
                ctx.write_reg_a(res);
            },
            Opcode::PLP => {
                let status = ctx.pop_stack_byte();
                ctx.write_status(status);
            },
            Opcode::ROL(addr) => {
                let res = ArithmeticResult::rotate_left(
                    unwrap_address_mode_data(addr, ctx),
                    ctx.read_flag_carry(),
                ).apply_flags(ctx);
                match addr {
                    AddrModeSimpleXAcc::Accumulator => 
                        ctx.write_reg_a(res),
                    am =>
                        ctx.write_memory_byte(unwrap_address_mode_address(am, ctx), res),
                }
            },
            Opcode::ROR(addr) => {
                let res = ArithmeticResult::rotate_right(
                    unwrap_address_mode_data(addr, ctx),
                    ctx.read_flag_carry(),
                ).apply_flags(ctx);
                match addr {
                    AddrModeSimpleXAcc::Accumulator => 
                        ctx.write_reg_a(res),
                    am =>
                        ctx.write_memory_byte(unwrap_address_mode_address(am, ctx), res),
                }
            },
            Opcode::RTI => ctx.return_from_interrupt(),
            Opcode::RTS => {
                let pc = ctx.pop_stack_word();
                ctx.write_reg_pc(pc);
            },
            Opcode::SBC(addr) => {
                let res = ArithmeticResult::carrying_subtract(
                    ctx.read_reg_a(), 
                    unwrap_address_mode_data(addr, ctx), 
                    ctx.read_flag_carry()
                ).apply_flags(ctx);
                ctx.write_reg_a(res);
            },
            Opcode::SEC => ctx.write_flag_carry(true),
            Opcode::SED => ctx.write_flag_decimal_mode(true),
            Opcode::SEI => ctx.write_flag_interrupt_disable(true),
            Opcode::STA(addr) => ctx.write_memory_byte(
                unwrap_address_mode_address(addr, ctx),
                ctx.read_reg_a()
            ),
            Opcode::STX(addr) => ctx.write_memory_byte(
                unwrap_address_mode_address(addr, ctx),
                ctx.read_reg_x()
            ),
            Opcode::STY(addr) => ctx.write_memory_byte(
                unwrap_address_mode_address(addr, ctx),
                ctx.read_reg_y()
            ),
            Opcode::TAX => {
                let a = ctx.read_reg_a();
                ArithmeticResult::new(a).apply_flags(ctx);
                ctx.write_reg_x(a);
            },
            Opcode::TAY => {
                let a = ctx.read_reg_a();
                ArithmeticResult::new(a).apply_flags(ctx);
                ctx.write_reg_y(a);
            },
            Opcode::TSX => {
                let sp = ctx.read_reg_sp();
                ArithmeticResult::new(sp).apply_flags(ctx);
                ctx.write_reg_x(sp);
            },
            Opcode::TXA => {
                let x = ctx.read_reg_x();
                ArithmeticResult::new(x).apply_flags(ctx);
                ctx.write_reg_a(x);
            },
            Opcode::TXS => {
                let x = ctx.read_reg_x();
                ArithmeticResult::new(x).apply_flags(ctx);
                ctx.write_reg_sp(x);
            },
            Opcode::TYA => {
                let y = ctx.read_reg_y();
                ArithmeticResult::new(y).apply_flags(ctx);
                ctx.write_reg_a(y);
            },

            // Unofficial Opcodes
            // Will implement later.
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