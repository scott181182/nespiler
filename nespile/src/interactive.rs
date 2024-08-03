use std::io::Write;
use std::str::FromStr;

use binrw::Error as BinError;
use thiserror::Error;



use crate::emulator::context::{EmulationContext, RealEmulationContext};
use crate::emulator::memory::NesMemory;
use crate::parser::rom::NesFile;
use crate::utils::TrimInPlace;

mod prompt;
mod utils;
use prompt::{PromptIntent, IntentParseError};
use utils::print_context;



#[derive(Error, Debug)]
pub enum InteractiveError {
    #[error(transparent)]
    PromptIo(std::io::Error),
    #[error(transparent)]
    PromptParse(#[from] IntentParseError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    OpcodeParse(#[from] BinError),
}



struct InteractiveSession {
    ctx: RealEmulationContext,
    input_buffer: String,
}




impl InteractiveSession {
    pub fn new(rom: NesFile) -> Self {
        let memory = NesMemory::new(rom.prgrom_data.into());
        let ctx = RealEmulationContext::new(memory);

        InteractiveSession{
            ctx,
            input_buffer: String::with_capacity(8)
        }
    }

    pub fn start(mut self) -> Result<(), InteractiveError> {
        print_context(&self.ctx);

        loop {
            let input = self.prompt_input()?;

            match input {
                PromptIntent::Quit => return Ok(()),
                PromptIntent::Run => {
                    // Run until something unexpected happens.
                    loop { self.ctx.step()?; }
                }
                PromptIntent::PrintContext => print_context(&self.ctx),
                PromptIntent::Step(step_size) => {
                    for _ in 0..step_size {
                        self.ctx.step()?
                    }
                    print_context(&self.ctx);
                },
                PromptIntent::Goto(pc) => {
                    while self.ctx.read_reg_pc() != pc {
                        self.ctx.step()?;
                    }
                    print_context(&self.ctx);
                },
                PromptIntent::PrintRange(start, end) =>
                    self.print_memory_block(start, end),
                PromptIntent::PrintZeroPage =>
                    self.print_memory_block(0, 0x100),
                PromptIntent::None => {
                    self.ctx.step()?;
                    print_context(&self.ctx);
                },
            }
        }
    }

    fn prompt_input(&mut self) -> Result<PromptIntent, InteractiveError> {
        let mut stdout = std::io::stdout();
        stdout.write(b"Press <enter> to step: ").map_err(InteractiveError::PromptIo)?;
        stdout.flush().map_err(InteractiveError::PromptIo)?;

        self.input_buffer.clear();
        let stdin = std::io::stdin();
        stdin.read_line(&mut self.input_buffer).map_err(InteractiveError::PromptIo)?;
        self.input_buffer.trim_in_place();

        Ok(PromptIntent::from_str(self.input_buffer.as_ref())?)
    }

    fn print_memory_block(&self, start: u16, end: u16) {
        const LINE_SIZE: u16 = 16;
        let aligned_start = start & !(LINE_SIZE - 1);
        let aligned_length = end - aligned_start;
        let total_lines = (aligned_length as f32 / LINE_SIZE as f32).ceil() as u16;

        let data_block = (0..total_lines)
            .map(|line_no| {
                let addr = aligned_start + line_no * LINE_SIZE;

                let line_text = (addr..(addr + LINE_SIZE))
                    .map(|a| {
                        if a < start || a >= end {
                            "  ".to_string()
                        } else {
                            format!("{:02x}", self.ctx.read_memory_byte(a))
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ");

                format!("${:04x} {}", addr, line_text)
            })
            .collect::<Vec<String>>()
            .join("\n");

        let header = (0..LINE_SIZE)
            .map(|idx| format!("{:02x}", idx))
            .collect::<Vec<String>>()
            .join(" ");

        println!("      {}\n{}", header, data_block);
    }
}


pub fn interactive_step(rom: NesFile) -> Result<(), InteractiveError> {
    let session = InteractiveSession::new(rom);
    session.start()
}
