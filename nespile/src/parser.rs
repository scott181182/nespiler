use std::io::Cursor;

use binrw::{BinRead, Error as BinError};
use thiserror::Error;

use opcodes::Operation;
use rom::NesFile;



pub mod address_mode;
pub mod rom;
pub mod opcodes;



#[derive(Error, Debug)]
pub enum ProgramParseError {
    #[error(transparent)]
    BinRead(#[from] BinError),
}

pub struct NesProgram {
    pub operations: Vec<Operation>
}
impl TryFrom<&NesFile> for NesProgram {
    type Error = ProgramParseError;

    fn try_from(file: &NesFile) -> Result<Self, Self::Error> {
        let size = file.header.prgrom_size as u64;
        let data = file.prgrom_data.clone();
        let mut cursor = Cursor::new(data);

        let mut operations = Vec::new();
        while cursor.position() < size {
            operations.push(Operation::read(&mut cursor)?)
        }

        Ok(NesProgram{ operations })
    }
}

impl NesProgram {
    pub fn to_source_string(&self, total_size: usize) -> String {
        let mut addr = 0x8000usize;
        let address_width = (((total_size + addr) as f32).log2() / 4f32).ceil() as usize;

        self.operations.iter()
            .map(|op| {
                let line = format!("${:0width$x}    {}", addr, op.to_source_string(), width = address_width);
                addr += 1 + op.arguments.size();
                return line;
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
