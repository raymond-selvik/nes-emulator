use crate::cpu::AddressingMode;
use std::{collections::HashMap, sync::LazyLock};

pub struct OpCode {
    pub operation: &'static str,
    pub code: u8,
    pub addressing_mode: AddressingMode,
    pub len: u8,
    pub cycles: u8,
}

impl OpCode {
    fn new(operation: &'static str, code: u8, addressing_mode: AddressingMode, len: u8, cycles: u8) -> Self {
        OpCode {
            operation: operation,
            code: code,
            addressing_mode: addressing_mode,
            len: len,
            cycles: cycles
        }
    }
}

pub static OPCODES_MAP: LazyLock<HashMap<u8, OpCode>> = LazyLock::new(|| {
    let mut map: HashMap<u8, OpCode> = HashMap::new();
    
    map.insert(0x00, OpCode::new("BRK", 0x00, AddressingMode::NoneAddressing, 1, 7));
    map.insert(0xaa, OpCode::new("TAX", 0xaa, AddressingMode::NoneAddressing, 1, 2));
    map.insert(0xe8, OpCode::new("INX", 0xe8, AddressingMode::NoneAddressing, 1, 2));
    
    map.insert(0xa9, OpCode::new("LDA", 0xa9, AddressingMode::Immediate, 2, 2));
    map.insert(0xa5, OpCode::new("LDA", 0xa5, AddressingMode::ZeroPage, 2, 3));
    map.insert(0xb5, OpCode::new("LDA", 0xb5, AddressingMode::ZeroPage_X, 2, 4));
    map.insert(0xad, OpCode::new("LDA", 0xad, AddressingMode::Absolute, 3, 2));
    map.insert(0xbd, OpCode::new("LDA", 0xbd, AddressingMode::Absolute_X, 3, 4));
    map.insert(0xb9, OpCode::new("LDA", 0xb9, AddressingMode::Absolute_Y, 3, 4));
    map.insert(0xa1, OpCode::new("LDA", 0xa1, AddressingMode::Indirect_X, 2, 6));
    map.insert(0xb1, OpCode::new("LDA", 0xb1, AddressingMode::Indirect_Y, 2, 6));
    
    map
});