use std::ops;

#[derive(PartialEq, Clone, Copy)]
pub struct Register(u8);

impl ops::Not for Register {
    type Output = Register;

    fn not(self) -> Self::Output {
        Register(!self.0)
    }
}

impl Register {


    pub fn new(value: u8) -> Self {
        Register(value)
    }

    pub fn to_u8(&self) -> u8 {
        self.0
    }

    pub fn write(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn high_nibble(&self) -> u8 {
        self.0 >> 4
    }

    pub fn low_nibble(&self) -> u8 {
        self.0 & 0xF
    }

    pub fn set_bit(&mut self, position: u8) {
        self.0 |= 1 << position;
    }

    pub fn unset_bit(&mut self, position: u8) {
        self.0 &= !(1 << position);
    }

    pub fn at(&self, position: u8) -> bool {
        self.0 & (1 << position) != 0
    }

}
