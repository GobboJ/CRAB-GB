mod registers;
mod memory;

use num_traits::FromPrimitive;
use registers::Registers;
use registers::Flag;
use memory::Memory;

pub struct CPU {

    registers: Registers,
    memory: Memory
   
}

impl CPU {

    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    pub fn step(&mut self) -> u8 {
        let pc = self.registers.read_pc();
        let byte = self.memory.read(pc);
        self.registers.increase_pc(1);

        // let first_operand = (byte >> 3) & 0b00000111;
        // let second_operand = byte & 0b00000111;
        // let double_operand = (byte >> 4) & 0b00000011;
        // let condition = (byte >> 3) & 0b00000011;

        match byte {
            /*
                GROUP 00
            */
            0x00 => {
                // NOP
                2
            },
            0x08 => {
                // LD (u16), SP
                let lsb = self.memory.read(pc);
                self.registers.increase_pc(1);
                let msb = self.memory.read(pc);
                self.registers.increase_pc(1);
                let address = (msb as u16) << 8 | (lsb as u16);
                let sp = self.registers.read_double_register(registers::DoubleRegister::SP);
                self.memory.write(address, (sp & 0x0F) as u8); // is it 0xFF instead ?
                self.memory.write(address + 1, (sp >> 8) as u8);
                5
            },
            0x10 => {
                // STOP
                0
            },
            0x18 => {
                // JR i8
                let offset = self.memory.read(pc);
                self.registers.increase_pc(1);

                self.registers.increase_pc(offset as u16);
                3
            },
            0x20 | 0x28 | 0x30 | 0x38 => {
                // JR cond, i8
                let offset = self.memory.read(pc);
                self.registers.increase_pc(1);

                let cond = FromPrimitive::from_u8((byte >> 3) & 0b00000011).unwrap();
                if self.registers.check_condition(cond) {
                    self.registers.increase_pc(offset as u16);
                    3
                } else {
                    2
                }
            },
            0x01 | 0x11 | 0x21 | 0x31 => {
                // LD r16, u16
                let register = FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();
                
                let lsb = self.memory.read(pc);
                self.registers.increase_pc(1);
                let msb = self.memory.read(pc);
                self.registers.increase_pc(1);

                self.registers.set_double_register(register, (msb as u16) << 8 | (lsb as u16));
                3
            },
            0x09 | 0x19 | 0x29 | 0x39 => {
                // ADD HL, r16
                let register = FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();
                let reg_content = self.registers.read_register(register) as u16;
                let hl_content = self.registers.read_double_register(registers::DoubleRegister::HL);
                let result = hl_content.wrapping_add(reg_content);
                self.registers.set_double_register(registers::DoubleRegister::HL, result);
                self.registers.unset_flag(Flag::N);
                if hl_content & 0xfff + reg_content & 0xfff > 0xfff {
                    self.registers.set_flag(Flag::H);
                }
                if hl_content > 0xffff - reg_content {
                    self.registers.set_flag(Flag::C);
                }
                
                2
            },
            0x02 | 0x12 | 0x22 | 0x32 => {
                // LD [r16mem], A
                let dest_register = FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();

                let data = self.registers.read_register(registers::Register::A);

                self.memory.write(self.registers.read_double_register_mem(dest_register), data);
                2
            },
            0x0A | 0x1A | 0x2A | 0x3A => {
                // LD A, [r16mem]
                let source_register = FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();

                let data = self.memory.read(self.registers.read_double_register_mem(source_register));
                self.registers.set_register(registers::Register::A, data);
                2
            },
            0x03 | 0x13 | 0x23 | 0x33 => {
                // INC r16
                let register = FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();
                self.registers.set_double_register(register, self.registers.read_double_register(register).wrapping_add(1));
                2
            },
            0x0B | 0x1B | 0x2B | 0x3B => {
                // DEC r16
                let register = FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();
                self.registers.set_double_register(register, self.registers.read_double_register(register).wrapping_sub(1));
                2
            },
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
                // INC r8
                let register = FromPrimitive::from_u8((byte >> 3) & 0b00000111).unwrap();
                let mut result = 0;
                let mut cycles = 1;
                if let registers::Register::HL_MEM = register {
                    result = self.memory.read(self.registers.read_double_register(registers::DoubleRegister::HL)).wrapping_add(1);
                    self.memory.write(self.registers.read_double_register(registers::DoubleRegister::HL), result);
                    cycles = 3
                } else {
                    result = self.registers.read_register(register).wrapping_add(1);
                    self.registers.set_register(register, result);
                }

                self.registers.unset_flag(Flag::N);
                if result == 0 {
                    self.registers.set_flag(Flag::Z);
                }
                if result & 0xf == 0xf {
                    self.registers.set_flag(Flag::H);
                }
                cycles
            },
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => {
                // DEC r8
                let register = FromPrimitive::from_u8((byte >> 3) & 0b00000111).unwrap();
                let mut result = 0;
                let mut cycles = 1;
                if let registers::Register::HL_MEM = register {
                    result = self.memory.read(self.registers.read_double_register(registers::DoubleRegister::HL)).wrapping_sub(1);
                    self.memory.write(self.registers.read_double_register(registers::DoubleRegister::HL), result);
                    cycles = 3
                } else {
                    result = self.registers.read_register(register).wrapping_sub(1);
                    self.registers.set_register(register, result);
                }

                self.registers.set_flag(Flag::N);
                if result == 0 {
                    self.registers.set_flag(Flag::Z);
                }
                if result & 0xf == 0 {
                    self.registers.set_flag(Flag::H);
                }
                cycles
            },
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                // LD r8, u8
                let r = FromPrimitive::from_u8((byte >> 3) & 0b00000111).unwrap();
                let n = self.memory.read(pc);
                self.registers.increase_pc(1);
                if let registers::Register::HL_MEM = r {
                    self.memory.write(self.registers.read_double_register(registers::DoubleRegister::HL), n);
                    3
                } else { 
                    self.registers.set_register(r, n);
                    2 
                }
            },
            0x07 => {
                // RLCA
                self.registers.set_register(registers::Register::A, self.registers.read_register(registers::Register::A).rotate_left(1));
                if self.registers.read_register(registers::Register::A) & 1 == 1{
                    self.registers.set_flag(Flag::C)
                } else {
                    self.registers.unset_flag(Flag::C)
                }
                self.registers.unset_flag(Flag::Z);
                self.registers.unset_flag(Flag::N);
                self.registers.unset_flag(Flag::H);
                1
            },
            0x0F => {
                // RRCA
                if self.registers.read_register(registers::Register::A) & 1 == 1{
                    self.registers.set_flag(Flag::C)
                } else {
                    self.registers.unset_flag(Flag::C)
                }
                self.registers.set_register(registers::Register::A, self.registers.read_register(registers::Register::A).rotate_right(1));
                self.registers.unset_flag(Flag::Z);
                self.registers.unset_flag(Flag::N);
                self.registers.unset_flag(Flag::H);
                1
            },
            0x17 => {
                // RLA
                let mut a_value = self.registers.read_register(registers::Register::A);
                let flag = self.registers.read_flag(Flag::C);
                if (a_value & 0b10000000) >> 7 == 1 {
                    self.registers.set_flag(Flag::C);
                } else {
                    self.registers.unset_flag(Flag::C);
                }
                a_value <<= 1;
                a_value |= flag as u8;
                self.registers.unset_flag(Flag::Z);
                self.registers.unset_flag(Flag::N);
                self.registers.unset_flag(Flag::H);
                1
            },
            0x1F => {
                // RRA
                let mut a_value = self.registers.read_register(registers::Register::A);
                let flag = self.registers.read_flag(Flag::C);
                if a_value & 1 == 1 {
                    self.registers.set_flag(Flag::C);
                } else {
                    self.registers.unset_flag(Flag::C);
                }
                a_value >>= 1;
                a_value |= (flag as u8) << 0b10000000;
                self.registers.unset_flag(Flag::Z);
                self.registers.unset_flag(Flag::N);
                self.registers.unset_flag(Flag::H);
                1
            },
            0x27 => {
                // DAA
                todo!()
            },
            0x2F => {
                // CPL
                self.registers.set_register(registers::Register::A, !self.registers.read_register(registers::Register::A));
                self.registers.set_flag(Flag::N);
                self.registers.set_flag(Flag::H);
                1
            },
            0x37 => {
                // SCF
                self.registers.set_flag(Flag::C);
                1
            },
            0x3F => {
                // CCF
                self.registers.toggle_flag(Flag::C);
                1
            },

