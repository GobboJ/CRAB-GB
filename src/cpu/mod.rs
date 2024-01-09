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
}
