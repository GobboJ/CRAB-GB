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

    pub fn ciao(&mut self) {
        let a = FromPrimitive::from_u8(0b010).unwrap();
        let res = self.registers.read_register(a);
        // self.registers.set_flag(Flag::Zero);
    }

    pub fn step(&mut self) {
        let pc = self.registers.read_pc();
        let byte = self.memory.read(pc);

        // let group_0 = byte >> 6;
        // let group_1 = (byte >> 3) & 0b00000111;
        // let group_2 = byte & 0b00000111;

        // match group_0 {
        //     0b00 => {
        //         match gr
        //     },
        //     0b01 => {},
        //     0b10 => {},
        //     0b11 => {},
        //     _ => panic!("Unrecognized instruction: {}", byte)
        // }

        match byte {
            /*
                GROUP 00
            */
            0x00 => {
                // NOP
            },
            0x08 => {
                // LD (u16), SP
            },
            0x10 => {
                // STOP
            },
            0x18 => {
                // JR i8
            },
            0x20 | 0x28 | 0x30 | 0x38 => {
                // JR cond, i8
            },
            0x01 | 0x11 | 0x21 | 0x31 => {
                // LD r16, u16
            },
            0x09 | 0x19 | 0x29 | 0x39 => {
                // ADD HL, r16
            },
            0x02 | 0x12 | 0x22 | 0x32 => {
                // LD [r16mem], A
            },
            0x0A | 0x1A | 0x2A | 0x3A => {
                // LD A, [r16mem]
            },
            0x03 | 0x13 | 0x23 | 0x33 => {
                // INC r16
            },
            0x0B | 0x1B | 0x2B | 0x3B => {
                // DEC r16
            },
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
                // INC r8
            },
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => {
                // DEC r8
            },
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                // LD r8, u8
            },
            0x07 => {
                // RLCA
            },
            0x0F => {
                // RRCA
            },
            0x17 => {
                // RLA
            },
            0x1F => {
                // RRA
            },
            0x27 => {
                // DAA
            },
            0x2F => {
                // CPL
            },
            0x37 => {
                // SCF
            },
            0x3F => {
                // CCF
            },

            /*
                GROUP 01
            */
            0x76 => {
                // HALT
            },
            0x40..=0x7F => {
                // LD r8, r8
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
            0xCB => {
                // CB PREFIX
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
                    },
                    0x80..=0xBF => {
                        // RES b3, r8
                    },
                    0xC0..=0xFF => {
                        // SET b3, r8
                    }
                    _ => panic!("Unrecognized CB instruction: {}", cb_instruction)
                }
            }           
            _ => panic!("Unrecognized instruction: {}", byte)
        }
    }
}
