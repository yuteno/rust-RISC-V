use std::fmt;
use std::ops::{Bound, Range, RangeBounds};

pub type CsrAddress = u16;

pub const MXLEN: usize = 64;

pub const CSR_SIZE: usize = 4096;



pub const USTATUS: CsrAddress = 0x000;
pub const UTVEC: CsrAddress = 0x005;


pub const UEPC: CsrAddress = 0x041;
pub const UCAUSE: CsrAddress = 0x042;
pub const UTVAL: CsrAddress = 0x043;


pub const FFLAGS: CsrAddress = 0x001;
pub const FRB: CsrAddress = 0x002;
pub const FCSR: CsrAddress = 0x003;


pub const SSTATUS: CsrAddress = 0x100;
pub const SDELEG: CsrAddress = 0x102;
pub const SIDELEG: CsrAddress = 0x103;
pub const STVEC: CsrAddress = 0x105;



pub const SEPC: CsrAddress = 0x141;
pub const SCAUSE: CsrAddress = 0x142;
pub const STVAL: CsrAddress = 0x143;


pub const SATP: CsrAddress = 0x180;




pub const MVENDORID: CsrAddress = 0xf11;
pub const MARCHID: CsrAddress = 0xf12;
pub const MIMPID: CsrAddress = 0xf13;
pub const MHARTID: CsrAddress = 0xf14;


pub const MSTATUS: CsrAddress = 0x300;
pub const MISA: CsrAddress = 0x301;
pub const MEDELEG: CsrAddress = 0x302;
pub const MIDELEG: CsrAddress = 0x303;
pub const MIE: CsrAddress = 0x304;
pub const MTVEC: CsrAddress = 0x305;
pub const MCOUNTEREN: CsrAddress = 0x306;


pub const MSCRATCH: CsrAddress = 0x340;
pub const MEPC: CsrAddress = 0x341;
pub const MCAUSE: CsrAddress = 0x342;
pub const MTVAL: CsrAddress = 0x343;
pub const MIP: CsrAddress = 0x344;

pub const PMPCFG0: CsrAddress = 0x3a0;
pub const PMPADDR0: CsrAddress = 0x3b0;



pub struct State {
    csrs: [u64; CSR_SIZE],
}


impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{}\n{}\n{}",
                format!(
                    "mstatus={:>#18x} mtvec={:>#18x} mepc={:>#18x}\n mcause={:>#18x} mtval={:>#18x}",
                    self.read(MSTATUS),
                    self.read(MTVEC),
                    self.read(MEPC),
                    self.read(MCAUSE),
                    self.read(MTVAL),
                ),
                format!(
                    "sstatus={:>#18x} stvec={:>#18x} sepc={:>#18x}\n scause={:>#18x} stval={:>#18x}",
                    self.read(SSTATUS),
                    self.read(STVEC),
                    self.read(SEPC),
                    self.read(SCAUSE),
                    self.read(STVAL),
                ),
                format!(
                    "ustatus={:>#18x} utvec={:>#18x} uepc={:>#18x}\n ucause={:>#18x} utval={:>#18x}",
                    self.read(USTATUS),
                    self.read(UTVEC),
                    self.read(UEPC),
                    self.read(UCAUSE),
                    self.read(UTVAL),
                ),
            )
        )
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            csrs: [0; CSR_SIZE],
        }
    }

    pub fn read(&self, addr: CsrAddress) -> u64 {
        self.csrs[addr as usize]
    }

    pub fn write(&mut self, addr: CsrAddress, val: u64) {
        match addr {
            MVENDORID => {}
            MARCHID => {}
            MIMPID => {}
            MHARTID => {}
            _ => self.csrs[addr as usize] = val,

        }
    }



}