            /*
                GROUP 01
            */
            0x76 => {
                // HALT
                todo!();
            },
            0x40..=0x7F => {
                // LD r8, r8
                let dest = FromPrimitive::from_u8((byte >> 3) & 0b00000111).unwrap();
                let source = FromPrimitive::from_u8(byte & 0b00000111).unwrap();

                self.registers.set_register(dest, self.registers.read_register(source));

                if let registers::Register::HL_MEM = dest { 2 }
                else if let registers::Register::HL_MEM = source { 2 }
                else { 1 }
            },
            
            /*
                GROUP 10
            */
            0x80..=0x87 => {
                // ADD A, r8
            },
            0x88..=0x8F => {
                // ADC A, r8
            },
            0x90..=0x97 => {
                // SUB A, r8
            },
            0x98..=0x9F => {
                // SBC A, r8
            },
            0xA0..=0xA7 => {
                // AND A, r8
            },
            0xA8..=0xAF => {
                // XOR A, r8
                let register = FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                let mut data = 0;
                let mut cycles = 1;
                if let registers::Register::HL_MEM = register {
                    data = self.memory.read(self.registers.read_double_register(registers::DoubleRegister::HL));
                    cycles = 2;
                } else {
                    data = self.registers.read_register(register);
                }
                let res = self.registers.read_register(registers::Register::A) ^ data;
                self.registers.set_register(registers::Register::A, res);

                self.registers.unset_flag(Flag::Z);
                self.registers.unset_flag(Flag::N);
                self.registers.unset_flag(Flag::H);
                self.registers.unset_flag(Flag::C);
                if res == 0 {
                    self.registers.set_flag(Flag::Z);
                }

                cycles
            },
            0xB0..=0xB7 => {
                // OR A, r8
            },
            0xB8..=0xBF => {
                // CP A, r8
            },

