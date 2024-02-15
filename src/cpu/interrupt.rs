pub enum InterruptHandler {
    VBlank,
    LCD,
    Timer,
    Serial,
    Joypad
}

pub struct Interrupt {
    interrupt_enable: u8,
    interrupt_flag: u8,
}

impl Interrupt {
    
    pub fn new() -> Interrupt {
        Interrupt { interrupt_enable: 0, interrupt_flag: 0 }
    }

    pub fn read_interrupt_enable(&self) -> u8 {
        self.interrupt_enable
    }

    pub fn write_interrupt_enable(&mut self, value: u8) {
        self.interrupt_enable = value;
    }

    pub fn write_interrupt_flag(&mut self, value: u8) {
        self.interrupt_flag = value;
    }

    pub fn write_bit_interrupt_enable(&mut self, interrupt: &InterruptHandler, value: bool) {
        let bit = value as u8;
        match interrupt {
            InterruptHandler::VBlank => self.interrupt_enable = (self.interrupt_enable & !1) | bit,
            InterruptHandler::LCD => self.interrupt_enable = (self.interrupt_enable & !(1 << 1)) | (bit << 1),
            InterruptHandler::Timer => self.interrupt_enable = (self.interrupt_enable & !(1 << 2)) | (bit << 2),
            InterruptHandler::Serial => self.interrupt_enable = (self.interrupt_enable & !(1 << 3)) | (bit << 3),
            InterruptHandler::Joypad => self.interrupt_enable = (self.interrupt_enable & !(1 << 4)) | (bit << 4),
        }
    }

    pub fn write_bit_interrupt_flag(&mut self, interrupt: &InterruptHandler, value: bool) {
        let bit = value as u8;
        match interrupt {
            InterruptHandler::VBlank => self.interrupt_flag = (self.interrupt_flag & !1) | bit,
            InterruptHandler::LCD => self.interrupt_flag = (self.interrupt_flag & !(1 << 1)) | (bit << 1),
            InterruptHandler::Timer => self.interrupt_flag = (self.interrupt_flag & !(1 << 2)) | (bit << 2),
            InterruptHandler::Serial => self.interrupt_flag = (self.interrupt_flag & !(1 << 3)) | (bit << 3),
            InterruptHandler::Joypad => self.interrupt_flag = (self.interrupt_flag & !(1 << 4)) | (bit << 4),
        }
    }

    pub fn is_enabled(&self, interrupt: &InterruptHandler) -> bool {
        match interrupt {
            InterruptHandler::VBlank => self.interrupt_enable &0x01 != 0,
            InterruptHandler::LCD => self.interrupt_enable & 0x02 != 0,
            InterruptHandler::Timer => self.interrupt_enable & 0x04 != 0,
            InterruptHandler::Serial => self.interrupt_enable & 0x08 != 0,
            InterruptHandler::Joypad => self.interrupt_enable & 0x10 != 0,
        }
    }

    pub fn is_requested(&self, interrupt: &InterruptHandler) -> bool {
        match interrupt {
            InterruptHandler::VBlank => self.interrupt_flag &0x01 != 0,
            InterruptHandler::LCD => self.interrupt_flag & 0x02 != 0,
            InterruptHandler::Timer => self.interrupt_flag & 0x04 != 0,
            InterruptHandler::Serial => self.interrupt_flag & 0x08 != 0,
            InterruptHandler::Joypad => self.interrupt_flag & 0x10 != 0,
        }
    }

    pub fn is_enabled_and_requested(&self, interrupt: &InterruptHandler) -> bool {
        self.is_enabled(interrupt) && self.is_requested(interrupt)
    }
}
