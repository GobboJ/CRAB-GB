use num_derive::FromPrimitive;
use num_traits::WrappingSub;

#[derive(FromPrimitive)]
pub enum Register {
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
    HL_MEM = 0b110,
    A = 0b111,
}

#[derive(FromPrimitive)]
pub enum DoubleRegister {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    SP = 0b11
}

#[derive(FromPrimitive)]
pub enum DoubleRegisterStack {
    BC,
    DE,
    HL,
    AF
}

#[derive(FromPrimitive)]
pub enum DoubleRegisterMem {
    BC,
    DE,
    HLI,
    HLD
}

pub enum Flag {
    Z,
    N,
    H,
    C
}

#[derive(FromPrimitive)]
pub enum Condition {
    NZ,
    Z,
    NC,
    C
}


pub struct Registers {
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    a: u8,
    f: u8,

    sp: u16,
    pc: u16
}

impl Registers {
    pub fn read_register(&self, r: Register) -> u8 {
        match r {
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::A => self.a,
            Register::HL_MEM => panic!("(HL) is not a register!"),
        }
    }

    pub fn write_register(&mut self, r: Register, value: u8) {
        match r {
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::A => self.a = value,
            Register::HL_MEM => panic!("(HL) is not a register!"),
        }
    }

    pub fn read_double_register(&self, rr: DoubleRegister) -> u16 {
        match rr {
            DoubleRegister::BC => (self.b as u16) << 8 | self.c as u16,
            DoubleRegister::DE => (self.d as u16) << 8 | self.e as u16,
            DoubleRegister::HL => (self.h as u16) << 8 | self.l as u16,
            DoubleRegister::SP => self.sp
        }
    }

    pub fn read_double_register_stack(&self, rr: DoubleRegisterStack) -> u16 {
        match rr {
            DoubleRegisterStack::BC => (self.b as u16) << 8 | self.c as u16,
            DoubleRegisterStack::DE => (self.d as u16) << 8 | self.e as u16,
            DoubleRegisterStack::HL => (self.h as u16) << 8 | self.l as u16,
            DoubleRegisterStack::AF => (self.a as u16) << 8 | self.f as u16
        }
    }

    pub fn read_double_register_mem(&self, rr: DoubleRegisterMem) -> u16 {
        match rr {
            DoubleRegisterMem::BC => (self.b as u16) << 8 | self.c as u16,
            DoubleRegisterMem::DE => (self.d as u16) << 8 | self.e as u16,
            DoubleRegisterMem::HLI => {
                let ret = (self.h as u16) << 8 | self.l as u16;
                let mut inc = ret.wrapping_add(1);
                self.h = (inc >> 8) as u8;
                self.l = inc as u8;
                ret
            },
            DoubleRegisterMem::HLD => {
                let ret = (self.h as u16) << 8 | self.l as u16;
                let mut dec = ret.wrapping_sub(1);
                self.h = (dec >> 8) as u8;
                self.l = dec as u8;
                ret
            },
        }
    }

    pub fn write_double_register(&mut self, rr: DoubleRegister, value: u16) {
        match rr {
            DoubleRegister::BC => {
                self.b = (value >> 8) as u8;
                self.c = value as u8;
            },
            DoubleRegister::DE => {
                self.d = (value >> 8) as u8;
                self.e = value as u8;
            },
            DoubleRegister::HL => {
                self.h = (value >> 8) as u8;
                self.l = value as u8;
            },
            DoubleRegister::SP => {
                self.sp = value;
            },
        }
    }

    pub fn write_double_register_stack(&mut self, rr: DoubleRegisterStack, value: u16) {
        match rr {
            DoubleRegisterStack::BC => {
                self.b = (value >> 8) as u8;
                self.c = value as u8;
            },
            DoubleRegisterStack::DE => {
                self.d = (value >> 8) as u8;
                self.e = value as u8;
            },
            DoubleRegisterStack::HL => {
                self.h = (value >> 8) as u8;
                self.l = value as u8;
            },
            DoubleRegisterStack::AF => {
                self.a = (value >> 8) as u8;
                self.f = value as u8;
            },
        }
    }

    pub fn new() -> Registers {
        Registers { b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, a: 0, f: 0, sp: 0, pc: 0 }
    }

    pub fn read_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => self.f & 0x80 != 0,
            Flag::N => self.f & 0x40 != 0,
            Flag::H => self.f & 0x20 != 0,
            Flag::C => self.f & 0x10 != 0,
        }
    }

    pub fn set_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Z => self.f |= 1 << 7,
            Flag::N => self.f |= 1 << 6,
            Flag::H => self.f |= 1 << 5,
            Flag::C => self.f |= 1 << 4,
        }
    }

    pub fn unset_flag(&mut self, flag: Flag) {
        match flag {
            Flag::Z => self.f &= !(1 << 7),
            Flag::N => self.f &= !(1 << 6),
            Flag::H => self.f &= !(1 << 5),
            Flag::C => self.f &= !(1 << 4),
        }
    }

    pub fn toggle_flag(mut self, flag: Flag) {
        match flag {
            Flag::Z => self.f ^= 1 << 7,
            Flag::N => self.f ^= 1 << 6,
            Flag::H => self.f ^= 1 << 5,
            Flag::C => self.f ^= 1 << 4,
        }
    }

    pub fn write_flag(mut self, flag: Flag, value: bool) {
        let bit = value as u8;
        match flag {
            Flag::Z => self.f = (self.f & !(bit << 7)) | (bit << 7),
            Flag::N => self.f = (self.f & !(bit << 6)) | (bit << 6),
            Flag::H => self.f = (self.f & !(bit << 5)) | (bit << 5),
            Flag::C => self.f = (self.f & !(bit << 4)) | (bit << 4),
        }
    }

    pub fn check_condition(&self, cond: Condition) -> bool {
        match cond {
            Condition::NZ => !self.read_flag(Flag::Z),
            Condition::Z => self.read_flag(Flag::Z),
            Condition::NC => !self.read_flag(Flag::C),
            Condition::C => self.read_flag(Flag::C),
        }
    }


    pub fn read_pc(&self) -> u16 {
        self.pc
    }

    pub fn increase_pc(&mut self, value: u16) {
        self.pc = self.pc.wrapping_add(value);
    }

    pub fn write_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn increase_sp(&mut self, value: u16) {
        self.sp = self.sp.wrapping_add(value);
    }
}
