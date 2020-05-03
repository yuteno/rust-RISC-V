pub const REGISTERS_COUNT: usize = 32;
pub mod base;
pub mod execute;
//pub mod mode;
use crate::memory::Memory;

pub struct Cpu {
    regs: [u64; REGISTERS_COUNT],
    pc: u64,
    memory: Memory,
}