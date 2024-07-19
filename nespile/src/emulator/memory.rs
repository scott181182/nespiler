


pub struct NesMemory {
    inner: [u8; 0x10000],
}
impl NesMemory {
    pub fn read_byte_at(&self, address: u16) -> u8 {
        self.inner[address as usize]
    }
    pub fn write_byte_at(&mut self, address: u16, value: u8) -> () {
        self.inner[address as usize] = value;
    }

    pub fn read_word_at(&self, address: u16) -> u16 {
        let lb = self.inner[address as usize] as u16;
        let hb = self.inner[(address + 1) as usize] as u16;
        (hb << 8) | lb
    }
}