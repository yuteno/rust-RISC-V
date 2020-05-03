pub mod memory;

pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

#[derive(Debug)]
pub struct Memory {
    pub dram: Vec<u8>,
    code_size: u64,
}