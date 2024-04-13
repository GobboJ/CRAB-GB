use core::panic;
use super::register::Register;

#[derive(Clone, Copy)]
pub enum Button {
    R = 0,
    L = 1,
    U = 2,
    D = 3,
    A = 4,
    B = 5,
    SEL = 6,
    STA = 7,
}

#[derive(Clone, Copy)]
enum Column { 
    DPAD,
    BUTTONS,
    NONE 
}

pub struct Joypad {
    buttons: Register,
    column: Column
}

impl Joypad {

    pub fn new() -> Self {
        Joypad { buttons: Register::new(), column: Column::NONE }
    }

    pub fn read_register(&self) -> u8 {
        match self.column {
            Column::DPAD => (0b10 << 4) | self.buttons.low_nibble(),
            Column::BUTTONS => (0b01 << 4) | self.buttons.high_nibble(),
            Column::NONE => (0b11 << 4) | 0b1111,
        }
    }

    pub fn write_register(&mut self, value: u8) {
        match value >> 4 & 0b11 {
            0b01 => self.column = Column::BUTTONS,
            0b10 => self.column = Column::DPAD,
            0b11 => self.column = Column::NONE,
            _ => panic!("Unexpected state")
        }
    }

    pub fn set_button(&mut self, button: Button) -> bool {
        
        let old_value = self.buttons;
        self.buttons.unset_bit(button as u8);

        let request_joypad = if old_value != self.buttons {
            let is_dpad = match button {
                Button::R | Button::L | Button::U | Button::D => true,
                _ => false
            };

            match self.column {
                Column::DPAD => is_dpad,
                Column::BUTTONS => !is_dpad,
                Column::NONE => false,
            }
        } else {
            false
        };

        request_joypad
    }

    pub fn unset_button(&mut self, button: Button) {
        self.buttons.set_bit(button as u8);
    }
}
