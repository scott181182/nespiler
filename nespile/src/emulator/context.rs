use std::io::{Read, Seek, SeekFrom};

use binrw::{BinRead, Error as BinError};
use bytes::Buf;



use crate::parser::opcodes::Opcode;
use super::{memory::NesMemory, opcodes::Emulate};



pub trait EmulationContext {
    fn read_reg_a(&self, ) -> u8;
    fn write_reg_a(&mut self, value: u8) -> ();

    fn read_reg_x(&self, ) -> u8;
    fn write_reg_x(&mut self, value: u8) -> ();

    fn read_reg_y(&self, ) -> u8;
    fn write_reg_y(&mut self, value: u8) -> ();

    fn read_reg_sp(&self, ) -> u8;
    fn write_reg_sp(&mut self, value: u8) -> ();
    fn read_reg_pc(&self, ) -> u16;
    fn write_reg_pc(&mut self, value: u16) -> ();

    fn read_status(&self) -> u8;
    fn write_status(&mut self, status: u8) -> ();

    fn read_flag_carry(&self, ) -> bool;
    fn write_flag_carry(&mut self, value: bool) -> ();
    fn read_flag_zero(&self, ) -> bool;
    fn write_flag_zero(&mut self, value: bool) -> ();
    fn read_flag_interrupt_disable(&self, ) -> bool;
    fn write_flag_interrupt_disable(&mut self, value: bool) -> ();
    fn read_flag_decimal_mode(&self, ) -> bool;
    fn write_flag_decimal_mode(&mut self, value: bool) -> ();
    fn read_flag_break_command(&self, ) -> bool;
    fn write_flag_break_command(&mut self, value: bool) -> ();
    fn read_flag_overflow(&self, ) -> bool;
    fn write_flag_overflow(&mut self, value: bool) -> ();
    fn read_flag_negative(&self, ) -> bool;
    fn write_flag_negative(&mut self, value: bool) -> ();

    fn read_memory_byte(&self, address: u16) -> u8;
    fn read_memory_word(&self, address: u16) -> u16;
    fn write_memory_byte(&mut self, address: u16, value: u8) -> ();
    fn write_memory_word(&mut self, address: u16, value: u16) -> ();

    fn push_stack_byte(&mut self, value: u8) -> ();
    fn pop_stack_byte(&mut self) -> u8;
    fn push_stack_word(&mut self, value: u16) -> ();
    fn pop_stack_word(&mut self) -> u16;

    fn interrupt(&mut self) -> ();
    fn return_from_interrupt(&mut self) -> ();

    fn peak_instruction(&self) -> Result<Opcode, BinError>;
    fn read_instruction(&mut self) -> Result<Opcode, BinError>;

    fn step(&mut self) -> Result<(), BinError>;
}



pub struct RealCpuContext {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub flags: u8,
}
pub struct RealEmulationContext {
    cpu: RealCpuContext,
    pub memory: NesMemory,
}
impl RealEmulationContext {
    const STACK_BOTTOM: u16 = 0x01ffu16;

    pub fn new(memory: NesMemory) -> Self {
        RealEmulationContext{
            cpu: RealCpuContext{
                a: 0, x: 0, y: 0,
                sp: 0,
                pc: 0x8000,
                flags: 0,
            },
            memory
        }
    }

    fn stack_address(&self) -> u16 {
        Self::STACK_BOTTOM - self.cpu.sp as u16
    }
}

impl Seek for RealEmulationContext {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(idx) =>
                self.cpu.pc = idx as u16,
            SeekFrom::End(rel) =>
                self.cpu.pc = (0x10000i64 + rel) as u16,
            SeekFrom::Current(rel) =>
                self.cpu.pc = self.cpu.pc.wrapping_add_signed(rel as i16),
        };
        Ok(self.cpu.pc as u64)
    }
}
impl Read for RealEmulationContext {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut idx = 0;
        loop {
            buf[idx] = self.read_memory_byte(self.cpu.pc);
            self.cpu.pc += 1;
            idx += 1;

            if (idx) >= buf.len() { return Ok(idx); }
        }
    }
}
impl EmulationContext for RealEmulationContext {
    fn read_reg_a(&self) -> u8 { self.cpu.a }
    fn write_reg_a(&mut self, value: u8) -> () { self.cpu.a = value; }

    fn read_reg_x(&self) -> u8 { self.cpu.x }
    fn write_reg_x(&mut self, value: u8) -> () { self.cpu.x = value; }

    fn read_reg_y(&self) -> u8 { self.cpu.y }
    fn write_reg_y(&mut self, value: u8) -> () { self.cpu.y = value; }

