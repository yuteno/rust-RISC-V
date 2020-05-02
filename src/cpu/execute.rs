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
                let imm = ((inst as i32 as i64) >> 20) as u64;
                let addr = self.regs[rs1].wrapping_add(imm);
                match funct3 {
                    0x0 => {
                        //lb
                        let val = self.read8(addr);
                        self.regs[rd] = val as i8 as i64 as u64;
                    }

                    0x1 => {
                        //lh
                        let val = self.read16(addr);
                        self.regs[rd] = val as i16 as i64 as u64;
                    }

                    0x2 => {
                        //lw
                        let val = self.read32(addr);
                        self.regs[rd] = val as i32 as i64 as u64;
                    }

                    0x3 => {
                        //ld
                        let val = self.read64(addr);
                        self.regs[rd] = val;
                    }

                    0x4 => {
                        //lbu
                        let val = self.read8(addr);
                        self.regs[rd] = val;
                    }

                    0x5 => {
                        //lhu
                        let val = self.read16(addr);
                        self.regs[rd] = val;
                    }

                    0x6 => {
                        //lwu
                        let val = self.read32(addr);
                        self.regs[rd] = val;
                    }
                    _ => {}

                }
            }

            0x13 => {
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                let shamt = (imm & 0x3f) as u32;
                match funct3 {
                    0x0 => {
                        //addi
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm);
                    }

                    0x0 => {
                        //addi
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm);
                    }

                    0x1 => {
                        //slli
                        self.regs[rd] = self.regs[rs1] << shamt;
                    }

                    0x2 => {
                        //slti
                        self.regs[rd] = if (self.regs[rs1] as i64) < (imm as i64) {
                            1
                        } else {
                            0
                        };
                    }

                    0x3 => {
                        //sltiu
                        self.regs[rd] = if self.regs[rs1] < imm {
                            1
                        } else {
                            0
                        };
                    }

                    0x4 => {
                        //xori
                        self.regs[rd] = self.regs[rs1] ^ imm;
                    }

                    0x5 => {
                        match funct7 >> 1 {
                            0x00 => {
                                //srli
                                self.regs[rd] = self.regs[rs1].wrapping_shr(shamt);
                            }

                            0x10 => {
                                //srai
                                self.regs[rd] = (self.regs[rs1] as i64).wrapping_shr(shamt) as u64;
                            }

                            _ => {}
                        }
                    }

                    0x6 => {
                        //ori
                        self.regs[rd] = self.regs[rs1] | imm;
                    }

                    0x7 => {
                        //andi
                        self.regs[rd] = self.regs[rs1] & imm;
                    }

                    _ => {}
                }
            }

            0x17 => {
                //auipc
                let imm = (inst & 0xfffff000) as i32 as i64 as u64;
                self.regs[rd] = self.pc.wrapping_add(imm).wrapping_sub(4);
            }

            0x1b => {
                let imm = ((inst as i32 as i64) >> 20) as u64;

                let shamt = (imm & 0x1f) as u32;
                match funct3 {
                    0x0 => {
                        //addiw
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm) as i32 as i64 as u64;
                    }

                    0x1 => {
                        //slliw
                        self.regs[rd] = self.regs[rs1].wrapping_shl(shamt) as i32 as i64 as u64;
                    }

                    0x5 => {
                        match funct7 {
                            0x00 => {
                                //srliw
                                self.regs[rd] = (self.regs[rs1] as u32).wrapping_shr(shamt) as i32 as i64 as u64;
                            }

                            0x20 => {
                                //sraiw
                                self.regs[rd] = (self.regs[rs1] as i32).wrapping_shr(shamt) as i64 as u64;
                            }

                            _ => {}
                        }
                    }

                    _ => {}
                }
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