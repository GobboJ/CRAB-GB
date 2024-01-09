use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum Register {
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
    A = 0b111,
}

#[derive(FromPrimitive)]
pub enum DoubleRegister {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    AF = 0b11
}

pub enum Flag {
    Zero,
    Sub,
    HalfCarry,
    Carry
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
        }
    }

    pub fn set_register(&mut self, r: Register, value: u8) {
        match r {
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::A => self.a = value,
        }
    }

    pub fn read_double_register(&self, rr: DoubleRegister) -> u16 {
        match rr {
            DoubleRegister::BC => (self.b as u16) << 8 | self.c as u16,
            DoubleRegister::DE => (self.d as u16) << 8 | self.e as u16,
            DoubleRegister::HL => (self.h as u16) << 8 | self.l as u16,
            DoubleRegister::AF => (self.a as u16) << 8 | self.f as u16
        }
    }

    pub fn set_double_register(&mut self, rr: DoubleRegister, value: u16) {
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
            DoubleRegister::AF => {
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
            Flag::Zero => self.f & 0x80 != 0,
            Flag::Sub => self.f & 0x40 != 0,
            Flag::HalfCarry => self.f & 0x20 != 0,
            Flag::Carry => self.f & 0x10 != 0,
        }
    }

    pub fn set_flag(&self, flag: Flag, value: bool) {
        todo!()
    }
}
