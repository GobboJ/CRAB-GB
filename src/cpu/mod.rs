mod registers;

use num_traits::FromPrimitive;
use registers::Registers;
use registers::Flag;

pub struct Cpu {

    registers: Registers 
   
}

impl Cpu {
    pub fn ciao(&mut self) {
        let a = FromPrimitive::from_u8(0b010).unwrap();
        let res = self.registers.read_register(a);
        self.registers.set_flag(Flag::Zero);
    }
}
