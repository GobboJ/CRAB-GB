mod registers;
mod memory;
mod timer;
mod interrupt;
mod gpu;
mod register;

pub(crate) mod joypad;

use num_traits::FromPrimitive;
use registers::Registers;
use registers::Flag;
use memory::Memory;

use crate::cpu::registers::DoubleRegister;
use crate::cpu::registers::Register;

use self::joypad::Button;
use self::joypad::Joypad;

pub struct CPU {

    registers: Registers,
    memory: Memory,
    enable_interrupts: bool,
    ime: bool,
    halted: bool
}

impl CPU {

    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            memory: Memory::new(),
            enable_interrupts: false,
            ime: false,
            halted: false
        }
    }

    pub fn update(&mut self) {
        let mut totalCycles = 0;
        while totalCycles < 69905 {
            let mut cycles = 0;

            if self.halted && self.memory.get_interrupts().is_pending() {
                self.halted = false;
            }

            if !self.halted {
                if self.ime {
                    let interrupt = self.handle_interrupts();
                    if interrupt {
                        cycles += 4;
                    }
                }
            
                let byte = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                cycles += self.decode(byte);
                
            } else {
                cycles = 1;
            }

            self.memory.update_timer(cycles);
            self.memory.update_gpu(cycles);

            totalCycles += cycles as usize;
        }
    }

    pub fn set_button(&mut self, button: Button) {
        self.memory.set_button(button);
    }

    pub fn unset_button(&mut self, button: Button) {
        self.memory.unset_button(button);
    }

    pub fn get_framebuffer(&self) -> [u8; 160*144*4] {
        self.memory.get_gpu().framebuffer
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        self.memory.load_rom(data);
    }

    pub fn decode(&mut self, byte: u8) -> u8 {

        // println!("[{:#06x}] {:#04x}", self.registers.read_pc() - 1, byte);
        if self.enable_interrupts {
            self.ime = true;
            self.enable_interrupts = false;
        }

        


        match byte {
            /*
                GROUP 00
            */
            0x00 => {
                // NOP
                1
            },
            0x08 => {
                // LD (u16), SP
                let lsb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let msb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let address = (msb as u16) << 8 | (lsb as u16);
                let sp = self.registers.read_double_register(&DoubleRegister::SP);
                self.memory.write(address, sp as u8);
                self.memory.write(address + 1, (sp >> 8) as u8);
                5
            },
            0x10 => {
                // STOP
                // self.registers.increase_pc();
                // 1
                todo!()
            },
            0x18 => {
                // JR i8
                let offset = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();

                self.registers.offset_pc(offset as i8);
                3
            },
            0x20 | 0x28 | 0x30 | 0x38 => {
                // JR cond, i8
                let offset = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();

                let cond = &FromPrimitive::from_u8((byte >> 3) & 0b00000011).unwrap();
                if self.registers.check_condition(cond) {
                    self.registers.offset_pc(offset as i8);
                    3
                } else {
                    2
                }
            },
            0x01 | 0x11 | 0x21 | 0x31 => {
                // LD r16, u16
                let register = &FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();
                
                let lsb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let msb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();

                self.registers.write_double_register(register, (msb as u16) << 8 | (lsb as u16));
                3
            },
            0x09 | 0x19 | 0x29 | 0x39 => {
                // ADD HL, r16
                let register = &FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();
                let reg_content = self.registers.read_double_register(register);
                let hl_content = self.registers.read_double_register(&DoubleRegister::HL);
                let result = hl_content.wrapping_add(reg_content);
                self.registers.write_double_register(&DoubleRegister::HL, result);
                self.registers.unset_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, (hl_content & 0xfff) + (reg_content & 0xfff) > 0xfff);
                self.registers.write_flag(&Flag::C, hl_content > 0xffff - reg_content);
                2
            },
            0x02 | 0x12 | 0x22 | 0x32 => {
                // LD [r16mem], A
                let dest_register = &FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();

                let data = self.registers.read_register(&Register::A);

                self.memory.write(self.registers.read_double_register_mem(dest_register), data);
                2
            },
            0x0A | 0x1A | 0x2A | 0x3A => {
                // LD A, [r16mem]
                let source_register = &FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();

                let data = self.memory.read(self.registers.read_double_register_mem(source_register));
                self.registers.write_register(&Register::A, data);
                2
            },
            0x03 | 0x13 | 0x23 | 0x33 => {
                // INC r16
                let register = &FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();
                self.registers.write_double_register(register, self.registers.read_double_register(register).wrapping_add(1));
                2
            },
            0x0B | 0x1B | 0x2B | 0x3B => {
                // DEC r16
                let register = &FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();
                self.registers.write_double_register(register, self.registers.read_double_register(register).wrapping_sub(1));
                2
            },
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
                // INC r8
                let register = &FromPrimitive::from_u8((byte >> 3) & 0b00000111).unwrap();

                let (cycles, old_value) = match register {
                    Register::HL => {
                        let old = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                        self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), old.wrapping_add(1));
                        (3, old)                    
                    },
                    _ => {
                        let old = self.registers.read_register(register);
                        self.registers.write_register(register, old.wrapping_add(1));
                        (1, old)
                    }
                };

                self.registers.write_flag(&Flag::Z, old_value.wrapping_add(1) == 0);
                self.registers.unset_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, old_value & 0xf == 0xf);
                cycles
            },
            0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => {
                // DEC r8
                let register = &FromPrimitive::from_u8((byte >> 3) & 0b00000111).unwrap();

                let (cycles, old_value) = match register {
                    Register::HL => {
                        let old = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                        self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), old.wrapping_sub(1));
                        (3, old)
                    },
                    _ => {
                        let old = self.registers.read_register(register);
                        self.registers.write_register(register, old.wrapping_sub(1));
                        (1, old)
                    }
                };
                self.registers.write_flag(&Flag::Z, old_value.wrapping_sub(1) == 0);
                self.registers.set_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, old_value & 0xf == 0);
                cycles
            },
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                // LD r8, u8
                let register = &FromPrimitive::from_u8((byte >> 3) & 0b00000111).unwrap();
                let n = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                if let Register::HL = register {
                    self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), n);
                    3
                } else { 
                    self.registers.write_register(register, n);
                    2 
                }
            },
            0x07 => {
                // RLCA
                self.registers.write_register(&Register::A, self.registers.read_register(&Register::A).rotate_left(1));
                self.registers.unset_flag(&Flag::Z);
                self.registers.unset_flag(&Flag::N);
                self.registers.unset_flag(&Flag::H);
                self.registers.write_flag(&Flag::C, self.registers.read_register(&Register::A) & 1 == 1);
                1
            },
            0x0F => {
                // RRCA
                self.registers.unset_flag(&Flag::Z);
                self.registers.unset_flag(&Flag::N);
                self.registers.unset_flag(&Flag::H);
                self.registers.write_flag(&Flag::C, self.registers.read_register(&Register::A) & 1 == 1);
                self.registers.write_register(&Register::A, self.registers.read_register(&Register::A).rotate_right(1));
                1
            },
            0x17 => {
                // RLA
                let a_value = self.registers.read_register(&Register::A);
                let flag = self.registers.read_flag(&Flag::C);
                self.registers.write_register(&Register::A, (a_value << 1) | (flag as u8));
                self.registers.unset_flag(&Flag::Z);
                self.registers.unset_flag(&Flag::N);
                self.registers.unset_flag(&Flag::H);
                self.registers.write_flag(&Flag::C, (a_value >> 7) & 1 == 1);
                1
            },
            0x1F => {
                // RRA
                let a_value = self.registers.read_register(&Register::A);
                let flag = self.registers.read_flag(&Flag::C);
                self.registers.write_register(&Register::A, (a_value >> 1) | ((flag as u8) << 7));
                self.registers.unset_flag(&Flag::Z);
                self.registers.unset_flag(&Flag::N);
                self.registers.unset_flag(&Flag::H);
                self.registers.write_flag(&Flag::C, a_value & 1 == 1);
                1
            },
            0x27 => {
                // DAA
                let mut a_value= self.registers.read_register(&Register::A);
                if !self.registers.read_flag(&Flag::N) {
                    if self.registers.read_flag(&Flag::C) || a_value > 0x99 {
                        self.registers.set_flag(&Flag::C);
                        a_value = a_value.wrapping_add(0x60);
                    }

                    if self.registers.read_flag(&Flag::H) || a_value & 0x0F > 0x09 {
                        a_value = a_value.wrapping_add(0x06);
                    }
                } else if self.registers.read_flag(&Flag::C) {
                    self.registers.set_flag(&Flag::C);
                    if self.registers.read_flag(&Flag::H) {
                        a_value = a_value.wrapping_add(0x9A);
                    } else {
                        a_value = a_value.wrapping_add(0xA0);
                    }
                } else if self.registers.read_flag(&Flag::H) {
                    a_value = a_value.wrapping_add(0xFA);
                }

                self.registers.write_flag(&Flag::Z, a_value == 0);
                self.registers.unset_flag(&Flag::H);

                self.registers.write_register(&Register::A, a_value);
                1
            },
            0x2F => {
                // CPL
                self.registers.write_register(&Register::A, !self.registers.read_register(&Register::A));
                self.registers.set_flag(&Flag::N);
                self.registers.set_flag(&Flag::H);
                1
            },
            0x37 => {
                // SCF
                self.registers.unset_flag(&Flag::N);
                self.registers.unset_flag(&Flag::H);
                self.registers.set_flag(&Flag::C);
                1
            },
            0x3F => {
                // CCF
                self.registers.unset_flag(&Flag::N);
                self.registers.unset_flag(&Flag::H);
                self.registers.toggle_flag(&Flag::C);
                1
            },

            /*
                GROUP 01
            */
            0x76 => {
                // HALT
                self.halted = true;
                1
            },
            0x40..=0x7F => {
                // LD r8, r8
                let dest = &FromPrimitive::from_u8((byte >> 3) & 0b00000111).unwrap();
                let source = &FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                let (cycles_source, data) = match source {

                    Register::HL => {
                        let data = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                        (2, data)
                    },
                    _ => {
                        let data = self.registers.read_register(source);
                        (1, data)
                    }
                }; 
                let cycles_dest = match dest {
                    Register::HL => {
                        self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), data);
                        2
                    },
                    _ => {
                        self.registers.write_register(dest, data);
                        1
                    }
                };
                std::cmp::max(cycles_source, cycles_dest)
            },
            
            /*
                GROUP 10
            */
            0x80..=0x87 => {
                // ADD A, r8
                let register = &FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                let a = self.registers.read_register(&Register::A);
                let (cycles, data) = match register {
                    Register::HL => (2, self.memory.read(self.registers.read_double_register(&DoubleRegister::HL))),
                    _ => (1, self.registers.read_register(register)),
                };

                let (result, overflow)= a.overflowing_add(data);
                self.registers.write_register(&Register::A, result);

                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.unset_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, (a & 0x0f).wrapping_add(data & 0x0f) > 0x0f);
                self.registers.write_flag(&Flag::C, overflow);
                cycles
            },
            0x88..=0x8F => {
                // ADC A, r8
                let register = &FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                let a = self.registers.read_register(&Register::A);
                let carry = self.registers.read_flag(&Flag::C) as u8;
                let (cycles, data) = match register {
                    Register::HL => (2, self.memory.read(self.registers.read_double_register(&DoubleRegister::HL))),
                    _ => (1, self.registers.read_register(register)),
                };

                let result= a.wrapping_add(data).wrapping_add(carry);
                self.registers.write_register(&Register::A, result);

                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.unset_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, (a & 0x0f) + (data & 0x0f) + carry > 0xf);
                self.registers.write_flag(&Flag::C, (a as u16) + (data as u16) + (carry as u16) > 0xff);
                cycles
            },
            0x90..=0x97 => {
                // SUB A, r8
                let register = &FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                let a = self.registers.read_register(&Register::A);
                let (cycles, data) = match register {
                    Register::HL => (2, self.memory.read(self.registers.read_double_register(&DoubleRegister::HL))),
                    _ => (1, self.registers.read_register(register)),
                };

                let (result, overflow )= a.overflowing_sub(data);
                self.registers.write_register(&Register::A, result);

                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.set_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, ((a & 0xf).wrapping_sub(data & 0xf)) & (0xf + 1) != 0);
                self.registers.write_flag(&Flag::C, overflow);
                cycles
            },
            0x98..=0x9F => {
                // SBC A, r8
                let register = &FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                let a = self.registers.read_register(&Register::A);
                let carry = self.registers.read_flag(&Flag::C) as u8;
                let (cycles, data) = match register {
                    Register::HL => (2, self.memory.read(self.registers.read_double_register(&DoubleRegister::HL))),
                    _ => (1, self.registers.read_register(register)),
                };

                let result = a.wrapping_sub(data).wrapping_sub(carry);
                self.registers.write_register(&Register::A, result);

                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.set_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, ((a & 0xf).wrapping_sub(data & 0xf).wrapping_sub(carry)) & (0xf + 1) != 0);
                self.registers.write_flag(&Flag::C, (a as u16) < (data as u16) + (carry as u16));
                cycles
            },
            0xA0..=0xA7 => {
                // AND A, r8
                let register = &FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                let a = self.registers.read_register(&Register::A);
                let (cycles, data) = match register {
                    Register::HL => (2, self.memory.read(self.registers.read_double_register(&DoubleRegister::HL))),
                    _ => (1, self.registers.read_register(register)),
                };
                let result = a & data;
                self.registers.write_register(&Register::A, result);

                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.unset_flag(&Flag::N);
                self.registers.set_flag(&Flag::H);
                self.registers.unset_flag(&Flag::C);

                cycles
            },
            0xA8..=0xAF => {
                // XOR A, r8
                let register = &FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                let (cycles, data) = match register {
                    Register::HL => {
                        let data = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                        (2, data)
                    },
                    _ => {
                        let data = self.registers.read_register(register);
                        (1, data)
                    }
                };
                let res = self.registers.read_register(&Register::A) ^ data;
                self.registers.write_register(&Register::A, res);

                self.registers.write_flag(&Flag::Z, res == 0);
                self.registers.unset_flag(&Flag::N);
                self.registers.unset_flag(&Flag::H);
                self.registers.unset_flag(&Flag::C);
                cycles
            },
            0xB0..=0xB7 => {
                // OR A, r8
                let register = &FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                let a = self.registers.read_register(&Register::A);
                let (cycles, data) = match register {
                    Register::HL => (2, self.memory.read(self.registers.read_double_register(&DoubleRegister::HL))),
                    _ => (1, self.registers.read_register(register)),
                };
                let result = a | data;
                self.registers.write_register(&Register::A, result);

                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.unset_flag(&Flag::N);
                self.registers.unset_flag(&Flag::H);
                self.registers.unset_flag(&Flag::C);

                cycles

            },
            0xB8..=0xBF => {
                // CP A, r8
                let register = &FromPrimitive::from_u8(byte & 0b00000111).unwrap();
                let a = self.registers.read_register(&Register::A);
                let (cycles, data) = match register {
                    Register::HL => (2, self.memory.read(self.registers.read_double_register(&DoubleRegister::HL))),
                    _ => (1, self.registers.read_register(register)),
                };

                let (result, overflow )= a.overflowing_sub(data);
                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.set_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, ((a & 0xf).wrapping_sub(data & 0xf)) & (0xf + 1) != 0);
                self.registers.write_flag(&Flag::C, overflow);
                cycles
            },

            /*
                GROUP 11
            */
            0xC0 | 0xC8 | 0xD0 | 0xD8 => {
                // RET cond
                let condition = &FromPrimitive::from_u8((byte >> 3) & 0b00000111).unwrap();
                if self.registers.check_condition(condition) {
                    let low_data = self.memory.read(self.registers.read_double_register(&DoubleRegister::SP));
                    self.registers.increase_sp(1);
                    let high_data = self.memory.read(self.registers.read_double_register(&DoubleRegister::SP));
                    self.registers.increase_sp(1);
                    self.registers.write_pc(((high_data as u16) << 8) | low_data as u16);

                    5
                } else {
                    2
                }
            },
            0xC1 | 0xD1 | 0xE1 | 0xF1 => {
                // POP r16stk
                let register = &FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();
                let low_data = self.memory.read(self.registers.read_double_register(&DoubleRegister::SP));
                self.registers.increase_sp(1);
                let high_data = self.memory.read(self.registers.read_double_register(&DoubleRegister::SP));
                self.registers.increase_sp(1);
                self.registers.write_double_register_stack(register, ((high_data as u16) << 8) | low_data as u16);
                3
            },
            0xC9 => {
                // RET
                let low_data = self.memory.read(self.registers.read_double_register(&DoubleRegister::SP));
                self.registers.increase_sp(1);
                let high_data = self.memory.read(self.registers.read_double_register(&DoubleRegister::SP));
                self.registers.increase_sp(1);
                self.registers.write_pc(((high_data as u16) << 8) | low_data as u16);
                4
            },
            0xD9 => {
                // RETI
                self.enable_interrupts = true;
                let low_data = self.memory.read(self.registers.read_double_register(&DoubleRegister::SP));
                self.registers.increase_sp(1);
                let high_data = self.memory.read(self.registers.read_double_register(&DoubleRegister::SP));
                self.registers.increase_sp(1);
                self.registers.write_pc(((high_data as u16) << 8) | low_data as u16);
                4

            },
            0xC2 | 0xCA | 0xD2 | 0xDA => {
                // JP cond, u16
                let condition = &FromPrimitive::from_u8((byte >> 3) & 0b00000011).unwrap();
                let lsb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let msb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let address = (msb as u16) << 8 | (lsb as u16);
                
                if self.registers.check_condition(condition) {
                    self.registers.write_pc(address);
                    4
                } else {
                    3
                }
            },
            0xC3 => {
                // JP u16
                let lsb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let msb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let address = (msb as u16) << 8 | (lsb as u16);
                self.registers.write_pc(address);
                4
            },
            0xC4 | 0xCC | 0xD4 | 0xDC => {
                // CALL cond, u16
                let condition = &FromPrimitive::from_u8((byte >> 3) & 0b00000011).unwrap();
                let lsb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let msb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let address = (msb as u16) << 8 | (lsb as u16);

                if self.registers.check_condition(condition) {
                    self.registers.decrement_sp(1);
                    self.memory.write(self.registers.read_double_register(&DoubleRegister::SP), (self.registers.read_pc() >> 8) as u8);
                    self.registers.decrement_sp(1);
                    self.memory.write(self.registers.read_double_register(&DoubleRegister::SP), self.registers.read_pc() as u8);
                    self.registers.write_pc(address);
                    6
                } else {
                    3
                }
            },
            0xC5 | 0xD5 | 0xE5 | 0xF5 => {
                // PUSH r16stk
                let register = &FromPrimitive::from_u8((byte >> 4) & 0b00000011).unwrap();
                let data = self.registers.read_double_register_stack(register);
                self.registers.decrement_sp(1);
                self.memory.write(self.registers.read_double_register(&DoubleRegister::SP), (data >> 8) as u8);
                self.registers.decrement_sp(1);
                self.memory.write(self.registers.read_double_register(&DoubleRegister::SP), data as u8);
                4
            },
            0xCD => {
                // CALL u16
                let lsb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let msb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let address = (msb as u16) << 8 | (lsb as u16);
                self.registers.decrement_sp(1);
                self.memory.write(self.registers.read_double_register(&DoubleRegister::SP), (self.registers.read_pc() >> 8) as u8);
                self.registers.decrement_sp(1);
                self.memory.write(self.registers.read_double_register(&DoubleRegister::SP), self.registers.read_pc() as u8);
                self.registers.write_pc(address);
                6
            },
            0xC6 => {
                // ADD A, u8
                let value = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let a = self.registers.read_register(&Register::A);

                let (result, overflow)= a.overflowing_add(value);
                self.registers.write_register(&Register::A, result);

                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.unset_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, (a & 0x0f).wrapping_add(value & 0x0f) > 0x0f);
                self.registers.write_flag(&Flag::C, overflow);
                2
            },
            0xCE => {
                // ADC A, u8
                let value = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let a = self.registers.read_register(&Register::A);
                let carry = self.registers.read_flag(&Flag::C) as u8;

                let result= a.wrapping_add(value).wrapping_add(carry);
                self.registers.write_register(&Register::A, result);

                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.unset_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, (a & 0x0f).wrapping_add(value & 0x0f).wrapping_add(carry) > 0x0f);
                self.registers.write_flag(&Flag::C, a as u16 + value as u16 + carry as u16 > 0xff);
                2
            },
            0xD6 => {
                // SUB A, u8
                let value = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let a = self.registers.read_register(&Register::A);

                let (result, overflow )= a.overflowing_sub(value);
                self.registers.write_register(&Register::A, result);

                self.registers.write_flag(&Flag::C, overflow);
                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.write_flag(&Flag::H, ((a & 0xf).wrapping_sub(value & 0xf)) & (0xf + 1) != 0);
                self.registers.set_flag(&Flag::N);
                2
            },
            0xDE => {
                // SBC A, u8
                let value = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let a = self.registers.read_register(&Register::A);
                let carry = self.registers.read_flag(&Flag::C) as u8;

                let result = a.wrapping_sub(value).wrapping_sub(carry);
                self.registers.write_register(&Register::A, result);

                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.set_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, (a & 0xf).wrapping_sub(value & 0xf).wrapping_sub(carry) & (0xf + 1) != 0);
                self.registers.write_flag(&Flag::C, (a as u16) < (value as u16) + (carry as u16));
                2
            },
            0xE6 => {
                // AND A, u8
                let value = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let a = self.registers.read_register(&Register::A);
                let result = a & value;
                self.registers.write_register(&Register::A, result);

                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.unset_flag(&Flag::N);
                self.registers.set_flag(&Flag::H);
                self.registers.unset_flag(&Flag::C);

                2
            },
            0xEE => {
                // XOR A, u8
                let value = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let result = self.registers.read_register(&Register::A) ^ value;
                self.registers.write_register(&Register::A, result);

                self.registers.unset_flag(&Flag::N);
                self.registers.unset_flag(&Flag::H);
                self.registers.unset_flag(&Flag::C);
                self.registers.write_flag(&Flag::Z, result == 0);
                2
            },
            0xF6 => {
                // OR A, u8
                let value = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let a = self.registers.read_register(&Register::A);
                let result = a | value;
                self.registers.write_register(&Register::A, result);

                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.unset_flag(&Flag::N);
                self.registers.unset_flag(&Flag::H);
                self.registers.unset_flag(&Flag::C);

                2
            },
            0xFE => {
                // CP A, u8
                let value = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let a = self.registers.read_register(&Register::A);
                let (result, overflow )= a.overflowing_sub(value);
                self.registers.write_flag(&Flag::Z, result == 0);
                self.registers.set_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, ((a & 0xf).wrapping_sub(value & 0xf)) & (0xf + 1) != 0);
                self.registers.write_flag(&Flag::C, overflow);

                2
            },
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                // RST tgt3
                let target= ((byte >> 3) & 0b111) as u16 * 8;

                self.registers.decrement_sp(1);
                self.memory.write(self.registers.read_double_register(&DoubleRegister::SP), (self.registers.read_pc() >> 8) as u8);
                self.registers.decrement_sp(1);
                self.memory.write(self.registers.read_double_register(&DoubleRegister::SP), self.registers.read_pc() as u8);
                self.registers.write_pc(target);
                
                4
            },
            0xE0 => {
                // LD (FF00+u8), A
                let offset = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();

                self.memory.write(0xff00 + offset as u16, self.registers.read_register(&Register::A));
                3
            },
            0xF0 => {
                // LD A, (FF00+u8)
                let offset = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();

                self.registers.write_register(&Register::A, self.memory.read(0xff00 + offset as u16));
                3
            },
            0xE8 => {
                // ADD SP, i8
                let value = self.memory.read(self.registers.read_pc()) as i8 as i16;
                self.registers.increase_pc();
                let sp = self.registers.read_double_register(&DoubleRegister::SP);

                let result = sp.wrapping_add_signed(value);
                self.registers.write_double_register(&DoubleRegister::SP, result);
                
                self.registers.unset_flag(&Flag::Z);
                self.registers.unset_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, (sp & 0x0f).wrapping_add(value as u16 & 0x0f) > 0x0f);
                self.registers.write_flag(&Flag::C, (sp & 0xff).wrapping_add(value as u16 & 0xff) > 0xff);
                4
            },
            0xF8 => {
                // LD HL, SP+i8
                let value = self.memory.read(self.registers.read_pc()) as i8 as i16;
                self.registers.increase_pc();
                let sp = self.registers.read_double_register(&DoubleRegister::SP);
                let result = sp.wrapping_add_signed(value);
                self.registers.write_double_register(&DoubleRegister::HL, result);

                self.registers.unset_flag(&Flag::Z);
                self.registers.unset_flag(&Flag::N);
                self.registers.write_flag(&Flag::H, (sp & 0x0f).wrapping_add(value as u16 & 0x0f) > 0x0f);
                self.registers.write_flag(&Flag::C, (sp & 0xff).wrapping_add(value as u16 & 0xff) > 0xff);
                3                
            },
            0xE9 => {
                // JP HL
                let address = self.registers.read_double_register(&DoubleRegister::HL);
                self.registers.write_pc(address);
                1
            },
            0xF9 => {
                // LD SP, HL
                self.registers.write_double_register(&DoubleRegister::SP, self.registers.read_double_register(&DoubleRegister::HL));
                2
            },
            0xE2 => {
                // LD (FF00+C), A
                let a = self.registers.read_register(&Register::A);
                let c = self.registers.read_register(&Register::C);

                self.memory.write(0xff00 + c as u16, a);
                2
            },
            0xEA => {
                // LD (u16), A
                let a = self.registers.read_register(&Register::A);
                let lsb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let msb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let address = (msb as u16) << 8 | (lsb as u16);

                self.memory.write(address, a);
                4
            },
            0xF2 => {
                // LD A, (FF00+C)
                let c = self.registers.read_register(&Register::C);

                self.registers.write_register(&Register::A, self.memory.read(0xff00 + c as u16));
                2
            },
            0xFA => {
                // LD A, (u16)
                let lsb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let msb = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                let address = (msb as u16) << 8 | (lsb as u16);

                self.registers.write_register(&Register::A, self.memory.read(address));
                4
            },
            0xF3 => {
                // DI
                self.ime = false;
                1
            },
            0xFB => {
                // EI
                self.enable_interrupts = true;
                1
            },
            
            /*
                Group CB
            */
            0xCB => {
                let cb_instruction: u8 = self.memory.read(self.registers.read_pc());
                self.registers.increase_pc();
                
                match cb_instruction {
                    0x00..=0x07 => {
                        // RLC r8
                        let register = &FromPrimitive::from_u8(cb_instruction & 0b00000111).unwrap();
                        let (cycles, result) = match register {
                            Register::HL => {
                                let r = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL)).rotate_left(1);
                                self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), r);
                                (4, r)
                            },
                            _ => {
                                let r = self.registers.read_register(register).rotate_left(1);
                                self.registers.write_register(register, r);
                                (2, r)
                            }    
                        };
                        self.registers.write_flag(&Flag::Z, result == 0);
                        self.registers.unset_flag(&Flag::N);
                        self.registers.unset_flag(&Flag::H);
                        self.registers.write_flag(&Flag::C, result & 1 == 1);
                        cycles
                    },
                    0x08..=0x0F => {
                        // RRC r8
                        let register = &FromPrimitive::from_u8(cb_instruction & 0b00000111).unwrap();
                        let (cycles, result) = match register {
                            Register::HL => {
                                let r = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                                let result = r.rotate_right(1);
                                self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), result);
                                (4, result)
                            },
                            _ => {
                                let r = self.registers.read_register(register);
                                let result = r.rotate_right(1);
                                self.registers.write_register(register, result);
                                (2, result)
                            }
                        };
                        self.registers.write_flag(&Flag::Z, result == 0);
                        self.registers.unset_flag(&Flag::N);
                        self.registers.unset_flag(&Flag::H);
                        self.registers.write_flag(&Flag::C, (result >> 7) & 1 == 1);
                        cycles
                    },
                    0x10..=0x17 => {
                        // RL r8
                        let register = &FromPrimitive::from_u8(cb_instruction & 0b00000111).unwrap();
                        let flag = self.registers.read_flag(&Flag::C);
                        let (cycles, old, result) = match register {
                            Register::HL => {
                                let old = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                                let result = (old << 1) | flag as u8;
                                self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), result);
                                (4, old, result)
                            },
                            _ => {
                                let old = self.registers.read_register(register);
                                let result = (old << 1) | flag as u8;
                                self.registers.write_register(register, result);
                                (2, old, result)
                            }
                        };

                        self.registers.write_flag(&Flag::Z, result == 0);
                        self.registers.unset_flag(&Flag::N);
                        self.registers.unset_flag(&Flag::H);
                        self.registers.write_flag(&Flag::C, (old & 0b10000000) >> 7 == 1);
                        cycles
                    },
                    0x18..=0x1F => {
                        // RR r8
                        let register = &FromPrimitive::from_u8(cb_instruction & 0b00000111).unwrap();
                        let flag = self.registers.read_flag(&Flag::C);
                        let (cycles, old, result) = match register {
                            Register::HL => {
                                let old = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                                let result = (old >> 1) | (flag as u8) << 7;
                                self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), result);
                                (4, old, result)
                            },
                            _ => {
                                let old = self.registers.read_register(register);
                                let result = (old >> 1) | (flag as u8) << 7;
                                self.registers.write_register(register, result);
                                (2, old, result)
                            }
                        };

                        self.registers.write_flag(&Flag::Z, result == 0);
                        self.registers.unset_flag(&Flag::N);
                        self.registers.unset_flag(&Flag::H);
                        self.registers.write_flag(&Flag::C, old & 1 == 1);
                        cycles
                    },
                    0x20..=0x27 => {
                        // SLA r8
                        let register = &FromPrimitive::from_u8(cb_instruction & 0b00000111).unwrap();
                        let (cycles, old_value) = match register {
                            Register::HL => {
                                let old = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                                self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), old << 1);
                                (4, old)
                            },
                            _ => {
                                let old = self.registers.read_register(register);
                                self.registers.write_register(register, old << 1);
                                (2, old)
                            }
                        };
                        self.registers.write_flag(&Flag::Z, old_value << 1 == 0);
                        self.registers.unset_flag(&Flag::N);
                        self.registers.unset_flag(&Flag::H);
                        self.registers.write_flag(&Flag::C, (old_value >> 7) & 1 == 1);
                        cycles
                    },
                    0x28..=0x2F => {
                        // SRA r8
                        let register = &FromPrimitive::from_u8(cb_instruction & 0b00000111).unwrap();
                        let (cycles, old_value) = match register {
                            Register::HL => {
                                let old = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                                self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), (old >> 1) | (old & 0b10000000) );
                                (4, old)
                            },
                            _ => {
                                let old = self.registers.read_register(register);
                                self.registers.write_register(register, (old >> 1) | (old & 0b10000000));
                                (2, old)
                            }
                        };
                        self.registers.write_flag(&Flag::Z, (old_value >> 1) | (old_value & 0b10000000) == 0);
                        self.registers.unset_flag(&Flag::N);
                        self.registers.unset_flag(&Flag::H);
                        self.registers.write_flag(&Flag::C, old_value & 1 == 1);
                        cycles
                    },
                    0x30..=0x37 => {
                        // SWAP r8
                        let register = &FromPrimitive::from_u8(cb_instruction & 0b00000111).unwrap();
                        let (cycles, old_value) = match register {
                            Register::HL => {
                                let r = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                                self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), ((r & 0x0f) << 4) | (r >> 4));
                                (4, r)
                            },
                            _ => {
                                let r = self.registers.read_register(register);
                                self.registers.write_register(register, ((r & 0x0f) << 4) | (r >> 4));
                                (2, r)
                            }
                        };
                        self.registers.write_flag(&Flag::Z, old_value == 0);
                        self.registers.unset_flag(&Flag::N);
                        self.registers.unset_flag(&Flag::H);
                        self.registers.unset_flag(&Flag::C);
                        cycles
                    },
                    0x38..=0x3F => {
                        // SRL r8
                        let register = &FromPrimitive::from_u8(cb_instruction & 0b00000111).unwrap();
                        let (cycles, old_value) = match register {
                            Register::HL => {
                                let old = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                                self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), old >> 1);
                                (4, old)
                            },
                            _ => {
                                let old = self.registers.read_register(register);
                                self.registers.write_register(register, old >> 1);
                                (2, old)
                            }
                        };
                        self.registers.write_flag(&Flag::Z, old_value >> 1 == 0);
                        self.registers.unset_flag(&Flag::N);
                        self.registers.unset_flag(&Flag::H);
                        self.registers.write_flag(&Flag::C, old_value & 1 == 1);
                        cycles
                    },
                    0x40..=0x7F => {
                        // BIT b3, r8
                        let register = &FromPrimitive::from_u8(cb_instruction & 0b00000111).unwrap();
                        let bit = (cb_instruction >> 3) & 0b00000111;

                        let (cycles, value) =  match register {
                            Register::HL => {
                                (3, self.memory.read(self.registers.read_double_register(&DoubleRegister::HL)))
                            },
                            _ => {
                                (2, self.registers.read_register(register))
                            }
                        };
                        self.registers.write_flag(&Flag::Z, value >> bit & 1 == 0);
                        self.registers.unset_flag(&Flag::N);
                        self.registers.set_flag(&Flag::H);

                        cycles
                    },
                    0x80..=0xBF => {
                        // RES b3, r8
                        let register = &FromPrimitive::from_u8(cb_instruction & 0b00000111).unwrap();
                        let bit = (cb_instruction >> 3) & 0b00000111;

                        if let Register::HL = register {
                            let value = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                            self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), value & !(1 << bit));
                            4
                        } else {
                            self.registers.write_register(register, self.registers.read_register(register) & !(1 << bit));
                            2
                        }
                    },
                    0xC0..=0xFF => {
                        // SET b3, r8
                        let register = &FromPrimitive::from_u8(cb_instruction & 0b00000111).unwrap();
                        let bit = (cb_instruction >> 3) & 0b00000111;

                        if let Register::HL = register {
                            let value = self.memory.read(self.registers.read_double_register(&DoubleRegister::HL));
                            self.memory.write(self.registers.read_double_register(&DoubleRegister::HL), value | (1 << bit));
                            4
                        } else {
                            self.registers.write_register(register, self.registers.read_register(register) | (1 << bit));
                            2
                        }

                    }
                }
            }           
            _ => panic!("Unrecognized instruction: {}", byte)
        }
    }

    fn jump_interrupt(&mut self, target: u16) {
        let pc = self.registers.read_pc();
        self.registers.decrement_sp(1);
        self.memory.write(self.registers.read_double_register(&DoubleRegister::SP), (pc >> 8) as u8);
        self.registers.decrement_sp(1);
        self.memory.write(self.registers.read_double_register(&DoubleRegister::SP), (pc & 0xff) as u8);
        self.registers.write_pc(target);
    }

    fn handle_interrupts(&mut self) -> bool {

        let mut interrupted = true;
        if self.memory.get_interrupts().is_enabled_and_requested(&interrupt::InterruptHandler::VBlank){
            self.jump_interrupt(0x40);
            self.memory.get_interrupts().write_bit_interrupt_flag(&interrupt::InterruptHandler::VBlank, false);
        } else if self.memory.get_interrupts().is_enabled_and_requested(&interrupt::InterruptHandler::LCD) {
            self.jump_interrupt(0x48);
            self.memory.get_interrupts().write_bit_interrupt_flag(&interrupt::InterruptHandler::LCD, false);
        } else if self.memory.get_interrupts().is_enabled_and_requested(&interrupt::InterruptHandler::Timer) {
            self.jump_interrupt(0x50);
            self.memory.get_interrupts().write_bit_interrupt_flag(&interrupt::InterruptHandler::Timer, false);
        } else if self.memory.get_interrupts().is_enabled_and_requested(&interrupt::InterruptHandler::Serial){
            self.jump_interrupt(0x58);
            self.memory.get_interrupts().write_bit_interrupt_flag(&interrupt::InterruptHandler::Serial, false);
        } else if self.memory.get_interrupts().is_enabled_and_requested(&interrupt::InterruptHandler::Joypad) {
            self.jump_interrupt(0x60);
            self.memory.get_interrupts().write_bit_interrupt_flag(&interrupt::InterruptHandler::Joypad, false);
        } else {
            interrupted = false;
        }

        interrupted
    }
}
