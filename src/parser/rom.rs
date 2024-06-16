use std::{fs, io};
use std::path::Path;

use binrw::{binread, BinRead, Error as BinError};
use modular_bitfield::{bitfield, BitfieldSpecifier};
use modular_bitfield::specifiers::{B2, B4, B7};
use thiserror::Error;



const PRGROM_SIZE_SHIFT: usize = 14;
const CHRROM_SIZE_SHIFT: usize = 13;



#[derive(Debug)]
pub enum NesFormatVersion {
    ArchaicINes,
    INES,
    NES2_0,
    Unknown,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 1]
pub enum NametableArrangement { Vertical, Horizontal }
#[derive(BitfieldSpecifier, Debug)]
#[bits = 1]
pub enum TVSystem { NTSC, PAL }

#[bitfield]
#[derive(BinRead, Debug)]
#[br(map = Self::from_bytes)]
pub struct INesHeaderFlags6 {
    pub nametable_arrangement: NametableArrangement,
    pub persistent_memory: bool,
    pub has_trainer: bool,
    pub alt_nametable_layout: bool,
    pub mapper_lower: B4
}
#[bitfield]
#[derive(BinRead, Clone, Debug)]
#[br(map = Self::from_bytes)]
pub struct INesHeaderFlags7 {
    pub vs_unisystem: bool,
    pub playchoice_10: bool,
    pub nes2_indicator: B2,
    pub mapper_upper: B4
}
#[bitfield]
#[derive(BinRead, Clone, Debug)]
#[br(map = Self::from_bytes)]
pub struct INesHeaderFlags9 {
    pub tv_system: TVSystem,
    #[skip]
    reserved: B7
}


#[binread]
#[derive(Debug)]
#[br(little, magic = b"NES\x1a")]
pub struct NesHeader {
    #[br(map = |x: u8| (x as usize) << PRGROM_SIZE_SHIFT)]
    pub prgrom_size: usize,
    #[br(map = |x: u8| (x as usize) << CHRROM_SIZE_SHIFT)]
    pub chrrom_size: usize,

    pub flags6: INesHeaderFlags6,
    pub flags7: INesHeaderFlags7,
    
    #[br(map = |x: u8| (x as usize) << CHRROM_SIZE_SHIFT)]
    prgram_size: usize,
    
    pub flags9: INesHeaderFlags9,

    unused: [u8; 6],



    #[br(calc = match flags7.clone().into_bytes()[0] {
        f if f & 0x0c == 0x04 => NesFormatVersion::ArchaicINes,
        f if f & 0x0c == 0x00 => NesFormatVersion::INES,
        f if f & 0x0c == 0x08 => NesFormatVersion::NES2_0,
        _ => NesFormatVersion::Unknown
    })]
    pub version: NesFormatVersion
}

#[binread]
#[br(little)]
pub struct NesFile {
    pub header: NesHeader,

    #[br(if(header.flags6.has_trainer()), count = 512)]
    pub trainer: Option<Vec<u8>>,

    #[br(count = header.prgrom_size)]
    pub prgrom_data: Vec<u8>,
    #[br(count = header.chrrom_size)]
    pub chrrom_data: Vec<u8>,
}

impl std::fmt::Debug for NesFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "NesFile(\n\theader={:?},\n\ttrainer=#{}\n\tprgrom=#{}\n\tchrrom=#{}\n)",
            &self.header,
            if let Some(t) = &self.trainer { t.len() } else { 0 },
            self.prgrom_data.len(),
            self.chrrom_data.len()
        )
    }
}


#[derive(Error, Debug)]
pub enum ParserError {
    #[error(transparent)]
    FS(#[from] io::Error),

    #[error(transparent)]
    Header(#[from] BinError)
}


pub fn parse_rom<P: AsRef<Path>>(rom_path: P) -> Result<NesFile, ParserError> {
    let rom_file = fs::File::open(rom_path)?;
    let mut reader = io::BufReader::new(rom_file);
    let header = NesFile::read(&mut reader)?;

    Ok(header)
}