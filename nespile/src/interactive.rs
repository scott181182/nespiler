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
    PrintZeroPage,
    PrintContext,
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
        } else if value == "p" {
            Ok(PromptIntent::PrintContext)
        } else if value == "zp" {
            Ok(PromptIntent::PrintZeroPage)  
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
        print_context(&self.ctx);

        loop {
            let input = self.prompt_input()?;

            match input {
                PromptIntent::Quit => return Ok(()),
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
