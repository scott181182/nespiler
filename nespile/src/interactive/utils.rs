use crate::emulator::context::{EmulationContext, RealEmulationContext};



pub fn print_context(ctx: &RealEmulationContext) {
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