    fn read_reg_sp(&self) -> u8 { self.cpu.sp }
    fn write_reg_sp(&mut self, value: u8) -> () { self.cpu.sp = value; }

    fn read_reg_pc(&self) -> u16 { self.cpu.pc }
    fn write_reg_pc(&mut self, value: u16) -> () { self.cpu.pc = value; }

    fn read_status(&self) -> u8 {
        self.cpu.flags
    }
    fn write_status(&mut self, status: u8) -> () {
        self.cpu.flags = status;
    }



    fn read_flag_carry(&self) -> bool { self.cpu.flags & 0x01 > 0 }
    fn write_flag_carry(&mut self, value: bool) -> () {
        if value { self.cpu.flags |= 0x01; }
        else { self.cpu.flags &= !0x01; }
    }

    fn read_flag_zero(&self) -> bool {
        self.cpu.flags & 0x02 > 0
    }
    fn write_flag_zero(&mut self, value: bool) -> () {
        if value { self.cpu.flags |= 0x02; }
        else { self.cpu.flags &= !0x02; }
    }

    fn read_flag_interrupt_disable(&self) -> bool {
        self.cpu.flags & 0x04 > 0
    }
    fn write_flag_interrupt_disable(&mut self, value: bool) -> () {
        if value { self.cpu.flags |= 0x04; }
        else { self.cpu.flags &= !0x04; }
    }

    fn read_flag_decimal_mode(&self) -> bool {
        self.cpu.flags & 0x08 > 0
    }
    fn write_flag_decimal_mode(&mut self, value: bool) -> () {
        if value { self.cpu.flags |= 0x08; }
        else { self.cpu.flags &= !0x08; }
    }

    fn read_flag_break_command(&self) -> bool {
        self.cpu.flags & 0x10 > 0
    }
    fn write_flag_break_command(&mut self, value: bool) -> () {
        if value { self.cpu.flags |= 0x10; }
        else { self.cpu.flags &= !0x10; }
    }

    fn read_flag_overflow(&self) -> bool {
        self.cpu.flags & 0x40 > 0
    }
    fn write_flag_overflow(&mut self, value: bool) -> () {
        if value { self.cpu.flags |= 0x40; }
        else { self.cpu.flags &= !0x40; }
    }

    fn read_flag_negative(&self) -> bool {
        self.cpu.flags & 0x80 > 0
    }

    fn write_flag_negative(&mut self, value: bool) -> () {
        if value { self.cpu.flags |= 0x80; }
        else { self.cpu.flags &= !0x80; }
    }

    fn read_memory_byte(&self, address: u16) -> u8 {
        self.memory.read_byte_at(address)
    }
    fn read_memory_word(&self, address: u16) -> u16 {
        self.memory.read_word_at(address)
    }
    fn write_memory_byte(&mut self, address: u16, value: u8) -> () {
        self.memory.write_byte_at(address, value)
    }
    fn write_memory_word(&mut self, address: u16, value: u16) -> () {
        self.memory.write_word_at(address, value)
    }
    
    fn push_stack_byte(&mut self, value: u8) -> () {
        self.memory.write_byte_at(self.stack_address(), value);
        self.cpu.sp -= 1;
    }
    fn pop_stack_byte(&mut self) -> u8 {
        let res = self.memory.read_byte_at(self.stack_address());
        self.cpu.sp += 1;
        res
    }
    fn push_stack_word(&mut self, value: u16) -> () {
        self.push_stack_byte((value & 0xff) as u8);
        self.push_stack_byte((value >> 8) as u8);
    }
    fn pop_stack_word(&mut self) -> u16 {
        let hb = self.pop_stack_byte();
        let lb = self.pop_stack_byte();
        return ((hb as u16) << 8) | lb as u16;
    }

    fn interrupt(&mut self) -> () {
        // Save current context.
        self.push_stack_word(self.read_reg_pc());
        self.push_stack_byte(self.read_status());
        // Load interrupt handler.
        self.write_reg_pc(self.read_memory_word(0xfffe));
        self.write_flag_break_command(true);
    }
    fn return_from_interrupt(&mut self) -> () {
        let status = self.pop_stack_byte();
        let pc = self.pop_stack_word();
        self.write_status(status);
        self.write_reg_pc(pc);
    }

    fn peak_instruction(&self) -> Result<Opcode, BinError> {
        let mut instr_slice = self.memory.get_bytes(self.cpu.pc, 4);
        Opcode::read(&mut instr_slice)
    }
    fn read_instruction(&mut self) -> Result<Opcode, BinError> {
        Opcode::read(self)
    }

    fn step(&mut self) -> Result<(), BinError> {
        let instr = self.read_instruction()?;
        instr.emulate(self);
        Ok(())
    }
}

