use super::register::Register;

#[derive(Clone, Copy)]
pub enum InterruptHandler {
    VBlank,
    LCD,
    Timer,
    Serial,
    Joypad
}

pub struct Interrupt {
    interrupt_enable: Register,
    interrupt_flag: Register,
}

impl Interrupt {
    
    pub fn new() -> Interrupt {
        Interrupt {
            interrupt_enable: Register::new(0),
            interrupt_flag: Register::new(0)
        }
    }

    pub fn read_interrupt_enable(&self) -> u8 {
        self.interrupt_enable.to_u8()
    }

    pub fn read_interrupt_flag(&self) -> u8 {
        self.interrupt_flag.to_u8()
    }

    pub fn write_interrupt_enable(&mut self, value: u8) {
        self.interrupt_enable.write(value);
    }

    pub fn write_interrupt_flag(&mut self, value: u8) {
        self.interrupt_flag.write(value);
    }

    // pub fn set_ie_bit(&mut self, interrupt: InterruptHandler) {
    //     self.interrupt_enable.set_bit(interrupt as u8);
    // }

    pub fn set_if_bit(&mut self, interrupt: InterruptHandler) {
        self.interrupt_flag.set_bit(interrupt as u8);
    }

    // pub fn unset_ie_bit(&mut self, interrupt: InterruptHandler) {
    //     self.interrupt_enable.unset_bit(interrupt as u8);
    // }

    pub fn unset_if_bit(&mut self, interrupt: InterruptHandler) {
        self.interrupt_flag.unset_bit(interrupt as u8);
    }

    pub fn is_enabled(&self, interrupt: InterruptHandler) -> bool {
        self.interrupt_enable.at(interrupt as u8)
    }

    pub fn is_requested(&self, interrupt: InterruptHandler) -> bool {
        self.interrupt_flag.at(interrupt as u8)
    }

    pub fn is_enabled_and_requested(&self, interrupt: InterruptHandler) -> bool {
        self.is_enabled(interrupt) && self.is_requested(interrupt)
    }

    pub fn is_pending(&self) -> bool {
        self.interrupt_enable.to_u8() & self.interrupt_flag.to_u8() != 0
    }
}
