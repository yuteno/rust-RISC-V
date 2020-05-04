pub const REGISTERS_COUNT: usize = 32;
pub mod cpu_main;
pub mod execute;
//pub mod exception;
pub mod mode;

use crate::csr::*;
use crate::memory::Memory;
use crate::exception::Exception;


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
    Debug,
}
pub struct Cpu {
    regs: [u64; REGISTERS_COUNT],
    pc: u64,
    mode: Mode,
    memory: Memory,
    state: State,
}
