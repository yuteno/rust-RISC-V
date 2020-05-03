use super::Memory;
use super::MEMORY_SIZE;

impl Memory {
    pub fn new() -> Memory {
        Self {
            dram: vec![0; MEMORY_SIZE as usize],
            code_size: 0,
        }
    }


    pub fn size(&self) -> u64 {
        self.code_size
    }

    pub fn set_dram(&mut self, binary: Vec<u8>) {
        self.code_size = binary.len() as u64;
        self.dram.splice(..binary.len(), binary.iter().cloned());
    }


    pub(crate) fn read8(&self, addr: u64) -> u64 {
        let index = addr as usize;
        self.dram[index] as u64
    }

    pub(crate) fn read16(&self, addr: u64) -> u64 {
        let index = addr as usize;
        return (self.dram[index] as u64) | ((self.dram[index + 1] as u64) << 8);
    }

    pub(crate) fn read32(&self, addr: u64) -> u64 {
        let index = addr as usize;
        return (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
            | ((self.dram[index + 2] as u64) << 16)
            | ((self.dram[index + 3] as u64) << 24);
    }

    pub(crate) fn read64(&self, addr: u64) -> u64 {
        let index = addr as usize;
        return (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
            | ((self.dram[index + 2] as u64) << 16)
            | ((self.dram[index + 3] as u64) << 24)
            | ((self.dram[index + 4] as u64) << 32)
            | ((self.dram[index + 5] as u64) << 40)
            | ((self.dram[index + 6] as u64) << 48)
            | ((self.dram[index + 7] as u64) << 56);
    }

    pub(crate) fn write8(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.dram[index] = val as u8;
    }

    pub(crate) fn write16(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.dram[index] = (val & 0xff) as u8;
        self.dram[index + 1] = ((val >> 8) & 0xff) as u8;
    }

    pub(crate) fn write32(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.dram[index] = (val & 0xff) as u8;
        self.dram[index + 1] = ((val >> 8) & 0xff) as u8;
        self.dram[index + 2] = ((val >> 16) & 0xff) as u8;
        self.dram[index + 3] = ((val >> 24) & 0xff) as u8;
    }

    pub(crate) fn write64(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.dram[index] = (val & 0xff) as u8;
        self.dram[index + 1] = ((val >> 8) & 0xff) as u8;
        self.dram[index + 2] = ((val >> 16) & 0xff) as u8;
        self.dram[index + 3] = ((val >> 24) & 0xff) as u8;
        self.dram[index + 4] = ((val >> 32) & 0xff) as u8;
        self.dram[index + 5] = ((val >> 40) & 0xff) as u8;
        self.dram[index + 6] = ((val >> 48) & 0xff) as u8;
        self.dram[index + 7] = ((val >> 56) & 0xff) as u8;
    }
}