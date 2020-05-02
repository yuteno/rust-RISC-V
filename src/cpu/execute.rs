use super::Cpu;

impl Cpu {

    pub fn execute(&mut self, inst: u32) {
        let opcode = inst & 0x0000007f;
        let rd = ((inst & 0x00000f80) >> 7) as usize;
        let rs1 = ((inst & 0x000f8000) >> 15) as usize;
        let rs2 = ((inst & 0x01f00000) >> 20) as usize;
        let funct3 = (inst & 0x00007000) >> 12;
        let funct7 = (inst & 0xfe000000) >> 25;
        self.regs[0] = 0;

        match opcode {
            0x03 => {

            }
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
    }
}