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

            0x23 => {
                let imm = (((inst & 0xfe00000) as i32 as i64 >> 20) as u64) | (((inst >> 7) & 0x1f)as u64);
                let addr = self.regs[rs1].wrapping_add(imm);

                match funct3 {
                    0x0 => self.write8(addr, self.regs[rs2]), // sb
                    0x1 => self.write16(addr, self.regs[rs2]), // sh
                    0x2 => self.write32(addr, self.regs[rs2]), // sw
                    0x3 => self.write64(addr, self.regs[rs2]), // sd
                    _ => {}
                }
            }
            0x33 => {
                let shamt = ((self.regs[rs2] & 0x3f) as u64) as u32;
                match (funct3, funct7) {
                    (0x0, 0x00) => {
                        //add
                        self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
                    }

                    (0x0, 0x20) => {
                        //sub
                        self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs2]);
                    }

                    (0x1, 0x00) => {
                        //sll
                        self.regs[rd] = self.regs[rs1].wrapping_shl(shamt);
                    }

                    (0x2, 0x00) => {
                        //slt
                        self.regs[rd] = if (self.regs[rs1] as i64) < (self.regs[rs2] as i64) {
                            1
                        } else {
                            0
                        };
                    }

                    (0x3, 0x00) => {
                        //sltu
                        self.regs[rd] = if self.regs[rs1] < self.regs[rs2]{
                            1
                        } else {
                            0
                        };
                    }

                    (0x4, 0x00) => {
                        //xor
                        self.regs[rd] = self.regs[rs1] ^ self.regs[rs2];
                    }

                    (0x5, 0x00) => {
                        //srl
                        self.regs[rd] = self.regs[rs1].wrapping_shr(shamt);
                    }

                    (0x5, 0x20) => {
                        //sra
                        self.regs[rd] = (self.regs[rs1] as i64).wrapping_shr(shamt) as u64;
                    }

                    (0x6, 0x00) => {
                        //or
                        self.regs[rd] = self.regs[rs1] | self.regs[rs2];
                    }

                    (0x7, 0x00) => {
                        //and
                        self.regs[rd] = self.regs[rs1] & self.regs[rs2];
                    }

                    _ => {}
                }
            }
            0x37 => {
                //lui
                self.regs[rd] = (inst & 0xfffff000) as i32 as i64 as u64;
            }

            0x3b => {
                let shamt = (self.regs[rs2] & 0x1f) as u32;
                match (funct3, funct7) {
                    (0x0, 0x00) => {
                        //addw
                        self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]) as i32 as i64 as u64;
                    }

                    (0x0, 0x20) => {
                        //subw
                        self.regs[rd] = ((self.regs[rs1].wrapping_sub(self.regs[rs2])) as i32) as u64;
                    }

                    (0x1, 0x00) => {
                        //sllw
                        self.regs[rd] = (self.regs[rs1] as u32).wrapping_shl(shamt) as i32 as u64;
                    }

                    (0x5, 0x00) => {
                        //srlw
                        self.regs[rd] = (self.regs[rs1] as u32).wrapping_shr(shamt) as i32 as u64;
                    }

                    (0x5, 0x20) => {
                        //sraw
                        self.regs[rd] = ((self.regs[rs1] as i32) >> (shamt as i32)) as u64;
                    }

                    _ => {}
                }
            }

            0x63 => {
                let imm = (((inst & 0x80000000) as i32 as i64 >> 19) as u64)
                    | (((inst & 0x80) << 4) as u64)
                    | (((inst >> 20) & 0x7e0) as u64)
                    | (((inst >> 7) & 0x1e) as u64) ;

                match funct3 {
                    0x0 => {
                        //beq
                        if self.regs[rs1] == self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }

                    0x1 => {
                        //bne
                        if self.regs[rs1] != self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }

                    0x4 => {
                        //blt
                        if (self.regs[rs1] as i64) < (self.regs[rs2] as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }

                    0x5 => {
                        //bge
                        if (self.regs[rs1] as i64) >= (self.regs[rs2] as i64) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }

                    0x6 => {
                        //bltu
                        if self.regs[rs1] < self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }

                    0x7 => {
                        //bgeu
                        if self.regs[rs1] >= self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                        }
                    }

                    _ => {}
                }
            }

            0x67 => {
                //jalr
                let t = self.pc;

                let imm = ((((inst & 0xfff00000) as i32) as i64) >> 20) as u64;
                self.pc = (self.regs[rs1].wrapping_add(imm)) & !1;

                self.regs[rd] = t;
            }

            0x6f => {
                //jal
                self.regs[rd] = self.pc;

                let imm = (((inst & 0x80000000) as i32 as i64 >> 11) as u64)
                    | ((inst & 0xff000) as u64)
                    | (((inst >> 9) & 0x800) as u64)
                    | (((inst >> 20) & 0x7fe) as u64);

                self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }

            _ => {
                dbg!(format!("not implemented yet: opcode {:#x}", opcode));
            }
        }
    }
}