            /*
                GROUP 11
            */
            0xC0 | 0xC8 | 0xD0 | 0xD8 => {
                // RET cond
            },
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {
                // POP r16stk
            },
            0xC9 => {
                // RET
            },
            0xD9 => {
                // RETI
            },
            0xC2 | 0xCA | 0xD2 | 0xDA => {
                // JP cond, u16
            },
            0xC3 => {
                // JP u16
            },
            0xC4 | 0xCC | 0xD4 | 0xDC => {
                // CALL cond, u16
            },
            0xC5 | 0xD5 | 0xE5 | 0xF5 => {
                // PUSH r16stk
            },
            0xCD => {
                // CALL u16
            },
            0xC6 => {
                // ADD A, u8
            },
            0xCE => {
                // ADC A, u8
            },
            0xD6 => {
                // SUB A, u8
            },
            0xDE => {
                // SBC A, u8
            },
            0xE6 => {
                // AND A, u8
            },
            0xEE => {
                // XOR A, u8
            },
            0xF6 => {
                // OR A, u8
            },
            0xFE => {
                // CP A, u8
            },
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                // RST tgt3
            },
            0xE0 => {
                // LD (FF00+u8), A
            },
            0xF0 => {
                // LD A, (FF00+u8)
            },
            0xE8 => {
                // ADD SP, i8
            },
            0xF8 => {
                // LD, SP+i8
            },
            0xE9 => {
                // JP HL
            },
            0xF9 => {
                // LD SP, HL
                self.registers.set_double_register(registers::DoubleRegister::SP, self.registers.read_double_register(registers::DoubleRegister::HL));
                2
            },
            0xE2 => {
                // LD (FF00+C), A
            },
            0xEA => {
                // LD (u16), A
            },
            0xF2 => {
                // LD A, (FF00+C)
            },
            0xFA => {
                // LD A, (u16)
            },
            0xF3 => {
                // DI
            },
            0xFB => {
                // EI
            },
            
            /*
                Group CB
            */
            0xCB => {
                let cb_instruction: u8 = self.memory.read(pc + 1);
                match cb_instruction {
                    0x00..=0x07 => {
                        // RLC r8
                    },
                    0x08..=0x0F => {
                        // RRC r8
                    },
                    0x10..=0x17 => {
                        // RL r8
                    },
                    0x18..=0x1F => {
                        // RR r8
                    },
                    0x20..=0x27 => {
                        // SLA r8
                    },
                    0x28..=0x2F => {
                        // SRA r8
                    },
                    0x30..=0x37 => {
                        // SWAP r8
                    },
                    0x38..=0x3F => {
                        // SRL r8
                    },
                    0x40..=0x7F => {
                        // BIT b3, r8
                        let register = FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                        let bit = (byte >> 3) & 0b00000111;

                        self.registers.unset_flag(Flag::N);
                        self.registers.set_flag(Flag::C);

                        if let registers::Register::HL_MEM = register {
                            if self.memory.read(self.registers.read_double_register(registers::DoubleRegister::HL)) >> bit & 1 == 0 {
                                self.registers.set_flag(Flag::Z);
                            }
                            3
                        } else {
                            if self.registers.read_register(register) >> bit & 1 == 0 {
                                self.registers.set_flag(Flag::Z);
                            }
                            2
                        }
                    },
                    0x80..=0xBF => {
                        // RES b3, r8
                        let register = FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                        let bit = (byte >> 3) & 0b00000111;

                        if let registers::Register::HL_MEM = register {
                            let value = self.memory.read(self.registers.read_double_register(registers::DoubleRegister::HL));
                            self.memory.write(self.registers.read_double_register(registers::DoubleRegister::HL), value & !(1 << bit));
                            4
                        } else {
                            self.registers.set_register(register, self.registers.read_register(register) & !(1 << bit));
                            2
                        }
                    },
                    0xC0..=0xFF => {
                        // SET b3, r8
                        let register = FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                        let bit = (byte >> 3) & 0b00000111;

                        if let registers::Register::HL_MEM = register {
                            let value = self.memory.read(self.registers.read_double_register(registers::DoubleRegister::HL));
                            self.memory.write(self.registers.read_double_register(registers::DoubleRegister::HL), value | (1 << bit));
                            4
                        } else {
                            self.registers.set_register(register, self.registers.read_register(register) | (1 << bit));
                            2
                        }

                    }
                    _ => panic!("Unrecognized CB instruction: {}", cb_instruction)
                }
            }           
            _ => panic!("Unrecognized instruction: {}", byte)
        }
    }
}
