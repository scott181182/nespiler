use bytes::{Bytes, BytesMut};



pub struct NesMemory {
    pub zeropage: BytesMut,
    pub stack: BytesMut,
    pub prgrom: Bytes
}
impl NesMemory {
    pub fn new(prgrom: Bytes) -> Self {
        NesMemory{
            zeropage: BytesMut::zeroed(0x100),
            stack: BytesMut::zeroed(0x100),
            prgrom
        }
    }



    pub fn read_byte_at(&self, address: u16) -> u8 {
        let address = address as usize;
        if address < 0x100 {
            self.zeropage[address]
        } else if address < 0x200 {
            self.stack[address - 0x100]
        } else if address >= 0x8000 {
            self.prgrom[address - 0x8000]
        } else {
            println!("Access at {:04x}", address);
            0
        }
    }
    pub fn write_byte_at(&mut self, address: u16, value: u8) -> () {
        let address = address as usize;
        if address < 0x100 {
            self.zeropage[address] = value;
        } else if address < 0x200 {
            self.stack[address - 0x100] = value;
        } else {
            println!("Invalid write at {:04x}", address);
        }
    }

    pub fn read_word_at(&self, address: u16) -> u16 {
        let lb = self.read_byte_at(address) as u16;
        let hb = self.read_byte_at(address + 1) as u16;
        (hb << 8) | lb
    }
    pub fn write_word_at(&mut self, address: u16, value: u16) -> () {
        self.write_byte_at(address, (value & 0xff) as u8);
        self.write_byte_at(address + 1, (value >> 8) as u8);
    }
}