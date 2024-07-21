use std::{fs::File, io::Write};

use clap::{Parser, command};
use tracer::{program_to_source_string, trace_program};



mod parser;
mod tracer;
mod emulator;

use parser::rom::parse_rom;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Program {
    /// Path to ROM to parse
    rom_path: String,

    /// Path to output the parsed program to (optional)
    #[arg(short, long)]
    output_path: Option<String>,
}

fn main() {
    let args = Program::parse();

    let rom = parse_rom(&args.rom_path)
        .expect("Failed to parse ROM file");
    // println!("{:?}", rom);

    let program = trace_program(rom.prgrom_data) 
        .expect("Error parsing prgrom");


    if let Some(output_path) = args.output_path {
        let mut output_file = File::create(output_path)
            .expect("Failed to create output file");
        write!(output_file, "{}\n", program_to_source_string(program))
            .expect("Failed to write to output file");
    }
}
