pub const REGISTERS_COUNT: usize = 32;
pub mod cpu_main;
pub mod execute;
pub mod exception;
pub mod mode;
use crate::csr::*;
use crate::memory::Memory;

pub struct Cpu {
    regs: [u64; REGISTERS_COUNT],
    pc: u64,
    mode: mode::Mode,
    memory: Memory,
    state: State,
}