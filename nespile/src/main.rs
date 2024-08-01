use std::{fs::File, io::Write};

use clap::{command, Parser, Subcommand};
use emulator::{context::RealEmulationContext, memory::NesMemory};
use thiserror::Error;
use tracer::{program_to_source_string, trace_program};



mod parser;
mod tracer;
mod emulator;
mod interactive;
mod utils;

use parser::rom::{parse_rom, NesFile};
use utils::TrimInPlace;



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Program {
    #[command(subcommand)]
    command: NespileSubcommand,

    /// Path to ROM to parse
    rom_path: String,
}

#[derive(Debug, Subcommand)]
enum NespileSubcommand {
    Compile{
        /// Path to output the parsed program to (optional)
        #[arg(short, long)]
        output_path: String,
    },
    Step
}



fn main() {
    let args = Program::parse();

    let rom = parse_rom(&args.rom_path)
        .expect("Failed to parse ROM file");


    match args.command {
        NespileSubcommand::Compile{ output_path } => {
            let program = trace_program(rom.prgrom_data) 
                .expect("Error parsing prgrom");

            let mut output_file = File::create(output_path)
                .expect("Failed to create output file");
            write!(output_file, "{}\n", program_to_source_string(program))
                .expect("Failed to write to output file");
        },
        NespileSubcommand::Step => {
            interactive::interactive_step(rom).expect("Unexpected Failure");
        },
    }
}
