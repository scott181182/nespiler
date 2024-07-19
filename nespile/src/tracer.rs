use std::{collections::BTreeMap, io::{Cursor, Read}, rc::Rc};

use binrw::{BinRead, Endian, Error as BinError};
use range_set::RangeSet;
use thiserror::Error;

use crate::parser::opcodes::Opcode;



mod opcodes;
mod range_set;

const PRGROM_OFFSET: u16 = 0x8000;



#[derive(Clone)]
pub struct TraceState {
    a: RangeSet<u8>,
    x: RangeSet<u8>,
    y: RangeSet<u8>,
}
impl TraceState {
    pub fn new() -> Self {
        TraceState{
            a: RangeSet::new(),
            x: RangeSet::new(),
            y: RangeSet::new(),
        }
    }
}

pub struct TraceContext {
    read_range: RangeSet<u16>,
    write_range: RangeSet<u16>,
    state: TraceState,
    program: ProgramTree,
}
impl TraceContext {
    pub fn new() -> Self {
        TraceContext {
            read_range: RangeSet::new(),
            write_range: RangeSet::new(),
            state: TraceState::new(),
            program: ProgramTree::new()
        }
    }

    pub fn add_write_address(&mut self, addr: u16) -> bool {
        self.write_range.insert_unit(addr)   
    }
    pub fn add_read_address(&mut self, addr: u16) -> bool {
        if self.write_range.contains(&addr) {
            println!("WARN: read from ${:04x} without initializing it", addr);
        }
        self.read_range.insert_unit(addr)   
    }
}



#[derive(Debug, Error)]
pub enum TraceError {
    #[error(transparent)]
    Parse(#[from] BinError)
}

pub struct OperationNode {
    address: u16,
    operation: Opcode,
    next: Vec<u16>
}
pub type ProgramTree = BTreeMap<u16, OperationNode>;

pub fn trace_program(prgrom: Vec<u8>) -> Result<ProgramTree, TraceError> {
    let mut cursor = Cursor::new(prgrom);

    let mut ctx = TraceContext::new();

    let mut to_trace = vec![PRGROM_OFFSET];

    while let Some(addr) = to_trace.pop() {
        // Already parsed, skip
        if ctx.program.contains_key(&addr) { continue; }

        cursor.set_position((addr - PRGROM_OFFSET) as u64);
        let op = Opcode::read_options(&mut cursor, Endian::Little, ())?;
        let next_addrs = op.next_address(addr);

        for next_addr in next_addrs.iter() {
            if next_addr < &PRGROM_OFFSET {
                println!("WARN: cannot trace address {:04x}", next_addr);
                continue;
            }

            if !ctx.program.contains_key(next_addr) && !to_trace.contains(next_addr) {
                to_trace.push(*next_addr);
            }
        }

        let node = OperationNode {
            address: addr,
            operation: op,
            next: next_addrs
        };
        ctx.program.insert(addr, node);
    }
    
    Ok(ctx.program)
}



pub fn program_to_source_string(prgtree: ProgramTree) -> String {
    let mut next_addr = PRGROM_OFFSET;
    prgtree.iter()
        .map(|(addr, node)| {
            let expected_addr = next_addr;
            next_addr = addr + node.operation.size() as u16;
            if addr == &expected_addr {
                format!("${:04x}    {}", addr, node.operation.to_source_string())
            } else {
                format!("...\n${:04x}    {}", addr, node.operation.to_source_string())
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}