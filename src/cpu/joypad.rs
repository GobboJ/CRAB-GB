use core::panic;

pub enum Button {
    A,
    B,
    SEL,
    STA,
    R,
    L,
    U,
    D
}

#[derive(Clone, Copy)]
enum Column { 
    DPAD,
    BUTTONS,
    NONE 
}

pub struct Joypad {
    buttons: u8,
    column: Column
}

impl Joypad {

    pub fn new() -> Self {
        Joypad { buttons: 0, column: Column::NONE }
    }

    pub fn read_register(&self) -> u8 {
        match self.column {
            Column::DPAD => (0b10 << 4) | (!self.buttons & 0xF),
            Column::BUTTONS => (0b01 << 4) | (!self.buttons >> 4),
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
        match button {
            Button::R => self.buttons |= 1,
            Button::L => self.buttons |= 1 << 1,
            Button::U => self.buttons |= 1 << 2,
            Button::D => self.buttons |= 1 << 3,
            Button::A => self.buttons |= 1 << 4,
            Button::B => self.buttons |= 1 << 5,
            Button::SEL => self.buttons |= 1 << 6,
            Button::STA => self.buttons |= 1 << 7,
        }

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
        match button {
            Button::R => self.buttons &= !(1),
            Button::L => self.buttons &= !(1 << 1),
            Button::U => self.buttons &= !(1 << 2),
            Button::D => self.buttons &= !(1 << 3),
            Button::A => self.buttons &= !(1 << 4),
            Button::B => self.buttons &= !(1 << 5),
            Button::SEL => self.buttons &= !(1 << 6),
            Button::STA => self.buttons &= !(1 << 7),
        }
    }
}
