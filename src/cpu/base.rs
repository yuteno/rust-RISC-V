use super::Cpu;
use crate::memory::MEMORY_SIZE;
use crate::memory::Memory;
//memory size = 128MiB

impl Cpu {
    pub fn new(binary: Vec<u8>) -> Self {
        //let mut memory = vec![0; MEMORY_SIZE as usize];
        //memory.splice(..binary.len(), binary.iter().cloned());

        let mut regs = [0; 32];
        let mut memory = Memory::new();
        memory.set_dram(binary);
        regs[2] = MEMORY_SIZE;
        Self {
            regs,
            pc: 0,
            memory,
        }
    }
    pub fn increment_pc(&mut self, val: u64) {
        self.pc += val;
    }

    pub fn get_pc(&self) -> u64 {
        self.pc
    }

    pub fn get_memlen(&self) -> u64 {
        self.memory.size() as u64
    }


    pub fn dump_registers(&self) {
        let mut output = String::from("");
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}={:>#18x} x{:02}={:>#18x} x{:02}={:>#18x} x{:02}={:>#18x}",
                    i,
                    self.regs[i],
                    i + 1,
                    self.regs[i + 1],
                    i + 2,
                    self.regs[i + 2],
                    i + 3,
                    self.regs[i + 3],
                )
            );
        }
        println!("{}", output);
    }

    pub fn fetch(&self) -> u32 {
        return self.memory.read32(self.pc) as u32;
    }
    /*
    pub(crate) fn read8(&self, addr: u64) -> u64 {
        let index = addr as usize;
        self.memory[index] as u64
    }

    pub(crate) fn read16(&self, addr: u64) -> u64 {
        let index = addr as usize;
        return (self.memory[index] as u64) | ((self.memory[index + 1] as u64) << 8);
    }

    pub(crate) fn read32(&self, addr: u64) -> u64 {
        let index = addr as usize;
        return (self.memory[index] as u64)
            | ((self.memory[index + 1] as u64) << 8)
            | ((self.memory[index + 2] as u64) << 16)
            | ((self.memory[index + 3] as u64) << 24);
    }

    pub(crate) fn read64(&self, addr: u64) -> u64 {
        let index = addr as usize;
        return (self.memory[index] as u64)
            | ((self.memory[index + 1] as u64) << 8)
            | ((self.memory[index + 2] as u64) << 16)
            | ((self.memory[index + 3] as u64) << 24)
            | ((self.memory[index + 4] as u64) << 32)
            | ((self.memory[index + 5] as u64) << 40)
            | ((self.memory[index + 6] as u64) << 48)
            | ((self.memory[index + 7] as u64) << 56);
    }

    pub(crate) fn write8(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.memory[index] = val as u8;
    }

    pub(crate) fn write16(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.memory[index] = (val & 0xff) as u8;
        self.memory[index + 1] = ((val >> 8) & 0xff) as u8;
    }

    pub(crate) fn write32(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.memory[index] = (val & 0xff) as u8;
        self.memory[index + 1] = ((val >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((val >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((val >> 24) & 0xff) as u8;
    }

    pub(crate) fn write64(&mut self, addr: u64, val: u64) {
        let index = addr as usize;
        self.memory[index] = (val & 0xff) as u8;
        self.memory[index + 1] = ((val >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((val >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((val >> 24) & 0xff) as u8;
        self.memory[index + 4] = ((val >> 32) & 0xff) as u8;
        self.memory[index + 5] = ((val >> 40) & 0xff) as u8;
        self.memory[index + 6] = ((val >> 48) & 0xff) as u8;
        self.memory[index + 7] = ((val >> 56) & 0xff) as u8;
    }
    */
}
