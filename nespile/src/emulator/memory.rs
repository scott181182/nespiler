use std::io::Cursor;

use bytes::{Bytes, BytesMut};



const INTERNAL_RAM_SIZE: usize = 0x0800;
const PROGRAM_RAM_START: usize = 0x8000;



pub struct NesMemory {
    pub internal_ram: BytesMut,
    pub prgrom: Bytes,
}
impl NesMemory {
    pub fn new(prgrom: Bytes) -> Self {
        NesMemory{
            internal_ram: BytesMut::zeroed(INTERNAL_RAM_SIZE),
            prgrom,
        }
    }



    pub fn read_byte_at(&self, address: u16) -> u8 {
        let address = address as usize;
        if address < INTERNAL_RAM_SIZE {
            self.internal_ram[address]
        } else if address >= PROGRAM_RAM_START {
            self.prgrom[address - PROGRAM_RAM_START]
        } else {
            println!("Access at {:04x}", address);
            0
        }
    }
    pub fn write_byte_at(&mut self, address: u16, value: u8) -> () {
        let address = address as usize;
        if address < INTERNAL_RAM_SIZE {
            self.internal_ram[address] = value;
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

    pub fn get_bytes(&self, address: u16, len: u16) -> Cursor<Bytes> {
        let slice = (address..(address + len))
            .map(|addr| self.read_byte_at(addr))
            .collect::<Bytes>();
        Cursor::new(slice)
    }
}
