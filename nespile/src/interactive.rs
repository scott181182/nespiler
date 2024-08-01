use std::io::{Seek, SeekFrom, Write};
use std::str::FromStr;

use binrw::Error as BinError;
use thiserror::Error;



use crate::emulator::context::{EmulationContext, RealEmulationContext};
use crate::emulator::memory::NesMemory;
use crate::parser::rom::NesFile;
use crate::utils::TrimInPlace;



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



enum PromptIntent {
    Quit,
    Step(usize),
    Goto(u16),
    PrintRange(u16, u16),
    None,
}
#[derive(Error, Debug)]
pub enum IntentParseError {
    #[error("Malformed range start: {0}")]
    MalformedRangeStart(String),
    #[error("Malformed range end: {0}")]
    MalformedRangeEnd(String),
    #[error("Malformed goto address: {0}")]
    MalformedGoto(String),
}
impl FromStr for PromptIntent {
    type Err = IntentParseError;

    fn from_str(value: &str) -> Result<Self, IntentParseError> {
        if value == "q" {
            Ok(PromptIntent::Quit)
        } else if value.starts_with("$") {
            if let Some((lhs, rhs)) = value.split_once(":") {
                let start = u16::from_str_radix(&lhs[1..], 16)
                    .map_err(|_err| IntentParseError::MalformedRangeStart(lhs.to_string()))?;

                let end = if rhs.starts_with("+") {
                    start + u16::from_str_radix(&rhs[1..], 10)
                        .map_err(|_err| IntentParseError::MalformedRangeEnd(rhs.to_string()))?
                } else {
                    u16::from_str_radix(&rhs, 16)
                        .map_err(|_err| IntentParseError::MalformedRangeStart(lhs.to_string()))?
                };

                Ok(PromptIntent::PrintRange(start, end))
            } else {
                if let Ok(start) = u16::from_str_radix(&value[1..], 16) {
                    Ok(PromptIntent::PrintRange(start, start + 1))
                } else {
                    Err(IntentParseError::MalformedRangeStart(value.to_string()))
                }
            }
        } else if value.starts_with(">") {
            let addr = u16::from_str_radix(&value[1..], 16)
                .map_err(|_err| IntentParseError::MalformedGoto(value.to_string()))?;
            Ok(PromptIntent::Goto(addr))
        } else if let Ok(step) = usize::from_str(&value) {
            Ok(PromptIntent::Step(step))
        } else {
            Ok(PromptIntent::None)
        }
    }
}



struct InteractiveSession {
    ctx: RealEmulationContext,
    input_buffer: String,
}

fn print_context(ctx: &RealEmulationContext) {
    let a = ctx.read_reg_a();
    let x = ctx.read_reg_x();
    let y = ctx.read_reg_y();
    let sp = ctx.read_reg_sp();
    let pc = ctx.read_reg_pc();

    let flags = ctx.read_status();

    let instr = ctx.peak_instruction();
    let instr_str = instr.map_or("None".to_string(), |op| {
        let memstr = (0..op.size())
            .map(|idx| ctx.read_memory_byte(ctx.read_reg_pc() + idx as u16))
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join("");

        format!("{}  {:?}", memstr, op)
    });

    println!("");
    println!("Registers:  a:{:02x}  x:{:02x}  y:{:02x}    sp:{:02x}  pc:{:02x}", a, x, y, sp, pc);
    println!("Flags:      {:08b}", flags);
    println!("Next Instr: {}", instr_str);
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
        loop {
            print_context(&self.ctx);

            let input = self.prompt_input()?;

            match input {
                PromptIntent::Quit => return Ok(()),
                PromptIntent::Step(step_size) => {
                    for _ in 0..step_size {
                        self.ctx.step()?
                    }
                },
                PromptIntent::Goto(pc) => {
                    while self.ctx.read_reg_pc() != pc {
                        self.ctx.step()?;
                    }
                },
                PromptIntent::PrintRange(start, end) => {
                    let range_str = (start..end)
                        .map(|addr| self.ctx.read_memory_byte(addr))
                        .map(|byte| format!("{:02x}", byte))
                        .collect::<Vec<String>>()
                        .join(" ");
                    println!("${:04x}-{:04x}: {}", start, end, range_str);
                },
                PromptIntent::None => {
                    self.ctx.step()?;
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
}


pub fn interactive_step(rom: NesFile) -> Result<(), InteractiveError> {
    let session = InteractiveSession::new(rom);
    session.start()
}
