#![feature(lazy_cell)]

use opcodes::{OpCode, OPCODES_MAP};


mod cpu;
mod opcodes;
fn main() {
    let code = 0x00;
    let opcode = OPCODES_MAP.get(&code).unwrap();

    let a = 1;

}
