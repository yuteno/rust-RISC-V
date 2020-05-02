use super::Cpu;


impl Cpu {
    pub fn new(binary: Vec<u8>) -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
            memory: binary,
        }
    }
    pub fn increment_pc(&mut self, val: u64) {
        self.pc += val;
    }

    pub fn get_pc(&self) -> u64 {
        self.pc
    }

    pub fn get_memlen(&self) -> u64 {
        self.memory.len() as u64
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
                    self.regs[i+1],
                    i + 2,
                    self.regs[i+2],
                    i + 3,
                    self.regs[i+3],
                )
            );
        }
        println!("{}", output);
    }
    pub fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        return (self.memory[index] as u32)
            | ((self.memory[index + 1] as u32) << 8)
            | ((self.memory[index + 2] as u32) << 16)
            | ((self.memory[index + 3] as u32) << 24);
    }
    /*
    pub fn execute(&mut self, inst: u32) {
        let opcode = inst & 0x0000007f;
        let rd = ((inst & 0x00000f80) >> 7) as usize;
        let rs1 = ((inst & 0x000f8000) >> 15) as usize;
        let rs2 = ((inst & 0x01f00000) >> 20) as usize;

        match opcode {
            0x13 => {
                //addi
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1] + imm;
            }
            0x33 => {
                //add
                self.regs[rd] = self.regs[rs1] + self.regs[rs2];
            }
            _ => {
                dbg!("not implemented yet");
            }
        }
    }*/


}