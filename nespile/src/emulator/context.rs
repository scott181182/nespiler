use super::memory::NesMemory;




pub trait EmulationContext {
    fn read_reg_a(&self, ) -> u8;
    fn write_reg_a(&mut self, value: u8) -> ();

    fn read_reg_x(&self, ) -> u8;
    fn write_reg_x(&mut self, value: u8) -> ();

    fn read_reg_y(&self, ) -> u8;
    fn write_reg_y(&mut self, value: u8) -> ();

    fn read_reg_sp(&self, ) -> u16;
    fn write_reg_sp(&mut self, value: u16) -> ();
    fn read_reg_pc(&self, ) -> u16;
    fn write_reg_pc(&mut self, value: u16) -> ();

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
}




pub struct RealEmulationContext {
    a: u8,
    x: u8,
    y: u8,
    sp: u16,
    pc: u16,
    flags: u8,

    memory: NesMemory,
}
impl EmulationContext for RealEmulationContext {
    fn read_reg_a(&self) -> u8 { self.a }
    fn write_reg_a(&mut self, value: u8) -> () { self.a = value; }

    fn read_reg_x(&self) -> u8 { self.x }
    fn write_reg_x(&mut self, value: u8) -> () { self.x = value; }

    fn read_reg_y(&self) -> u8 { self.y }
    fn write_reg_y(&mut self, value: u8) -> () { self.y = value; }

    fn read_reg_sp(&self) -> u16 { self.sp }
    fn write_reg_sp(&mut self, value: u16) -> () { self.sp = value; }

    fn read_reg_pc(&self) -> u16 { self.pc }
    fn write_reg_pc(&mut self, value: u16) -> () { self.pc = value; }

    fn read_flag_carry(&self) -> bool { self.flags & 0x01 > 0 }
    fn write_flag_carry(&mut self, value: bool) -> () {
        if value { self.flags |= 0x01; }
        else { self.flags &= !0x01; }
    }

    fn read_flag_zero(&self) -> bool {
        self.flags & 0x02 > 0
    }
    fn write_flag_zero(&mut self, value: bool) -> () {
        if value { self.flags |= 0x02; }
        else { self.flags &= !0x02; }
    }

    fn read_flag_interrupt_disable(&self) -> bool {
        self.flags & 0x04 > 0
    }
    fn write_flag_interrupt_disable(&mut self, value: bool) -> () {
        if value { self.flags |= 0x04; }
        else { self.flags &= !0x04; }
    }

    fn read_flag_decimal_mode(&self) -> bool {
        self.flags & 0x08 > 0
    }
    fn write_flag_decimal_mode(&mut self, value: bool) -> () {
        if value { self.flags |= 0x08; }
        else { self.flags &= !0x08; }
    }

    fn read_flag_break_command(&self) -> bool {
        self.flags & 0x10 > 0
    }
    fn write_flag_break_command(&mut self, value: bool) -> () {
        if value { self.flags |= 0x10; }
        else { self.flags &= !0x10; }
    }

    fn read_flag_overflow(&self) -> bool {
        self.flags & 0x40 > 0
    }
    fn write_flag_overflow(&mut self, value: bool) -> () {
        if value { self.flags |= 0x40; }
        else { self.flags &= !0x40; }
    }

    fn read_flag_negative(&self) -> bool {
        self.flags & 0x80 > 0
    }

    fn write_flag_negative(&mut self, value: bool) -> () {
        if value { self.flags |= 0x80; }
        else { self.flags &= !0x80; }
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
}

